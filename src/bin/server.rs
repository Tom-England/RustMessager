use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>>{
    // ..
	loop {
		stream.readable().await?;
		let mut buffer: [u8; 128] = [0; 128]; 
		let read_bytes = stream.try_read(&mut buffer[..]);
		println!("Received: {:?}", &buffer[..read_bytes.unwrap()]);
	}
	//println!("Got connection");
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80").await.unwrap();

	loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
    Ok(())
}