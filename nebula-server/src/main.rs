extern crate libc;
#[macro_use]
extern crate crossbeam_channel;

use libc::{c_int, size_t};
use std::time::{Instant};
use std::{ptr, thread};
use std::net::TcpListener;
use tungstenite::server::accept;
use crossbeam_channel::bounded;
use std::sync::Arc;

mod encoder;
mod grabber;

fn main() {
    
    println!("Launching Nebula Server");

    let (frame_sender, fr) = bounded(10);  // 10 frame capacity

    let frame_receiver = Arc::new(fr);  // Create a referenced count

    let grabber = move || {
        
        if !grabber::create_manager(1920, 1080) {
            println!("Failed to create manager");
            return;
        }
    
        if !grabber::is_supported() {
            println!("Not supported");
            return;
        }

        match grabber::get_output_bits() {
            None => println!("Failed to get frame"),
            Some(frame) => {
                println!("We have a frame nv12={0} width={1} height={2} size={3}",
                        frame.nv12,
                        frame.width,
                        frame.height,
                        frame.data.len());

                frame_sender.send(frame).unwrap();
            }
        }    
    }; 

    // Start thread to grab screen frames
    thread::spawn(grabber);
    
    if !encoder::initialise() {
        println!("Failed to initliase encoder");
    }

    // Create server and block on connections which are spawned into own thread
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    for stream in server.incoming() {
        
        let frame_receiver = Arc::clone(&frame_receiver);
        
        thread::spawn (move || {
            
            let mut websocket = accept(stream.unwrap()).unwrap();
            
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() {
                    // If cmd 'f'
                    // Grab latest frame from channel
                    // Encode and send back in this thread
                    let frames: Vec<grabber::Frame> = frame_receiver.iter().collect();
                    let frame = frames.last();

                }
            }
        });
    }
}
