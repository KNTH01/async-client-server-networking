# async-client-server-networking

This is a program that start a TCP echo server or client in async or sync mode, written in Rust.

## Usage

```sh
$> ./async-client-server-networking -h

# Starts server on port 1234
$> ./async-client-server-networking server async 1234
$> ./async-client-server-networking server sync 12345

# Start client to connect
$> ./async-client-server-networking client async 1234
$> ./async-client-server-networking client sync 1234
```

![image](https://github.com/KNTH01/async-client-server-networking/assets/1632391/bf15aa45-0da4-459d-99d0-7137db990ba3)
