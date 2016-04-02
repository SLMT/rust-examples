use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::{Write};
use std::io::{BufReader, BufRead, BufWriter};

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

fn handle_client(stream: TcpStream, chatRoom: ChatRoom) {
    // Check IP Address
    let addr = stream.peer_addr().unwrap();
    println!("Get a connection from {}", addr);

    // Create a buffer reader and a writer
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    // Send a message
    writer.write(b"Hello Meow!! Please enter a message:\n").unwrap();
    writer.flush().unwrap();

    // Receive a message
    let mut message = String::new();
    reader.read_line(&mut message).unwrap();

    // Echo it
    writer.write(b"Your message: ").unwrap();
    writer.write(message.trim().as_bytes()).unwrap();

    writer.write(b"\nThank you for your using.\n").unwrap();
}
