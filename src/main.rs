use std::{net::UdpSocket, str};

const SERVER_ADDR: &str = "127.0.0.1:4242";
const LOCAL_ADDR: &str = "127.0.0.1:5555"; // Client binds to a local port

fn read_message<'a>(socket: &UdpSocket, buf: &'a mut [u8; 1024]) -> &'a str {
    match socket.recv(buf) {
        Ok(received) => str::from_utf8(&buf[..received]).unwrap_or("<Invalid UTF-8>"),
        Err(e) => panic!("Error receiving data: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(LOCAL_ADDR)?;
    socket.connect(SERVER_ADDR)?;

    println!("Connected to {}", SERVER_ADDR);
    socket.send(b"READY")?;

    let mut buf = [0; 1024];

    println!("A {}", read_message(&socket, &mut buf));
    println!("B {}", read_message(&socket, &mut buf));
    println!("C {}", read_message(&socket, &mut buf));
    println!("D {}", read_message(&socket, &mut buf));
    println!("E {}", read_message(&socket, &mut buf));
    println!("F {}", read_message(&socket, &mut buf));
    println!("G {}", read_message(&socket, &mut buf));
    println!("H {}", read_message(&socket, &mut buf));
    socket.send(b"0 0 0")?;
    println!("I {}", read_message(&socket, &mut buf));
    println!("J {}", read_message(&socket, &mut buf));
    println!("K {}", read_message(&socket, &mut buf));

    Ok(())
}
