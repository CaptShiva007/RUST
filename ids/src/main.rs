use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

struct IntrusionDetectionSystem {
    listen_address: String,
    listen_port: u16,
    whitelist: Vec<String>,
    blacklist: Vec<String>,
}

impl IntrusionDetectionSystem {
    fn new(listen_address: &str, listen_port: u16, whitelist: Vec<String>, blacklist: Vec<String>) -> Self {
        IntrusionDetectionSystem {
            listen_address: listen_address.to_string(),
            listen_port: listen_port,
            whitelist: whitelist,
            blacklist: blacklist,
        }
    }

    fn start(&self) {
        let listener = TcpListener::bind((self.listen_address.as_str(), self.listen_port)).unwrap();
        println!("Listening on {}:{}", self.listen_address, self.listen_port);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let remote_addr = stream.peer_addr().unwrap();
            let remote_ip = remote_addr.ip().to_string();

            println!("New connection from {}", remote_ip);

            if self.is_blacklisted(&remote_ip) {
                println!("{} is blacklisted. Dropping connection.", remote_ip);
                continue;
            }

            if !self.is_whitelisted(&remote_ip) {
                println!("{} is not whitelisted. Dropping connection.", remote_ip);
                continue;
            }

            let handle = thread::spawn(move || {
                IntrusionDetectionSystem::handle_connection(stream, remote_ip);
            });
            handle.join();
        }
    }

    fn is_whitelisted(&self, ip: &str) -> bool {
        self.whitelist.contains(&ip.to_string())
    }

    fn is_blacklisted(&self, ip: &str) -> bool {
        self.blacklist.contains(&ip.to_string())
    }

    fn handle_connection(mut stream: TcpStream, remote_ip: String) {
        let mut buffer = [0; 1024];
        let mut request = String::new();

        stream.read(&mut buffer).unwrap();
        request = String::from_utf8_lossy(&buffer).to_string();

        if request.contains("malicious") {
            println!("Malicious request from {}", remote_ip);
            // TODO: Perform further analysis and take action
        } else {
            println!("Normal request from {}", remote_ip);
        }

        stream.write(b"HTTP/1.1 200 OK\r\n\r\nHello, World!").unwrap();
        stream.flush().unwrap();
    }
}

fn main() {
    let ids = IntrusionDetectionSystem::new("127.0.0.1", 8080, vec![
        "127.0.0.1".to_string(),
    ], vec![
        "192.168.0.1".to_string(),
        "10.0.0.1".to_string(),
    ]);
    ids.start();
}