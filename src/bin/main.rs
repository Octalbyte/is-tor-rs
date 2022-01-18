use std::env;
use istor::istor;
use colored::*;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.1.0";

fn main() {
    println!("isTor CLI v{} made by {}", VERSION.bold(), AUTHOR.bold());
    let args: Vec<String> = env::args().collect();
    println!("{}", istor::istor(args[1].as_str()));
}
