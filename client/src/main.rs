use std::io::TcpStream;

fn main() {
    let host = "ec2-54-148-208-119.us-west-2.compute.amazonaws.com:8080";
    let port = 8080u16;
    let mut socket = TcpStream::connect(host);
    socket.write(b"Get / HTTP/1.0\n\n");
    let response = socket.read_to_end();
    //comment
}
