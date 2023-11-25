use crate::{
    message::{Request, Response},
    utils::{next_addr, set_priority, socket_addr_range},
};
use anyhow::Result;
use deku::prelude::*;
use itertools::{izip, Itertools};
use std::{
    io::prelude::*,
    net::{SocketAddr, TcpListener},
    thread,
    time::Duration,
};

pub fn run(base_addr: SocketAddr) -> Result<()> {
    // Listen and accept an TCP connection
    let listener = TcpListener::bind(base_addr)?;

    loop {
        let (mut stream, client_addr) = listener.accept()?;
        println!("Start a benchmark from {client_addr}");

        // Wait for a request
        let request = {
            let mut len_buf = [0u8; 4];
            stream.read_exact(&mut len_buf)?;
            let len = u32::from_le_bytes(len_buf) as usize;

            let mut request_buf = vec![0u8; len];
            stream.read_exact(&mut request_buf)?;
            let (_, request) = Request::from_bytes((&request_buf, 0))?;
            request
        };
        let Request {
            payload_size,
            period_millis,
            count,
            priorities,
            ..
        } = request;
        let period = Duration::from_millis(period_millis);

        // Open listening sockets
        let addrs = socket_addr_range(next_addr(base_addr)?, priorities.len() as u16)?;
        let listeners: Vec<_> = addrs.into_iter().map(TcpListener::bind).try_collect()?;

        // Send a response
        stream.write_all(&Response::Ready.to_bytes()?)?;

        let handles: Vec<_> = izip!(listeners, priorities)
            .map(|(listener, priority)| -> Result<_> {
                let handle = thread::spawn(move || {
                    run_pong(listener, priority, payload_size, count, period)
                });
                Ok(handle)
            })
            .try_collect()?;

        // Join threads
        for handle in handles {
            handle.join().unwrap()?;
        }
    }
}

fn run_pong(
    listener: TcpListener,
    priority: i32,
    payload_size: usize,
    count: usize,
    _period: Duration,
) -> Result<()> {
    let (mut stream, _) = listener.accept()?;
    set_priority(&stream, priority)?;
    let peer_addr = stream.peer_addr()?;

    println!("Accepted {peer_addr:?} for priority {priority}");

    // Create a counter iterator 1, 2, ... up to `count`.
    let counter = create_counter(count);
    let mut reply_buf = vec![0u8; payload_size];

    for nth in counter {
        // Receive the ping
        stream.read_exact(&mut reply_buf)?;

        // Reply the pong back
        stream.write_all(&reply_buf)?;
        println!("{payload_size} bytes from {peer_addr}: seq={nth}");
    }

    Ok(())
}

pub fn create_counter(count: usize) -> Box<dyn Iterator<Item = usize>> {
    if count > 0 {
        Box::new((1..).take(count))
    } else {
        Box::new(1..)
    }
}
