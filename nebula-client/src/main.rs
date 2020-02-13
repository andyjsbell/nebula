use tungstenite::{connect, Message};
use url::Url;

fn main() {

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:9001/socket").unwrap()).expect("Can't connect");
    
    loop {
        println!("request frame");
        socket
            .write_message(Message::Binary(vec!['f' as u8]))
            .unwrap();
        
        let msg = socket.read_message().expect("Error reading message");
        println!("frame received {:?}", msg)
    }
}
