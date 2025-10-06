use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::common;

const RESP: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 49\r\n\r\n<html><body><h1>CORE CHALLENGE</h1></body></html>";

pub async fn run() -> io::Result<()> {
    let socket = common::build_socket()?;
    let listener = TcpListener::from_std(socket.into())?;
    println!("running async server.");
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move { handle_connection(stream).await });
    }
}

async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 128];
    let mut used = 0usize;
    loop {
        let read = stream.read(&mut buf[used..]).await?;
        if read == 0 {
            return Ok(());
        }
        used += read;
        if common::header_end_found(&buf[..used]) {
            stream.write_all(RESP).await?;
            used = 0;
        }
    }
}
