use std::io::stdin;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::utils::get_addr;
use crate::Cli;

const SERVER_HOST_ADDR: &str = "localhost:1234";

pub async fn connect(cli: &Cli) {
    let host = "127.0.0.1"; // TODO: get host from cli
    let addr = get_addr(host, cli.port);
    println!("Connecting to {addr}");
    let mut stream = TcpStream::connect(SERVER_HOST_ADDR)
        .await
        .expect("Should allow connection");

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
            .await
            .expect("Should be able to send a message");
        print!("Sent: {message}");

        // read the result
        let mut buffer = [0; 1024];
        let len_read = stream.read(&mut buffer[..]).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);

        println!("Received: {message}, size: {len_read}");
    }
}
