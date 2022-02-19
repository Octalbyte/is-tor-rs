use std::io::prelude::*;
use std::net::TcpStream;

pub fn find_nodes() {

    let reqstr = "
    GET /torbulkexitlist HTTP/2
    ";
    let mut stream = TcpStream::connect("116.202.120.181:80").unwrap();
    stream.write(reqstr.as_bytes()).unwrap();
    let mut resp: String = String::new();
    stream.read_to_string(&mut resp).unwrap();
    println!("{}", resp);
    //Ok(())




}