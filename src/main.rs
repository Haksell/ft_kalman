use std::{net::UdpSocket, str};

const SERVER_ADDR: &str = "127.0.0.1:4242";
const LOCAL_ADDR: &str = "127.0.0.1:5555"; // Client binds to a local port

#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn from_lines(lines: &[&str]) -> Option<Self> {
        if lines.len() < 3 {
            return None;
        }
        let x = lines[0].parse().ok()?;
        let y = lines[1].parse().ok()?;
        let z = lines[2].parse().ok()?;
        Some(Vec3 { x, y, z })
    }
}

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

    // Reading true position
    let pos_msg = read_message(&socket, &mut buf);
    println!("D {}", pos_msg);
    let pos_lines: Vec<&str> = pos_msg.lines().skip(1).collect();
    let position = Vec3::from_lines(&pos_lines).unwrap_or(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // Reading speed
    let speed_msg = read_message(&socket, &mut buf);
    println!("E {}", speed_msg);
    let speed: f64 = speed_msg
        .lines()
        .skip(1)
        .next()
        .unwrap_or("0.0")
        .parse()
        .unwrap_or(0.0);

    // Reading acceleration
    let acc_msg = read_message(&socket, &mut buf);
    println!("F {}", acc_msg);
    let acc_lines: Vec<&str> = acc_msg.lines().skip(1).collect();
    let acceleration = Vec3::from_lines(&acc_lines).unwrap_or(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // Reading direction
    let dir_msg = read_message(&socket, &mut buf);
    println!("G {}", dir_msg);
    let dir_lines: Vec<&str> = dir_msg.lines().skip(1).collect();
    let direction = Vec3::from_lines(&dir_lines).unwrap_or(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    // End message
    println!("H {}", read_message(&socket, &mut buf));

    println!("\nParsed Data:");
    println!("Position: {:?}", position);
    println!("Speed: {}", speed);
    println!("Acceleration: {:?}", acceleration);
    println!("Direction: {:?}", direction);

    // Responding with fixed position (0, 0, 0)
    socket.send(format!("{} {} {}", position.x, position.y, position.z).as_bytes())?;

    println!("I {}", read_message(&socket, &mut buf));
    println!("J {}", read_message(&socket, &mut buf));
    println!("K {}", read_message(&socket, &mut buf));
    println!("L {}", read_message(&socket, &mut buf));
    println!("M {}", read_message(&socket, &mut buf));
    println!("N {}", read_message(&socket, &mut buf));
    println!("O {}", read_message(&socket, &mut buf));

    Ok(())
}
