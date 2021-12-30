use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

pub fn connect(send_message_rx: Receiver<String>) {

    // TODO: MUCH BETTER ERROR HANDLING (stream not open, closed stream, etc.)
    // TODO: implement actually decent logging

    let mut stream = TcpStream::connect("127.0.0.1:42069").unwrap();

    let header = "PAC MESSAGE ->";

    loop {
        let packet = format!("{}{}", header, send_message_rx.recv().unwrap());

        stream.write(packet.as_bytes()).unwrap();
        stream.flush().unwrap();

        // TODO: fix response issues

        // let mut response = String::new();
        // stream.read_to_string(&mut response).unwrap();

        let before_log = if let Ok(s) = std::fs::read_to_string("pac-log.log") {
            s
        } else {
            String::new()
        };
        let log_contents = format!("{}\n{}", before_log, packet);
        std::fs::write("pac-log.log", log_contents).unwrap();
    }
}
