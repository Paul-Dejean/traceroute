# ctraceroute

`ctraceroute` is a Rust-based traceroute tool that mimics the classic Unix traceroute command. It sends UDP packets with increasing TTL values and listens for ICMP responses to map the network path to a target host.

## Features

- **Traceroute:** Send UDP packets with incrementing TTL to trace the path.
- **Custom Options:** Configure packet size, max hops, and timeout.
- **Hostname Resolution:** Automatically resolves target hostnames to IPv4 addresses.
- **ICMP Listening:** Displays hop number, hostname, IP, and round-trip time.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable recommended)
- Elevated privileges for raw socket operations (e.g., run with `sudo` on Unix).

### Build

Clone the repository and build in release mode:

```bash
git clone https://github.com/yourusername/ctraceroute.git
cd ctraceroute
cargo build --release
```

### Usage

Run the command

```bash
sudo ./target/release/ctraceroute dns.google.com
```
