use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpStream};
use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:80").await.unwrap();

    stream.write_all(b"Hello, World!").await?;

    Ok(())
} // the stream is closed here