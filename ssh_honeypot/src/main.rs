use std::net::{IpAddr,TcpListener};
use std::io::{Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:22").unwrap();
    println!("Listening for incoming SSH connections...");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer = [0;1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let ip_addr = stream.peer_addr().unwrap().ip();
        println!("SSH connection from {:?} ",ip_addr);
        println!("Received Data: {:?}",&buffer[..bytes_read]);
    }
}