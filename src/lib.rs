use clap::Parser;
use std::{
    io,
    net::{IpAddr, ToSocketAddrs},
    time::Duration,
};

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType::Layer4};
use std::net::UdpSocket;

#[derive(Parser, Debug)]
#[command(name = "ctraceroute")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool traceroute", long_about = None)]
pub struct Args {
    hostname: String,

    #[arg(default_value_t = 40, value_parser = clap::value_parser!(u32).range(28..))]
    packets_size: u32,

    #[arg(short = 'm', default_value_t = 64)]
    max_hops: u32,

    #[arg(short = 'w', default_value_t = 5)]
    timeout: u32,
}
pub fn execute_command(args: &mut Args) -> i32 {
    let ip = match resolve_ip_from_hostname(&args.hostname) {
        Ok(ip) => ip,
        Err(_) => return 1,
    };
    println!(
        "Traceroute to {} ({}), {} hops max, {} byte packets",
        args.hostname, ip, args.max_hops, args.packets_size,
    );

    let error_code = match send_packets(ip, args.max_hops, args.timeout) {
        Ok(_) => 0,
        Err(_) => 1,
    };
    return error_code;
}

fn create_packet_payload(size: u32) -> Vec<u8> {
    let mut payload = Vec::with_capacity(size as usize);
    for _ in 0..size {
        payload.push(1 as u8);
    }
    payload
}

fn send_packets(destination_ip: IpAddr, packet_size: u32, timeout: u32) -> io::Result<()> {
    const PORT: u16 = 33434;
    let destination = format!("{destination_ip}:{PORT}");

    let socket = UdpSocket::bind("0.0.0.0:33434")?;
    socket.set_write_timeout(Some(Duration::from_secs(3)))?;
    let payload = create_packet_payload(packet_size);

    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (_tx, mut rx) = transport_channel(4096, protocol)
        .expect("Failed to create transport channel. Are you running with elevated privileges?");

    let mut iter = icmp_packet_iter(&mut rx);

    let ttl = 1;
    socket.set_ttl(ttl)?;
    socket.send_to(&payload, &destination)?;

    match iter.next_with_timeout(Duration::from_secs(timeout as u64)) {
        Ok(Some((_packet, addr))) => {
            println!("{ttl} {addr}",);
            if destination_ip == addr {
                return Ok(());
            }
        }
        Ok(None) => {
            println!("{ttl} *");
        }
        Err(e) => {
            eprintln!("Error receiving ICMP packet: {}", e);
        }
    }

    Ok(())
}

fn resolve_ip_from_hostname(hostname: &str) -> io::Result<IpAddr> {
    let socket_address = format!("{}:80", hostname);
    let ip = match socket_address.to_socket_addrs() {
        Ok(addrs) => match addrs.filter(|addr| addr.is_ipv4()).next() {
            Some(addr) => addr.ip(),
            None => {
                eprintln!("Could not resolve hostname: {}", hostname);
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("No IPv4 address found for hostname: {}", hostname),
                ));
            }
        },
        Err(e) => {
            eprintln!("Could not resolve hostname: {}", e);
            return Err(e);
        }
    };
    Ok(ip)
}
