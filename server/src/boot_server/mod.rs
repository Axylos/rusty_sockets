use std::io::{BufferedReader, BufferedWriter, Acceptor, Listener, TcpListener};

use std::io::net::tcp::TcpStream as TcpStream;
use std::thread::Thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use user_socket_manager::Message;
use user_socket_manager::server::boot_server;

pub fn double_write<W: Writer>(mut stream: BufferedWriter<W>, output: &[u8]) {
        stream.write(output);
        stream.write(b"\x04");
}

fn handle_req<'a, R: Reader>(mut stream: BufferedReader<R>) -> String {
    println!("connection received\n reading");

    match stream.read_until(b'\x04') {
        Ok(nread) => {
            let str = nread;
            return String::from_utf8(str).unwrap();
        }
        Err(e) => {
            println!("error reading: {}", e);
            return "failure".to_string();
        }

    }
}

fn double_read<'a, R: Reader>(mut stream: BufferedReader<R>) {
    println!("called double_read\n");
    //let first = stream.read_until(b'\x04');
    //println!("first: {}\n", first );
    let second = stream.read_until(b'\x04');
    println!("second: {}\n", String::from_utf8(second.ok().unwrap()).unwrap() );

    println!("finished double read\n");

}

pub fn handle_stream(mut stream: TcpStream) {
    let input_reader = BufferedReader::new(stream.clone());
    let writer = BufferedWriter::new(stream.clone());

    let input = handle_req(input_reader);
    stream.write(b"stuff23498203948");
    let double_reader = BufferedReader::new(stream.clone());
    double_write(writer, input.as_bytes());
    double_read(double_reader);

    println!("called");
}

pub fn take_receiver(mut stream: TcpStream, mut receiver: Receiver<Message <'static>>) {
        let mut writer = BufferedWriter::new(stream.clone());
        writer.write_u8(b'\x10');
        loop {
            println!("wrote byte");
            let j = receiver.recv().unwrap();
            println!("received message: {}", j.msg);
            writer.write(j.msg.as_bytes());
            writer.write_u8(b'\x04');
        }
}

pub fn take_sender(mut sender: Sender<Message<'static>>) {
    println!("bye");
    let msg: Message = Message { msg: "whoa" } ;
    sender.send(msg);
    boot_server(sender.clone());
}


