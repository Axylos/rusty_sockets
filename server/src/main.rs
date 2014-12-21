    use std::str::from_utf8;
    use std::io::net::tcp::TcpAcceptor;
    use std::io::{Acceptor, Listener, TcpListener, TcpStream };
    use std::thread::Thread;


fn main() {
    println!("called");

    let mut listener = TcpListener::bind("127.0.0.1:8080").ok().unwrap();
    //let s_name = listener.socket_name();
    //println!("{}", s_name);


    //bind listener to address

    let mut acceptor = listener.listen().unwrap();
    let mut buf = [0u8, ..1024];

    fn handle_client(mut stream: TcpStream) {

        println!("connection received");
    }

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { /* connectoin failed */ }
            Ok(stream) => Thread::spawn(move|| {
                handle_client(stream)
            }).detach()
        }
    }

    drop(acceptor);
}
