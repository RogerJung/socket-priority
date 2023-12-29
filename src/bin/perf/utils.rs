use anyhow::bail;
use anyhow::Result;
use nix::sys::socket::{getsockopt, setsockopt, sockopt::Priority};
use std::{io, net::SocketAddr, os::fd::AsRawFd};

pub fn set_priority<S>(socket: &S, priority: i32) -> io::Result<()>
where
    S: AsRawFd,
{
    // Set priority on the socket
    let fd = socket.as_raw_fd();
    setsockopt(fd, Priority, &priority)?;
    let actual_priority = getsockopt(fd, Priority)?;
    assert_eq!(priority, actual_priority);
    Ok(())
}

pub fn next_addr(addr: SocketAddr) -> Result<SocketAddr> {
    let ip = addr.ip();
    let port = addr.port();
    let Some(next_port) = port.checked_add(1) else {
        bail!("Unable to find the next port of {port}");
    };
    Ok(SocketAddr::new(ip, next_port))
}

pub fn socket_addr_range(base_addr: SocketAddr, count: u16) -> Result<Vec<SocketAddr>> {
    let ip = base_addr.ip();
    let start_port = base_addr.port();

    let Some(end_port) = start_port.checked_add(count) else {
        bail!("Unable to get {count} port since port {start_port}");
    };

    let addrs: Vec<_> = (start_port..end_port)
        .map(|port| SocketAddr::new(ip, port))
        .collect();

    Ok(addrs)
}
