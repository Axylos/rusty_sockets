use std::io::TcpStream;

fn main() {
    let host = "ec2-54-148-208-119.us-west-2.compute.amazonaws.com:8080";
    let port = 8080u16;
    let mut socket = TcpStream::connect(host);
    println!("connecting to server");
    let msg = b"would you like to play a game?\n\n";
    socket.write(msg);
    println!("writing to server");
    let response = socket.read_to_end();
    let parsed_response = String::from_utf8(response.ok().unwrap()).ok().unwrap();
    println!("response: \n{}\n", parsed_response);
    println!("\n done");
}
