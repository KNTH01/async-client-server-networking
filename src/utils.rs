use std::net::SocketAddr;

pub fn get_addr(host: &str, port: u16) -> String {
    format!("{host}:{port}")
}

pub fn print_connection_established(socket_addr: &SocketAddr) {
    println!("Connection established from {socket_addr}");
}
