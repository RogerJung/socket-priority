use anyhow::Result;
use clap::Parser;
use nix::sys::socket::sockopt::Priority;
use nix::sys::socket::{getsockopt, setsockopt};
//use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::os::unix::io::AsRawFd;
use std::time::{Duration, Instant};
use std::io::{Read, Write};
use socket2::{Socket, Domain, Type};
use net2::TcpBuilder;
use std::process;

#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(short = 'c', long, default_value = "127.0.0.1:55555")]
    pub connect_addr: SocketAddr,

    #[clap(short = 'b', long, default_value = "0.0.0.0:55555")]
    pub bind_addr: SocketAddr, // Specify the local address and port to bind

    #[clap(short = 'p', long)]
    pub priority: i32,

    #[clap(short = 's', long)]
    pub payload_size: usize,

    #[clap(short = 'T', long, default_value ="200")]
    pub period: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    //Bind port

    // Connect to server
    //let mut stream = TcpStream::connect(opts.connect_addr)?;
    let tcp = TcpBuilder::new_v4().unwrap();
    tcp.reuse_address(true)?;
    let bound_tcp = tcp.bind(&opts.bind_addr)?;
    let mut stream = bound_tcp.connect(&opts.connect_addr)?;
    // Bind to the specified local address and port
    //stream.bind(&opts.bind_addr.into())?;
    

    // Set priority
    let fd = stream.as_raw_fd();
    setsockopt(fd, Priority, &opts.priority)?;
    let actual_priority = getsockopt(fd, Priority)?;
    assert_eq!(opts.priority, actual_priority);

    // Write loop
    //let buf = vec![0u8; opts.payload_size];

    let mut since = Instant::now();
    let mut acc = 0;
    let period: u64 = opts.period.parse().unwrap_or(1) as u64;
    let start=Instant::now();
    let mut round_trip_times: Vec<Duration> = Vec::new();
    loop {

        stream.set_nonblocking(true)?;

        let mut reply_buf = [0u8; 256];
        //stream.read(&mut reply_buf);
        if let Ok(bytes_read) = stream.read(&mut reply_buf) {
            //println!("reply_buf len{}", reply_buf.len());
            if bytes_read == 0 {
                //println!("No data read.");
            } else {
                
                let received_time_bytes = &reply_buf[..16];
                //println!("received_time_bytes: {received_time_bytes:?}", received_time_bytes = received_time_bytes);
                let duration_value = u128::from_le_bytes(received_time_bytes.try_into().unwrap());
                //println!("duration_value: {duration_value:?}", duration_value = duration_value as u64);
                let send_time = Duration::from_micros(duration_value as u64);
                let send_time_seconds = send_time.as_secs_f64(); // 轉換為秒

                //println!("Send Time in Seconds: {:.6}", send_time_seconds); // 格式化為小數點後 6 位的浮點數
                //println!("send_time: {send_time:?}", send_time = send_time);
                let receive_time = Instant::now()-start;
                //let round_trip_time=(receive_time-send_time).as_micros();
                let round_trip_time=receive_time-send_time;
                println!("Round trip time: {round_trip_time:?}", round_trip_time = round_trip_time);
                round_trip_times.push(round_trip_time);
            }
        } 


        let elapsed = since.elapsed();
        if elapsed >= Duration::from_micros(period) {
            let send_time=Instant::now() - start;
            let send_time_bytes = send_time.as_micros().to_le_bytes();
            //println!("send_time_bytes: {send_time_bytes:?}", send_time_bytes = &send_time_bytes);
            let mut buf = vec![0u8; opts.payload_size];
            buf.splice(..std::mem::size_of::<u128>(), send_time_bytes.iter().cloned());
            
            //println!("buf: {buf:?}", buf = &buf);
            stream.write_all(&buf)?;
            since = Instant::now();
            acc = 0;
        }
        
        // 在程式運行 3 秒後停止整個程式
        if start.elapsed() >= Duration::from_secs(1) {
            //println!("Program is stopping after 3 seconds.");
            
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
            process::exit(0);
        }
    }
}
