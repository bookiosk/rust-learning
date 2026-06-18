use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello from simple Rust server!\n";
    let _ = stream.write_all(response.as_bytes());
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on http://127.0.0.1:7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("connection failed: {}", e),
        }
    }
    Ok(())
}
