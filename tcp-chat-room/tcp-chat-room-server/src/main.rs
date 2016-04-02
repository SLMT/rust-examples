use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::{Write};
use std::io::{BufReader, BufRead, BufWriter};
use std::sync::{Arc, RwLock};

// We must declare module `chat_room` before `use`ing it
mod chat_room;

// Don't forget to declare the module
use chat_room::ChatRoom;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:54321").unwrap();
    let room_lock = Arc::new(RwLock::new(ChatRoom::new()));

    {
        let mut room = room_lock.write().unwrap();
        room.join(String::from("Haha"));
        room.join(String::from("Kerker"));
        room.join(String::from("Papa"));
    }

    for stream in listener.incoming() {
        match stream {
            Ok(to_client_stream) => {
                let rl = room_lock.clone();
                thread::spawn(move || {
                    handle_client(to_client_stream, rl);
                });
            }
            Err(e) => {
                println!("Error Message: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: TcpStream, room_lock: Arc<RwLock<ChatRoom>>) {
    // Check IP Address
    let addr = stream.peer_addr().unwrap();
    println!("Get a connection from {}", addr);

    // Create a buffer reader and a writer
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    print_online_list(&mut writer, &room_lock);

    //
    // // Send a message
    // writer.write(b"Hello Meow!! Please enter a message:\n").unwrap();
    // writer.flush().unwrap();
    //
    // // Receive a message
    // let mut message = String::new();
    // reader.read_line(&mut message).unwrap();
    //
    // // Echo it
    // writer.write(b"Your message: ").unwrap();
    // writer.write(message.trim().as_bytes()).unwrap();
    //
    // writer.write(b"\nThank you for your using.\n").unwrap();
    // {
    //     room_lock.write().unwrap().join(String::from("Haha"));
    // }
}

fn print_online_list(writer: &mut BufWriter<&TcpStream>, room_lock: &Arc<RwLock<ChatRoom>>) {
    let list;

    // Acquire the read lock of the chat room.
    // Then, get the name list from it.
    {
        list = room_lock.read().unwrap().get_name_list();
    } // Release the read lock here

    // Print the online list
    writer.write(b"=== Online List ===\n").unwrap();
    for name in &list {
        writer.write_fmt(format_args!("{}\n", name)).unwrap();
    }
    writer.write(b"===================\n").unwrap();
}
