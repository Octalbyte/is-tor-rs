# istor
Identify if an IP is a Tor exit node, for Rust.

## Install

You can use `istor` as a CLI tool or as a crate

## CLI usage

`istor --ip <IP> [options]`

options:

`--connect` -> Connect to the tor online list of nodes instead of using hardcoded values. Requires internet connection
`--quiet` -> Do not display version + author info. Useful if a programm is going to read the output

## API
```rust
istor::istor::get_nodes() -> String
istor::istor::get_nodes_realtime() -> String
istor::istor::istor(ip: String, connect: bool) -> bool
```
```rust
extern crate istor;
use istor::istor;

fn main(){
    println!(istor::get_nodes());
    //=> String with hardcoded list of nodes
    println!(istor::get_nodes_real_time());
    //=> Will check the official up-to-date Tor Project list and print the String
    println!(istor::istor("176.10.99.200", false));
    //=> Will check if this IP is in the hardcoded list; true
    println!(istor::istor("176.10.99.200", true));
    //=> Will check if the Ip is in the online list; also true
}
```