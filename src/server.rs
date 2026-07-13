use std::io;
use std::net::TcpListener;
use std::io::{Read};


fn handle_client(mut stream: std::net::TcpStream) -> io::Result<()>{
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer){
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(bytes_read) => {
                let recv_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received data: {}", recv_data);
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind to port 6767");
    println!("Server listening on port 6767");

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("Connection established with {}", addr);
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client {}: {}", addr, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
