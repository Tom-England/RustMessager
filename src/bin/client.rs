use std::io::prelude::*;
use std::net::TcpStream;

pub fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;

    stream.write(&[1; 128])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here