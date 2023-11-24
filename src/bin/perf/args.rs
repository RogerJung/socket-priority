use clap::Parser;
use std::net::SocketAddr;

#[derive(Debug, Clone, Parser)]
pub enum Args {
    Server(Server),
    Client(Client),
}

#[derive(Debug, Clone, Parser)]
pub struct Client {
    pub address: SocketAddr,
    pub priorities: Vec<i32>,

    #[clap(short = 's', long)]
    pub payload_size: usize,

    #[clap(short = 'p', long)]
    pub period: u64,

    #[clap(short = 'c', long)]
    pub count: usize,

    #[clap(short = 'u', long)]
    pub udp: bool,
}

#[derive(Debug, Clone, Parser)]
pub struct Server {
    pub address: SocketAddr,

    #[clap(short = 'u', long)]
    pub udp: bool,
}
