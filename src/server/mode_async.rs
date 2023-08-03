use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::utils::{get_addr, print_connection_established};
use crate::Cli;

const HOST: &str = "127.0.0.1";

pub async fn start(cli: &Cli) {
    let addr = get_addr(HOST, cli.port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server is now listening asynchronously on {addr}");

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    print_connection_established(
        stream.local_addr().unwrap().ip(),
        stream.local_addr().unwrap().port(),
    );

    // read the buffer
    let mut buffer = [0; 1024];
    let len_read = stream.read(&mut buffer[..]).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len_read])
        .trim()
        .to_string();
    println!("Received: {message}");

    // call another server
    // TODO

    let output = message;

    // write the message
    stream.write_all(output.as_bytes()).await.unwrap();
    println!("Sent: {output}");
}
