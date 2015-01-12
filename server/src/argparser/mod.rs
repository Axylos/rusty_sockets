#![crate_name = "argparser"]
#![crate_type = "lib"]
use getopts::{optopt, getopts};
use std::io::net::ip::ToSocketAddr;
use std::io::net::ip::SocketAddr;
use std::os;

pub fn build_address(args: Vec<String>) -> SocketAddr {

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

    let socket_addr = (host.as_slice(), port).to_socket_addr();
    //let socket_addr = ToSocketAddr::to_socket_addr((host.as_slice(), port);
    return socket_addr.unwrap();
}

