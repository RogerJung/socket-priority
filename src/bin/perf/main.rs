mod args;
mod message;
mod tcp_client;
mod tcp_server;
mod udp_client;
mod udp_server;
mod utils;

use anyhow::Result;
use args::{Args, Client, Server};
use clap::Parser;
use std::time::Duration;

fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();

    match args {
        Args::Server(Server { address, udp }) => {
            if udp {
                udp_server::run(address)?;
            } else {
                tcp_server::run(address)?;
            }
        }
        Args::Client(Client {
            address,
            priorities,
            payload_size,
            count,
            period: period_millis,
            udp,
        }) => {
            let period = Duration::from_millis(period_millis);

            if udp {
                udp_client::run(address, &priorities, payload_size, count, period)?;
            } else {
                tcp_client::run(address, &priorities, payload_size, count, period)?;
            }
        }
    }

    Ok(())
}
