use std::{
    io::{stdin, Read, Write},
    net::TcpStream,
};

use crate::{utils::get_addr, Cli};

pub fn connect(cli: &Cli) {
    let host = "127.0.0.1"; // TODO: get host from cli

    // connection
    let addr = get_addr(host, cli.port);
    println!("Connecting to {addr}");

    let mut stream = TcpStream::connect(addr).expect("Should allow connection");

    println!("Connected... {}", stream.local_addr().unwrap(),);

    loop {
        let mut message = String::new();
        stdin()
            .read_line(&mut message)
            .expect("Should be able to read input");

        if message.trim().is_empty() {
            continue;
        }

        stream
            .write_all(message.as_bytes())
            .expect("Should be able to send a message");
        print!("Sent: {message}");

        // read the result
        let mut buffer = [0; 1024];
        let len_read = stream.read(&mut buffer[..]).unwrap();
        let message = String::from_utf8_lossy(&buffer);

        println!("Received: {message}, size: {len_read}");
    }
}
