use tokio::io::{self};
use tokio::net::{TcpStream};
use std::sync::{Arc, Mutex};

async fn run_client(input_backlog_mutex: Arc<Mutex<Vec<String>>>) {
    let stream = TcpStream::connect("127.0.0.1:80").await.unwrap();
	loop {
		stream.writable().await.unwrap();
		let mut value = input_backlog_mutex.lock().unwrap();
		if value.len() > 0 {
			let data: String = value.pop().unwrap();
			write_value(&stream, data);
		}

	}
}

fn write_value(stream: &TcpStream, value: String) {
	match stream.try_write(value.as_bytes()) {
		Ok(_n) => {},
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

	let input_backlog: Vec<String> = Vec::new();
	let input_backlog_mutex: Arc<Mutex<Vec<String>>>  = Arc::new(Mutex::new(input_backlog));
	let input_backlog_mutex_message_thread = input_backlog_mutex.clone();
	tokio::spawn(async move {run_client(input_backlog_mutex_message_thread).await;});

	loop {
		println!("Get Some input");
		let mut test_data = String::new();

    	match std::io::stdin().read_line(&mut test_data) {
			Ok(_n) => {},
			Err(e) => {
				eprintln!("{:?}",e);
			}
		}

		input_backlog_mutex.lock().unwrap().push(test_data.to_string());
	}
} // the stream is closed here