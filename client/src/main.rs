extern crate getopts;
use std::io::TcpStream;
use getopts::{optopt, optflag, getopts, OptGroup, usage};
use std::os;
use std::io;

fn main() {
    let args: Vec<String> = os::args();
    
    let opts = &[
        optopt("s", "", "server", "connect to this server host"),
        optopt("p", "", "port", "connect to this host port")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let host = match matches.opt_present("s") {
        true => matches.opt_str("s").unwrap(),
        false => "ec2-54-148-208-119.us-west-2.compute.amazonaws.com".to_string(),
    };

    println!("{}", host);

    let port = match matches.opt_present("p") {
        true => (matches.opt_str("p")).unwrap().parse().unwrap(),
        false => 80u16,
    };

    let mut read_stream = TcpStream::connect((host.as_slice(), port));
    let mut write_stream = read_stream.clone();
    println!("connecting to server");
    println!("writing to server");
    let msg = b"would you like to play a game?\n\n";


    let first  = io::stdin().read_until(b'\n').ok().unwrap();
    write_stream.write(format!("{}\x04", String::from_utf8(first).ok().unwrap()).as_slice().as_bytes());
    let second  = io::stdin().read_until(b'\n').ok().unwrap();
    write_stream.write(format!("{}\x04", String::from_utf8(second).ok().unwrap()).as_slice().as_bytes());

    let response = read_stream.read_to_end();
    let response = match response {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    let parsed_response = String::from_utf8(response).ok().unwrap();
    println!("response: \n{}\n", parsed_response);
    println!("\n done");
}
