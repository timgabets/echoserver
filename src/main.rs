use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
	let host = "127.0.0.1:8080";

	let listener = TcpListener::bind(host).expect("bind() error");
	println!("Listening on {}", host);

	for stream in listener.incoming() {
		match stream {
			Ok(s) => {
				println!("New client from {}", s.peer_addr().unwrap());
				handle_connection(s);
			},
			Err(e) => {
				println!("{}", e);
				continue;
			},
		}
		println!("Client disconnected");
	}
}

fn handle_connection(mut s : TcpStream) {
	let mut buffer = [0; 2048];
	s.read(&mut buffer).expect("read() error");
	s.write(&buffer).expect("write() error");
}