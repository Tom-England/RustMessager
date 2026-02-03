use tokio::io::{self, AsyncWriteExt};
use tokio::net::{TcpStream};
use std::error::Error;


async fn run_client() {
    let mut stream = TcpStream::connect("127.0.0.1:80").await.unwrap();
	loop {
		stream.writable().await.unwrap();
    	stream.write_all(b"Hello, World!").await.unwrap();
	}
}

fn write_value(stream: &TcpStream) {
	match stream.try_write(b"Hello, World!") {
		Ok(_n) => {
			;
		},
		Err(ref e) if e.kind()  == io::ErrorKind::WouldBlock => {
			println!("Block More n00b");
		},
		Err(e) => {
			eprintln!("{:?}",e);
		}
	}
}

#[tokio::main]
pub async fn main() {

    let mut stream = TcpStream::connect("127.0.0.1:80").await.unwrap();
	loop {
		stream.writable().await.unwrap();
    	write_value(&stream);
	}
} // the stream is closed here