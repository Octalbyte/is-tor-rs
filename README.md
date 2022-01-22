# istor
Identify if an IP is a Tor exit node, for Rust.

## Install

```bash
cargo install istor #You can install as cli tool
```
```toml
#Or as crate
#In your Cargo.toml
[dependencies]
istor = "0.1.0"
```
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
If you set `connect` to `true`, the api might take longer to respond, as it will have to connect to https://check.torproject.org/torbulkexitlist to get the up-to-date exit node list.
Usually the hardcoded list will be up-to-date, but always check for new releases!

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
## Donate
```toml
# Wallets to which you can donate :)

  BTC = "3NC14JNuzdLkxJTdNa6bnFXaYzMKMc1Uwt"
  ETH = "0xc41c0f847d58121f552c683e454ddafeb415f25e"
  XMR = "875smxvwbP64MFZnHrHwHcGahoEB4a5ARGCBidr95LqL4GEPiB4T8J74UB5TzrXK3wbTZ1iidfYoV37KZq1vqWCQSNztDAF"  
```
