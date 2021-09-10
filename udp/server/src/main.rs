use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9876").expect("Could not bind to address");
    println!("Server is running.");

    loop {
        let mut receive_buffer: [u8; 1024] = [0; 1024];
        let (number_of_bytes, source_address) = socket.recv_from(&mut receive_buffer).expect("Did not receive data.");
        let data  = String::from_utf8(receive_buffer[..number_of_bytes].to_vec()).expect("Could not read data.");
        println!("RECEIVED: {}", data);
        let capitalised_sentence = data.to_uppercase();
        let send_buffer = capitalised_sentence.into_bytes();
        socket.send_to(&send_buffer, source_address).expect("Could not send data.");
    }
}
