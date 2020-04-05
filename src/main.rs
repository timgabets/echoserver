use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
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
	let listener = listener.unwrap();
	match listener.accept() {
		Ok((socket, addr)) => {
			println!("New client connected from {}", addr);
			handle_connection(socket);
		}
		Err(e) => {
			println!("accept() error: {}", e);
			exit(0x2);
		}
	}
}

fn handle_connection(mut s : TcpStream) {
	let mut buffer = [0; 2048];
	match s.read(&mut buffer) {
		Ok(_) => {
			s.write(&buffer).unwrap();
		},
		Err(e) => println!("read() error: {}", e),
	}
}