use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::signal;

async fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>>
{
	loop {
		stream.readable().await?;
		let mut buffer: [u8; 128] = [0; 128]; 
		let read_bytes = stream.try_read(&mut buffer[..]);

		let output: String = String::from_utf8(buffer[..read_bytes.unwrap()].to_vec())?;

		println!("Received: {}",output);
	}
	Ok(())
}

async fn run_server() {
	let listener = TcpListener::bind("127.0.0.1:80").await.unwrap();

	loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    tokio::spawn(async move {
        run_server().await;
    });
	
	match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }
	

    Ok(())
}