extern crate wasm_bindgen;
use tungstenite::{connect, Message};
use url::Url;
use std::time::{Instant};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn start() {
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:9001/socket").unwrap()).expect("Can't connect");
    
    let mut now = Instant::now();
    let mut count = 0;
    loop {
        // println!("request frame");
        socket
            .write_message(Message::Binary(vec!['f' as u8]))
            .unwrap();
        
        let _ = socket.read_message().expect("Error reading message");
        // println!("frame received");
        count = count + 1;
        if count == 100 {
            let frame_ms = now.elapsed().as_millis() / 100;
            println!("fps:{}", 1000 / frame_ms);
            now = Instant::now();
            count = 0;
        } 
    }
}