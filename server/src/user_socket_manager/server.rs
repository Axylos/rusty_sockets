use std::io::net::pipe::UnixListener;
use std::io::{Listener, Acceptor};
use std::sync::mpsc::{Sender};
use std::io::fs;
use std::io::fs::PathExtensions;
use super::Message;
use std::io::BufferedReader;


pub fn boot_server(mut sender: Sender<Message<'static>>) {
    let server = Path::new("/var/run/rusty/serv_socket.sock");
    if server.exists() {
        fs::unlink(&server).unwrap();
    }
    let bound_serv = UnixListener::bind(&server); 
    let stream = match bound_serv {
        Ok(stream) => {
            println!("bound");

            for mut client in stream.listen().incoming() {
                let mut reader = BufferedReader::new(client.clone());
                let words = reader.read_until(b'\x04');
                let parsed_words = String::from_utf8(words.unwrap()).unwrap();
                let msg: Message<'static> = Message { msg: parsed_words };
                sender.send(msg);
                println!("wrote some stuff");
            }
        },
        Err(e) => println!("it didn't work, {}", e),
    };
    
}
