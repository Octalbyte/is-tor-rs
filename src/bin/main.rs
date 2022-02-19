use clap::Parser;
use istor::istor;
use minparse;

const AUTHOR: &str = "@Octalbyte";
const VERSION: &str = "0.2.0";

struct Args {
    ip: String,
    connect: bool,
    quiet: bool,
}

fn main() {
    let fi = minparse::fields();
    let ip = match fi.get(String::from("ip")) {
        Some(i) => {
            i
        },
        None => {
            println!("Provide an ip");
            std::process::exit(1);
        }
    }
    let sw = minparse::switches();
    let args = Args {
        ip: ip,
        connect: sw.contains(String::from("connect")),
        quiet: sw.contains(String::from("quiet"))           
    }
    if !args.quiet {
        println!("isTor CLI v{} made by {}", VERSION, AUTHOR);
    }
    println!("{}", istor::istor(args.ip.as_str(), args.connect));
}
