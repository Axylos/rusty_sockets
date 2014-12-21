use std::io::TcpStream;

fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
    socket.write(b"Get / HTTP/1.0\n\n");
    let response = socket.read_to_end();
}
