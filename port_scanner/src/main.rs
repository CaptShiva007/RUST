use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

async fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    match timeout(Duration::from_secs(1), TcpStream::connect(&address)).await {
        Ok(Ok(_)) => true,
        _ => false,
    }
}

#[tokio::main]
async fn main() {
    let ip = "127.0.0.1";
    let ports = 1..1024;

    println!("Scanning {}...", ip);

    for port in ports {
        if scan_port(ip, port).await {
            println!("Port {} is open", port);
        }
    }

    println!("Scan complete.");
}

