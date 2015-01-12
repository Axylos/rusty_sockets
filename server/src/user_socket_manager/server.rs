use std::io::net::pipe::UnixListener;
use std::io::{Listener, Acceptor};

pub fn boot_server() {
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
