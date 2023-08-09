use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use crate::utils::{get_addr, print_connection_established};
use crate::Cli;

const HOST: &str = "127.0.0.1";

pub fn start(cli: &Cli) {
    let addr = get_addr(HOST, cli.port);
    let listener = TcpListener::bind(addr.clone()).unwrap();
    println!("Server is now listening synchronously on {addr}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, cli.delay);
            }
            Err(_) => {
                println!("Err when handle connection");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    let peer_addr = stream.peer_addr().unwrap();
    print_connection_established(&peer_addr);

    // read the buffer
    let mut buffer = [0; 1024];

    loop {
        let len_read = stream.read(&mut buffer[..]).unwrap();

        if len_read == 0 {
            println!("connection with {} has closed", peer_addr);
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..len_read])
            .trim()
            .to_string();
        println!("Received: {message}");

        // delay the thread
        thread::sleep(Duration::from_millis(delay));

        // write the message
        stream.write_all(message.as_bytes()).unwrap();
        println!("Sent: {message}");
    }
}
