extern crate getopts;
use std::io::{BufferedReader, BufferedWriter, Acceptor, Listener, TcpListener};
use std::thread::Thread;
use getopts::{optopt, getopts};
use std::os;


fn main() {
    println!("called");

    let args: Vec<String> = os::args();

    let opts = &[
        optopt("p", "", "port", "connect to this port")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    let port = match matches.opt_present("p") {
        true => matches.opt_str("p").unwrap().parse().unwrap(),
        false => 80u16,
    };

    let listener = TcpListener::bind(("127.0.0.1", port)).ok().unwrap();


    let mut acceptor = listener.listen();

    fn double_write<W: Writer>(mut stream: BufferedWriter<W>, output: &[u8]) {
        stream.write(b"\nstuff\n");
        stream.write(output);
        stream.write(output);
    }

    fn handle_req<'a, R: Reader>(mut stream: BufferedReader<R>) -> String {
        println!("connection received\n reading");

        match stream.read_line() {
            Ok(nread) => {
                println!("Read {}", nread);
                let str = nread;
                return str;
            }
            Err(e) => {
                println!("error reading: {}", e);
                return b"failure".to_string();
            }

        }
    }


    for stream in acceptor.incoming() {
        match stream {
            Err(e) => { /* connectoin failed */ }
            Ok(stream) => Thread::spawn(move|| {
                let reader = BufferedReader::new(stream.clone());
                let writer = BufferedWriter::new(stream.clone());
                let input = handle_req(reader);
                double_write(writer, input.as_bytes());
            }).detach()
        }
    }

    drop(acceptor);
}
