use std::io::net::pipe::UnixListener;
use std::io::{Listener, Acceptor};
use std::sync::mpsc::{Sender};
use super::Message;


pub fn boot_server(mut sender: Sender<Message<'static>>) {
    let server = Path::new("/var/run/rusty/serv_socket.sock");
    let bound_serv = UnixListener::bind(&server); 
    let stream = match bound_serv {
        Ok(stream) => {
            println!("bound");

            for mut client in stream.listen().incoming() {
                let msg = "1, 2, 3, 4";
                println!("{}", &msg);
                client.write(msg.as_bytes());
                println!("wrote some stuff");
            }
        },
        Err(e) => println!("it didn't work, {}", e),
    };
    
}
