use std::env;
use istor::istor;
use colored::*;
use clap::Parser;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ip to check
    #[clap(short, long)]
    ip: String,

    /// Should connect to the online node list?
    #[clap(short, long)]
    connect: bool,
}

fn main() {
    println!("isTor CLI v{} made by {}", VERSION.bold(), AUTHOR.bold());
    let args = Args::parse();
    println!("{}", istor::istor(args.ip.as_str(), args.connect));
}
