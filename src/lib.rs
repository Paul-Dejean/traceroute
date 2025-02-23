use clap::Parser;
use std::{
    io,
    net::{IpAddr, ToSocketAddrs},
};

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
    return 0;
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
