use std::{
    io::{Read as _, Result, Write as _},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

use socket2::{Domain, Socket, Type};

static RESP: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 49\r\n\r\n<html><body><h1>CORE CHALLENGE</h1></body></html>";

fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("invalid socket address");
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.bind(&addr.into())?;
    socket.listen(8192)?;

    for stream in TcpListener::from(socket).incoming() {
        thread::spawn(move || handle_connection(stream?));
    }
    Ok(())
}

#[rustfmt::skip]
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0u8; 128];
    loop {
        if stream.read(&mut buf)? == 0 { return Ok(()); }
        stream.write_all(RESP)?;
    }
}
