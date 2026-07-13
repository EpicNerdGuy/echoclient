use std::io;
use std::net::TcpStream;
use std::io::{Read,Write};

fn send_message(mut stream: std::net::TcpStream){
    let mut buffer: [u8; 512] = [0;512];
    loop {
        let n = io::stdin().read(&mut buffer).expect("Failed to read input");
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n]).expect("Failed to send message");
    }
}


fn main(){
    let stream = TcpStream::connect("127.0.0.1:6767").expect("Failed to connect to server");
    println!("Connected to server");

    send_message(stream);
}