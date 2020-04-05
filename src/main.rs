use std::net::{TcpListener};
use std::process::exit;

fn main() {
	let host = "127.0.0.1:8080";

	let listener = TcpListener::bind(host);
	match listener {
		Ok(_) => println!("Listening on {}", host),
		Err(e) => {
			println!("Error binding to {}: {}", host, e);
			exit(0x1);
		},
	}
	match listener.unwrap().accept() {
		Ok((_socket, addr)) => println!("New client connected from {}", addr),
		Err(e) => {
			println!("accept() error: {}", e);
			exit(0x2);
		}
	}
}
/*
fn handle_connection(stream : TcpStream) {
	stream.read();
}
*/