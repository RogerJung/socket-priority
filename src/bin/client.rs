use anyhow::Result;
use clap::Parser;
use nix::sys::socket::sockopt::Priority;
use nix::sys::socket::{getsockopt, setsockopt};
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::io::AsRawFd;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(long, default_value = "127.0.0.1:55555")]
    pub connect_addr: SocketAddr,

    #[clap(long)]
    pub priority: i32,

    #[clap(long)]
    pub payload_size: usize,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // Connect to server
    let mut stream = TcpStream::connect(opts.connect_addr)?;

    // Set priority
    let fd = stream.as_raw_fd();
    setsockopt(fd, Priority, &opts.priority)?;
    let actual_priority = getsockopt(fd, Priority)?;
    assert_eq!(opts.priority, actual_priority);

    // Write loop
    let buf = vec![0u8; opts.payload_size];

    let mut since = Instant::now();
    let mut acc = 0;

    loop {
        stream.write_all(&buf)?;
        acc += buf.len();

        let elapsed = since.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let rate = acc as f64 * 8.0 / 1_000_000_000.0 / elapsed.as_secs_f64();
            eprintln!("{rate:.3} Gbits");

            since = Instant::now();
            acc = 0;
        }
    }
}
