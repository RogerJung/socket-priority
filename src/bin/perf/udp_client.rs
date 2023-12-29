use anyhow::Result;
use std::{net::SocketAddr, time::Duration};

pub fn run(
    base_addr: SocketAddr,
    priorities: &[i32],
    payload_size: usize,
    count: usize,
    period: Duration,
) -> Result<()> {
    todo!();
}
