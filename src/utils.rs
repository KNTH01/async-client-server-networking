use std::net::IpAddr;

pub fn get_addr(host: &str, port: u16) -> String {
    format!("{host}:{port}")
}

pub fn print_connection_established(ip: IpAddr, port: u16) {
    println!("Connection established from {}:{}...", ip, port);
}
