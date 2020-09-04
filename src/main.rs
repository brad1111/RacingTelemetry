extern crate byteorder;

use std::net::UdpSocket;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

fn main() -> std::io::Result<()> {
    {
        let mut socket = UdpSocket::bind("127.0.0.1:20777")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = vec![0u8; 68 * std::mem::size_of::<f32>()];
        loop {
            println!("_____________________________________");
            let (recv, peer) = socket.recv_from(&mut buf)?;

            let mut reader = Cursor::new(&buf);
    
            for i in 0..68{
                let value = reader.read_f32::<LittleEndian>().unwrap();
                let output = match i {
                    7 => Some("Speed"),
                    29 => Some("Throttle"),
                    30 => Some("Steering"),
                    31 => Some("Brake"),
                    32 => Some("Clutch"),
                    33 => Some("Gear"),
                    37 => Some("RPM"),
                    _ => None
                };
                if output.is_some() {
                    print!("{}: {},", output.unwrap(), value);
                }
            }
            println!();
            // for i in 0..68{
            //     print!("{:x?} ", buf[i]);
            // }
        }
        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        // let buf = &mut buf[..amt];
        // buf.reverse();
        // socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}