use clap::Parser;
use ctraceroute::{execute_command, Args};
use std::process;

fn main() {
    let mut args = Args::parse();
    let error_code = execute_command(&mut args);
    process::exit(error_code);
}
