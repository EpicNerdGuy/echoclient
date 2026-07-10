use std::io;
use std::net::TcpListener;
use std::io::{Read,Write};


fn handle_client(mut stream: std::net::TcpStream){
    // logic to handle the client connection
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind to port 6767");
    println!("Server listening on port 6767");

    loop {
        match listener.accept(){
            Ok((stream, addr)) => {
                println!("Connection established with {}", addr);
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}",e);
            }
        }
    }

}
