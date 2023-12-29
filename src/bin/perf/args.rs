use clap::Parser;
use std::net::SocketAddr;

/// A multi-connection multi-priority benchmark program.
#[derive(Debug, Clone, Parser)]
pub enum Args {
    /// Run server mode.
    Server(Server),
    /// Run client mode.
    Client(Client),
}

#[derive(Debug, Clone, Parser)]
pub struct Client {
    /// The server address to connect to.
    pub address: SocketAddr,
    /// A list of socket priorities to be tested, one connection per
    /// priority.
    pub priorities: Vec<i32>,

    /// Data payload size in bytes per round.
    #[clap(short = 's', long)]
    pub payload_size: usize,

    /// Period per round in milliseconds.
    #[clap(short = 'p', long)]
    pub period: u64,

    /// Maximum number of rounds to be executed. Set 0 for infinite
    /// rounds.
    #[clap(short = 'c', long)]
    pub count: usize,

    /// Use UDP instead of TCP.
    #[clap(short = 'u', long)]
    pub udp: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct Server {
    /// The listening address the server binds to.
    pub address: SocketAddr,

    /// Use UDP instead of TCP.
    #[clap(short = 'u', long)]
    pub udp: bool,
}
