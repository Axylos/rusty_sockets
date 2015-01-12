extern crate server;
use server::build_socket_addr::build_address as b_addr;
use std::io::{BufferedReader, BufferedWriter, Acceptor, Listener, TcpListener};
use std::thread::Thread;
use std::os;
use server::boot_server::handle_stream as handle_stream;
use server::boot_server::{take_receiver, take_sender};
use std::sync::mpsc::{channel, Sender, Receiver};
use server::user_socket_manager::Message;

fn main() {
    println!("called");

    let args  = os::args();
    let socket_address = b_addr(args);


    let listener = match TcpListener::bind(socket_address) {
        Ok(m) => { m }
        Err(f) => { panic!("port probably in use: {}", f) }
    };
    let mut counter = 0;


    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        let read_stream = stream.ok().unwrap();
        let write_stream = read_stream.clone();
        let (tx, rx) = channel();
        counter += 1;

        Thread::spawn(move|| {
            server::boot_server::take_sender(tx.clone());
        });

        Thread::spawn(move || {
            server::boot_server::take_receiver(write_stream, rx);
        });

    }

    drop(acceptor);
}
