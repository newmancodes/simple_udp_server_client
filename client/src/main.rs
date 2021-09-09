use std::io::stdin;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind to address.");
    socket.connect("127.0.0.1:9876").expect("Could not connect to server.");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("No data supplied by user.");
    while user_input.ends_with('\n') || user_input.ends_with('\r') {
        user_input.pop();
    }

    socket.send(&user_input.into_bytes()).expect("Could not send data.");
    let mut receive_buffer: [u8; 1024] = [0; 1024];
    let (number_of_bytes, _) = socket.recv_from(&mut receive_buffer).expect("Did not receive data.");
    let data  = String::from_utf8(receive_buffer[..number_of_bytes].to_vec()).expect("Could not read data.");
    println!("FROM SERVER: {}", data);
}
