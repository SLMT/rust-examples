use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:54321").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(to_client_stream) => {
                thread::spawn(move || {
                    handle_client(to_client_stream);
                });
            }
            Err(e) => {
                println!("Error Message: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: TcpStream) {
    match stream.peer_addr() {
        Ok(addr) => {
            println!("Get a connection from {}", addr);
        }
        Err(e) => {
            println!("Error Message: {}", e);
        }
    }

}
