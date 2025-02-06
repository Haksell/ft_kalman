use std::{net::UdpSocket, str};

const SERVER_ADDR: &str = "127.0.0.1:4242";
const LOCAL_ADDR: &str = "127.0.0.1:5555"; // Client binds to a local port

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(LOCAL_ADDR)?;
    socket.connect(SERVER_ADDR)?;

    println!("Connected to {}", SERVER_ADDR);
    socket.send(b"READY")?;
    println!("Sent: READY");

    let mut buf = [0; 1024];

    loop {
        match socket.recv(&mut buf) {
            Ok(received) => {
                let received_str = str::from_utf8(&buf[..received]).unwrap_or("<Invalid UTF-8>");
                println!("Received: {}", received_str);

                let response = b"0 0 0";
                socket.send(response)?;
                println!("Sent: 0 0 0");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
