// smallest possible TCP server in Rust

use std::{
    io::{Read, Write},
    net::TcpListener,
};

use base64::{Engine, engine::general_purpose};
use sha1::{Digest, Sha1};

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0u8; 1024];
        stream.read(&mut buffer).unwrap();

        let req = String::from_utf8_lossy(&buffer);
        let key = req
            .lines()
            .find(|line| line.starts_with("Sec-WebSocket-Key:"))
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .trim();

        // println!("{}",&req);

        let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
        let combined = format!("{}{}", key, magic);

        let mut hasher = Sha1::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();

        let final_key = general_purpose::STANDARD.encode(result);

        println!("{}", key);
        println!("{}", final_key);

        println!("{:?}", &buffer[..10]);

        let response = format!(
            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
            final_key
        );
        stream.write(response.as_bytes()).unwrap();


        let mut msg_buffer = [0u8; 1024];
        stream.read(&mut msg_buffer).unwrap();
        println!("{:?}", &msg_buffer[..11]);


    }
}