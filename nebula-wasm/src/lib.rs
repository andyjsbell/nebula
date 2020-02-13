extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

//https://rustwasm.github.io/docs/wasm-bindgen/examples/websockets.html
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

pub fn open_connection() {
    let ws = WebSocket::new("ws://localhost:9001/socket");
}