// Updated example from http://rosettacode.org/wiki/Hello_world/Web_server#Rust
// to work with Rust 1.0 beta

use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;

#[no_mangle]
pub fn ukl_main() {
    main()
}


fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8 ;4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => (),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

