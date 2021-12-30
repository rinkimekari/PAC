use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

pub fn connect(send_message_rx: Receiver<String>) {
    let mut stream = TcpStream::connect("127.0.0.1:42069").unwrap();

    let header = "PAC MESSAGE ->";

    let packet = format!("{}{}", header, send_message_rx.recv().unwrap());

    stream.write(packet.as_bytes()).unwrap();
    stream.flush().unwrap();

    // let client_buf = "\nTEST PAC NUM 1\n";

    // stream.write(client_buf.as_bytes()).unwrap();
    // stream.flush().unwrap();

    // // let mut buf = [0; 1024];
    // let mut response = String::new();

    // stream.read_to_string(&mut response).unwrap();

    // comm_tx.send(response).unwrap();
}
