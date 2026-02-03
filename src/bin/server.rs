use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use tokio::signal;

async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>>
{
	loop {
		stream.readable().await?;
		let mut buffer: [u8; 128] = [0; 128]; 
		let mut read_bytes = 0;

		match stream.try_read(&mut buffer[..]) {
			Ok(n) => {
				read_bytes = n;
			},
			Err(ref e) if e.kind()  == io::ErrorKind::WouldBlock => {
				println!("Block More n00b");
			},
			Err(e) => {
				eprintln!("{:?}",e);
			}
		}

		let output: String = String::from_utf8(buffer[..read_bytes].to_vec())?;

		println!("Received: {}",output);
	}
}

async fn run_server() {
	let listener = TcpListener::bind("127.0.0.1:80").await.unwrap();

	loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            match handle_client(socket).await {
				Ok(()) => {},
				Err(e) => {
					eprintln!("{:?}", e);
				}
			}
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