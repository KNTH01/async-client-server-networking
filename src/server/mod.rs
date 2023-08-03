pub mod sync {

    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    const HOST: &str = "127.0.0.1";

    pub fn start_sync(port: u16) {
        // // read arguments
        // let delay = std::env::args()
        //     .nth(1)
        //     .unwrap_or("10000".to_owned())
        //     .parse::<u64>()
        //     .unwrap_or(10000);

        // bind
        let listener = TcpListener::bind(HOST.to_string() + ":" + &port.to_string()).unwrap();
        println!("Server is now listening synchronously on {HOST}:{port}");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connection established");
                    handle_connection(stream, 4444);
                }
                Err(_) => {
                    println!("Err when handle connection");
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream, delay: u64) {
        // read the buffer
        let mut buffer = [0; 1024];
        let len_read = stream.read(&mut buffer[..]).unwrap();
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
