use crate::message::Response;
use crate::utils::{next_addr, socket_addr_range};
use crate::{message::Request, utils::set_priority};
use anyhow::{bail, ensure, Result};
use deku::prelude::*;
use itertools::izip;
use itertools::Itertools;
use net2::TcpBuilder;
use std::iter;
use std::{
    io::prelude::*,
    mem,
    net::{SocketAddr, TcpStream},
    thread,
    time::{Duration, Instant},
};

pub fn run(
    base_addr: SocketAddr,
    priorities: &[i32],
    payload_size: usize,
    count: usize,
    period: Duration,
) -> Result<()> {
    ensure!(
        !priorities.is_empty(),
        "Empty list of priorities is not allowed"
    );

    // Create a TCP socket.
    let mut stream = TcpStream::connect(base_addr)?;

    // Send a request
    {
        // Construct the request message.
        let request = Request {
            payload_size,
            period_millis: period.as_millis() as u64,
            count,
            num_priorities: priorities.len(),
            priorities: priorities.to_vec(),
        };

        // Serialize the request and count the size.
        let request_buf = request.to_bytes()?;
        let len = request_buf.len() as u32;

        // Write the length and then write the request.
        stream.write_all(&len.to_le_bytes())?;
        stream.write_all(&request_buf)?;
    }

    // Wait for a response
    {
        // Read response bytes.
        let mut respones_buf = [0u8];
        stream.read_exact(&mut respones_buf)?;

        // Deserialize the bytes.
        let (_, response) = Response::from_bytes((&respones_buf, 0))?;

        match response {
            Response::Ready => {}
            Response::Abort => {
                bail!("Connection rejected by server");
            }
        }
    }

    // Close the connection to the server.
    drop(stream);

    // List ports to be used.
    let addrs: Vec<SocketAddr> = socket_addr_range(next_addr(base_addr)?, priorities.len() as u16)?;

    // Create TCP sockets
    let sockets: Vec<_> = izip!(addrs, priorities)
        .map(|(addr, &priority)| create_socket(addr, priority))
        .try_collect()?;

    // Start writing tasks
    let handles: Vec<_> = izip!(sockets, priorities)
        .map(|(socket, &priority)| -> Result<_> {
            let handle =
                thread::spawn(move || run_ping(socket, priority, payload_size, count, period));
            Ok(handle)
        })
        .try_collect()?;

    // Join threads
    for handle in handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

fn run_ping(
    mut stream: TcpStream,
    priority: i32,
    payload_size: usize,
    count: usize,
    period: Duration,
) -> Result<()> {
    let peer_addr = stream.peer_addr()?;

    // The buffer is used in both writing and reading.
    let mut buf = vec![0u8; payload_size];

    // Recorded RTTs per round.
    let mut round_trip_times: Vec<Duration> = vec![];

    let start = Instant::now();

    // Generate an iterator of ticks (1, time1), (2, time2), ...
    // Each timeX is the starting time of each round.
    let ticks = create_ticks(start, period, count);

    for (nth, when) in ticks {
        // Sleep until the next round starts.
        sleep_until(when);

        let now = Instant::now();

        // Send a ping
        {
            // Record the sending time at the beginning of the buffer.
            let send_time = now - start;
            let send_time_bytes = send_time.as_micros().to_le_bytes();
            buf[0..mem::size_of::<u128>()].copy_from_slice(&send_time_bytes);
            stream.write_all(&buf)?;
        }

        // Receive a pong
        {
            // Read bytes from the server
            stream.read_exact(&mut buf)?;

            // Record the receiving time.
            let receive_time = Instant::now() - start;

            // Extract the sending time from the payload.
            let duration_micros = u128::from_le_bytes(buf[..16].try_into().unwrap());
            let send_time = Duration::from_micros(duration_micros as u64);

            // Record the round-trip time
            let round_trip_time = receive_time - send_time;
            round_trip_times.push(round_trip_time);
            println!(
                "{payload_size} bytes from {peer_addr}: \
                      seq={nth} \
                      priority={priority} \
                      time={round_trip_time:?}"
            );
        }
    }

    // 在程式運行結束後停止整個程式
    for rtt in &round_trip_times {
        println!("round_trip_times: {:?}", rtt);
    }
    if !round_trip_times.is_empty() {
        let total_time: Duration = round_trip_times.iter().sum();
        let avg_time = total_time / round_trip_times.len() as u32;
        let min_time = round_trip_times.iter().min().unwrap();
        let max_time = round_trip_times.iter().max().unwrap();

        println!("Average time: {:?}", avg_time);
        println!("Minimum time: {:?}", min_time);
        println!("Maximum time: {:?}", max_time);
    }

    Ok(())
}

fn create_socket(addr: SocketAddr, priority: i32) -> Result<TcpStream> {
    // Create a TCP socket
    let tcp = TcpBuilder::new_v4().unwrap();
    tcp.reuse_address(true)?;
    let stream = tcp.connect(addr)?;
    set_priority(&stream, priority)?;
    Ok(stream)
}

fn create_ticks(
    start: Instant,
    period: Duration,
    count: usize,
) -> Box<dyn Iterator<Item = (usize, Instant)>> {
    let ticks = iter::successors(Some(start), move |&prev| Some(prev + period));
    let ticks_with_counts = izip!(1.., ticks);

    if count > 0 {
        Box::new(ticks_with_counts.take(count))
    } else {
        Box::new(ticks_with_counts)
    }
}

fn sleep_until(until: Instant) {
    let remain = until.checked_duration_since(Instant::now());
    if let Some(remain) = remain {
        thread::sleep(remain);
    }
}
