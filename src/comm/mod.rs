use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

pub fn connect(comm_tx: Sender<String>) {
    let mut stream = TcpStream::connect("127.0.0.1:42069").unwrap();
    let client_buf = "\nTEST PAC NUM 1\n";

    stream.write(client_buf.as_bytes()).unwrap();
    stream.flush().unwrap();

    // let mut buf = [0; 1024];
    let mut response = String::new();

    stream.read_to_string(&mut response).unwrap();

    comm_tx.send(response).unwrap();
}
