use anyhow::Result;
use clap::Parser;
use nix::sys::socket::sockopt::Priority;
use nix::sys::socket::{getsockopt, setsockopt};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::os::unix::io::AsRawFd;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(short = 'l', long, default_value = "0.0.0.0:55555")]
    pub listen_addr: SocketAddr,

    #[clap(short = 'p', long)]
    pub priority: i32,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // Listen and accept an TCP connection
    let listener = TcpListener::bind(opts.listen_addr).unwrap();

    let (mut stream, _addr) = listener.accept()?;

    // Set priority
    let fd = stream.as_raw_fd();
    setsockopt(fd, Priority, &opts.priority)?;
    let actual_priority = getsockopt(fd, Priority)?;
    assert_eq!(opts.priority, actual_priority);

    // loop
    let mut since = Instant::now();
    let mut acc = 0;

    loop {
        stream.set_nonblocking(true)?;

        let mut reply_buf = [0u8; 256];
        if let Ok(bytes_read) = stream.read(&mut reply_buf) {
            if bytes_read == 0 {
                //println!("No data read.");
            } else {
                stream.write_all(&reply_buf)?;
                println!("reply_buf: {reply_buf:?}", reply_buf = &reply_buf);
            }
        }

        /*


        let mut buf = vec![0; 4096];
        let n_recv = stream.read(&mut buf)?;
        acc += n_recv;
        // Sending a reply immediately after reading the data
        stream.write_all(&buf[..n_recv])?;

        let elapsed = since.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let rate = acc as f64 * 8.0 / 1_000_000_000.0 / elapsed.as_secs_f64();
            println!("{rate:.3} Gbits");

            since = Instant::now();
            acc = 0;
        }*/
    }
}
