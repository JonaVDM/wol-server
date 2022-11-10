use std::error;
use std::fmt;
use std::io;
use std::net::UdpSocket;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum WolErrors {
    InvalidMacError(),
    IoError(io::Error),
}

impl fmt::Display for WolErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMacError() => write!(f, "invalid mac address"),
            Self::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for WolErrors {
    fn from(err: std::io::Error) -> Self {
        WolErrors::IoError(err)
    }
}

impl error::Error for WolErrors {}

pub fn wol(mac: &str, boardcast: &str) -> Result<(), WolErrors> {
    let header: [u8; 6] = [0xff; 6];
    let mac = parse_mac(mac)?;
    let mut packet: Vec<u8> = Vec::new();

    packet.extend(header);

    for _ in 0..16 {
        packet.extend(&mac);
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, format!("{}:9", boardcast))?;

    Ok(())
}

fn parse_mac(mac: &str) -> Result<Vec<u8>, WolErrors> {
    if mac.len() != 17 {
        return Err(WolErrors::InvalidMacError());
    }

    let c: Result<Vec<u8>, ParseIntError> = (0..17)
        .step_by(3)
        .map(|i| u8::from_str_radix(&mac[i..i + 2], 16))
        .collect();

    match c {
        Ok(m) => Ok(m),
        Err(_) => Err(WolErrors::InvalidMacError()),
    }
}
