use std::io::prelude::*;
use std::net::TcpStream;

pub fn find_nodes() {

    let reqstr = "
    GET /torbulkexitlist HTTP/2
    Host: www.bing.com
    User-Agent: curl/7.54.0
    Accept: */*
    ";



    let mut stream = TcpStream::connect("check.torproject.org").unwrap();
    stream.write(reqstr.as_bytes()).unwrap();
    //stream.read(&mut [0; 128])?;
    //Ok(())




}