use std::io::prelude::*;
use std::net::TcpStream;
use std::net::IpAddr;
use dns_lookup::lookup_host;



fn get_ip(host: &str) -> Result<String, &str> {
    let mut s: String = String::new();
    for i in lookup_host(host).unwrap() {
        match i {
            IpAddr::V4(ip4) => s = ip4.to_string(),
            IpAddr::V6(_) => return Err(""),
        }
    }
    Ok(s)
}

fn main() -> std::io::Result<()> {
    println!("Resolving host {}", "google.com");
    let res= get_ip("google.com");  

    let host: String = match res {
        Ok(host)=> host,
        Err(_) => panic!("Problem only IPv6 Address")
    };

    println!("Connecting to address {}:{}", host, 443);
    let mut stream = TcpStream::connect(format!("{}:443", host))?;

    stream.write(&[1])?;
    stream.read(&mut[0; 128])?;
    Ok(())
}
