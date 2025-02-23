use clap::Parser;
use std::process;
use traceroute::{execute_command, Args};

fn main() {
    let mut args = Args::parse();
    let error_code = execute_command(&mut args);
    process::exit(error_code);
}
