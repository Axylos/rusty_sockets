extern crate server;
use server::build_socket_addr::build_address as b_addr;
use std::io::{BufferedReader, BufferedWriter, Acceptor, Listener, TcpListener};
use std::thread::Thread;
use std::os;
use server::boot_server::handle_stream as handle_stream;



fn main() {
    println!("called");

    let args  = os::args();
    let socket_address = b_addr(args);


    let listener = match TcpListener::bind(socket_address) {
        Ok(m) => { m }
        Err(f) => { panic!("port probably in use: {}", f) }
    };


    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        let read_stream = stream.ok().unwrap();
        let write_stream = read_stream.clone();

        Thread::spawn(move|| {
            handle_stream(read_stream);
        });

        Thread::spawn(move || {
            handle_stream(write_stream);
        });

    }

    drop(acceptor);
}
