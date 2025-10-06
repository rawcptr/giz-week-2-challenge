use std::{
    io::{Read as _, Result, Write as _},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::common;

pub fn run() -> Result<()> {
    let socket = common::build_socket()?;
    println!("running sync server.");
    for stream in TcpListener::from(socket).incoming() {
        thread::spawn(move || handle_connection(stream?));
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 128];
    let mut used = 0usize;
    loop {
        let read = stream.read(&mut buf[used..])?;
        if read == 0 {
            return Ok(());
        }
        used += read;
        if common::header_end_found(&buf[..used]) {
            stream.write_all(common::RESP)?;
            used = 0;
        }
    }
}
