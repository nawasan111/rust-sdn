use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

extern crate byteorder;

fn send_hello(stream: &mut TcpStream) {
    let mut buf: Vec<u8> = Vec::new();
    buf.write_u8(1).unwrap();
    buf.write_u8(0).unwrap();
    buf.write_u16::<BigEndian>(8).unwrap();
    buf.write_u32::<BigEndian>(0).unwrap();
    println!("send: {:?}", buf);
    stream.write_all(&buf).unwrap();
}

fn feture_req(stream: &mut TcpStream, xid: u32) -> Result<(), std::io::Error> {
    let mut buf: Vec<u8> = Vec::new();
    buf.write_u8(1)?; // version
    buf.write_u8(5)?; // message type
    buf.write_u16::<BigEndian>(8)?; // size of packet
    buf.write_u32::<BigEndian>(xid)?; // transaction id
    println!("send: {:?}", buf);
    stream.write_all(&buf)?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(("127.0.0.1", 6633)).unwrap();
    let mut buf = vec![0u8; 8];
    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                // std::thread::spawn(move || {
                println!("=================== connection =======================");

                // after tcp is connected, it will send hello message
                send_hello(&mut stream);

                // loop for receive data
                loop {
                    // first receive with Openflow header 64 bit to buf
                    let res = stream.read(&mut buf);
                    match res {
                        Ok(v) if v > 0 => {
                            let mut packet = Cursor::new(buf.to_vec());
                            println!("buf: {:?}", packet);
                            // split data from header
                            let _version = packet.read_u8().unwrap();
                            let message = packet.read_u8().unwrap();
                            // length is only payload size to receive
                            let length = packet.read_u16::<BigEndian>().unwrap() - 8;
                            let xid = packet.read_u32::<BigEndian>().unwrap();

                            // message_body is var to receive payload if it has
                            // and assign size by length
                            let mut message_body = vec![0u8; length as usize];
                            stream.read(&mut message_body)?;

                            match message {
                                // 0 is Hello message
                                0 => {
                                    // after get Hello, send fetureReq
                                    feture_req(&mut stream, xid)?;
                                }
                                _ => {
                                    println!("others message");
                                }
                            }
                        }
                        Ok(_) | Err(_) => break,
                    }
                }
                println!("======================================================");

                // });
            }
            Err(_) => {
                // connection failed
                panic!("Connection failed")
            }
        }
    }
    Ok(())
}
