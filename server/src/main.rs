extern crate getopts;
use std::io::{BufferedReader, BufferedWriter, Acceptor, Listener, TcpListener};
use std::thread::Thread;
use getopts::{optopt, getopts};
use std::os;


fn main() {
    println!("called");

    let args: Vec<String> = os::args();

    let opts = &[
        optopt("p", "", "port", "connect to this port"),
        optopt("s", "", "server", "connect to this host server")
    ];


    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    let host = match matches.opt_present("s") {
        true => matches.opt_str("s").unwrap(),
        false => "ec2-54-148-208-119.us-west-2.compute.amazonaws.com".to_string(),
    };

    let port = match matches.opt_present("p") {
        true => matches.opt_str("p").unwrap().parse().unwrap(),
        false => 80u16,
    };

    let listener = match TcpListener::bind((host.as_slice(), port)) {
        Ok(m) => { m }
        Err(f) => { panic!("port probably in use: {}", f) }
    };


    let mut acceptor = listener.listen();

    fn double_write<W: Writer>(mut stream: BufferedWriter<W>, output: &[u8]) {
        stream.write(b"\nstuff\n");
        stream.write(output);
        stream.write(output);
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

    fn handle_stream(mut stream: std::io::TcpStream) {
        let input_reader = BufferedReader::new(stream.clone());
        let writer = BufferedWriter::new(stream.clone());

        let input = handle_req(input_reader);
        stream.write(b"stuff23498203948");
        let double_reader = BufferedReader::new(stream.clone());
        double_write(writer, input.as_bytes());
        double_read(double_reader);

        println!("called");
    }



    for stream in acceptor.incoming() {
             Thread::spawn(move|| {
                    handle_stream(stream.ok().unwrap());
                });
    }

    drop(acceptor);
}
