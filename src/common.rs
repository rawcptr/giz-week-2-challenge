use std::{io::Error, net::SocketAddr};

use socket2::{Domain, Socket, Type};

pub static RESP: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 49\r\n\r\n<html><body><h1>CORE CHALLENGE</h1></body></html>";

pub fn header_end_found(buf: &[u8]) -> bool {
    buf.windows(4).any(|w| w == b"\r\n\r\n")
}

pub fn build_socket() -> Result<Socket, Error> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    #[cfg(feature = "async-rt")]
    socket.set_nonblocking(true)?;
    #[cfg(unix)]
    socket.set_reuse_port(true)?;
    socket.bind(&addr.into())?;
    socket.listen(8192)?;
    Ok(socket)
}
