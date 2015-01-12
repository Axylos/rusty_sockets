use std::io::net::pipe::UnixStream;
use std::io::{BufferedReader};

pub fn boot_client() {

    let server = Path::new("/var/run/rusty/serv_socket.sock");
    let mut stream = UnixStream::connect(&server);

    println!("running\n");
    let mut resp = BufferedReader::new(stream.clone());

    match stream {

        Ok(rep) => {
            let mut response = resp.read_to_end();
            let msg = String::from_utf8(response.unwrap());
            println!("here's the response: {}", msg.unwrap());
        },

        Err(e) => println!("{}", e),
    };

}
