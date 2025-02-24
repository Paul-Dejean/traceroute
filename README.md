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

## Installation

To build and run `ctraceroute`, you need to have [Rust](https://www.rust-lang.org/) installed on your system.

1. Clone the repository:

   ```bash
   git clone git@github.com:Paul-Dejean/traceroute.git
   cd traceroute
   ```

2. Build the project using Cargo:

   ```bash
   cargo build --release
   ```

3. (Optional) Install the binary globally:

   ```bash
   cargo install --path .
   ```

### Usage

Run the command

```bash
sudo ./target/release/ctraceroute dns.google.com
```

Or if installed globally:

```bash
sudo ctraceroute dns.google.com
```
