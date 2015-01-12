use std::io::net::pipe::UnixStream;
use std::io::{BufferedReader};
use std::os;
pub struct Message {
    pub msg: String
}

fn main() {

    let args = os::args();
    let msg = args[1].as_bytes();
    let server = Path::new("/var/run/rusty/serv_socket.sock");
    let mut stream = UnixStream::connect(&server);
    stream.write(msg);
    //writer = BufferedWriter::new(stream.clone());
    stream.write(&[b'\x04']);

    println!("running\n");
    let mut resp = BufferedReader::new(stream.clone());

    match stream {

        Ok(rep) => {
            let mut response = resp.read_to_end();
            let parsed_rep = String::from_utf8(response.unwrap()).unwrap();
            let package: Message = Message { msg: parsed_rep };
            //sender.send(package);
            //println!("here's the response: {}", msg.unwrap());
        },

        Err(e) => println!("{}", e),
    };

}
