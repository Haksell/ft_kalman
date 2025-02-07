use std::{net::UdpSocket, str};

const SERVER_ADDR: &str = "127.0.0.1:4242";
const LOCAL_ADDR: &str = "127.0.0.1:5555";

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn read(socket: &UdpSocket, buf: &mut [u8; 1024]) -> Self {
        let msg = read_message(socket, buf);
        println!("{}", &msg[0..14]);
        let lines: Vec<&str> = msg.lines().skip(1).collect();
        Self::from_lines(&lines).unwrap()
    }

    fn from_lines(lines: &[&str]) -> Option<Self> {
        if lines.len() != 3 {
            return None;
        }
        let x = lines[0].parse().ok()?;
        let y = lines[1].parse().ok()?;
        let z = lines[2].parse().ok()?;
        Some(Self { x, y, z })
    }
}

fn ignore_message(socket: &UdpSocket) {
    let mut buf = [0u8; 1];
    let _ = socket.recv(&mut buf);
}

fn read_message<'a>(socket: &UdpSocket, buf: &'a mut [u8; 1024]) -> &'a str {
    match socket.recv(buf) {
        Ok(received) => str::from_utf8(&buf[..received]).unwrap_or("<Invalid UTF-8>"),
        Err(e) => panic!("Error receiving data: {}", e),
    }
}

fn read_measurements(socket: &UdpSocket, buf: &mut [u8; 1024]) -> (Vec3, Vec3) {
    ignore_message(socket);

    let acceleration = Vec3::read(socket, buf);
    let direction = Vec3::read(socket, buf);

    ignore_message(socket);

    (acceleration, direction)
}

fn read_debug_measurements(socket: &UdpSocket, buf: &mut [u8; 1024]) -> (Vec3, f64, Vec3, Vec3) {
    ignore_message(socket);

    let position = Vec3::read(socket, buf);
    let speed: f64 = read_message(socket, buf)
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let acceleration = Vec3::read(socket, buf);
    let direction = Vec3::read(socket, buf);

    ignore_message(socket);

    (position, speed, acceleration, direction)
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(LOCAL_ADDR)?;
    socket.connect(SERVER_ADDR)?;

    println!("Connected to {}", SERVER_ADDR);
    socket.send(b"READY")?;

    let mut buf = [0; 1024];

    read_message(&socket, &mut buf);
    read_message(&socket, &mut buf);

    let (position, speed, acceleration, direction) = read_debug_measurements(&socket, &mut buf);
    println!("\nParsed Data:");
    println!("Position: {:?}", position);
    println!("Speed: {}", speed);
    println!("Acceleration: {:?}", acceleration);
    println!("Direction: {:?}", direction);
    socket.send(format!("{} {} {}", position.x, position.y, position.z).as_bytes())?;

    loop {
        let (acceleration, direction) = read_measurements(&socket, &mut buf);
        println!("\nParsed Data:");
        println!("Acceleration: {:?}", acceleration);
        println!("Direction: {:?}", direction);
        socket.send(format!("{} {} {}", position.x, position.y, position.z).as_bytes())?;
    }
}
