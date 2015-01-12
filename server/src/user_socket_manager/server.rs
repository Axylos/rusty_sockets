use std::io::net::pipe::UnixListener;
use std::io::{Listener, Acceptor};
use std::sync::mpsc::{Sender};
use std::io::fs;
use std::io::fs::PathExtensions;
use super::Message;


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
                let msg: Message = Message { msg: "1, 2, 3, 4" };
                sender.send(msg);
                println!("wrote some stuff");
            }
        },
        Err(e) => println!("it didn't work, {}", e),
    };
    
}
