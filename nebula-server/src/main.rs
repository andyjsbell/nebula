extern crate libc;
extern crate crossbeam_channel;
extern crate timer;
extern crate chrono;

use std::time::{Instant};
use std::{thread};
use std::net::TcpListener;
use tungstenite::server::accept;
use tungstenite::Message;
use crossbeam_channel::bounded;
use std::sync::Arc;
// use std::fs::File;
// use std::io::Write;

mod encoder;
mod grabber;

fn main() {
    
    println!("Launching Nebula Server");

    let (capturer_channel_sender, capturer_channel_receiver) = bounded(0);
    let (_web_channel_sender, web_channel_receiver) = bounded(1); // Block on web requests
    let (encoder_channel_sender, _encoder_channel_receiver) = bounded(1); // One encoded frame at a time
    
    let fps: u64 = 60;

    let web_channel_sender = Arc::new(_web_channel_sender);  // Create a referenced count
    let encoder_channel_receiver = Arc::new(_encoder_channel_receiver);  // Create a referenced count
    // Grabber scope
    let grabber;
    {   
        grabber = move || {
        
            if !grabber::create_manager(1920, 1080) {
                println!("Failed to create manager");
                return;
            }
        
            if !grabber::is_supported() {
                println!("Not supported");
                return;
            }

            loop {

                let now = Instant::now();

                match grabber::get_output_bits() {
                    None => (),
                    Some(frame) => {
                        
                        capturer_channel_sender.send(frame).unwrap();
                    }
                }

                let diff = now.elapsed().as_millis() as u64;
                
                if diff < 1000 / fps {
                    let d = std::time::Duration::from_millis(1000 / fps - diff); 
                    thread::sleep(d);
                }
            }
        }; 
    }

    // Start thread to grab screen frames
    thread::spawn(grabber);
    
    let encoder_thread;
    {
        encoder_thread = move || {
            
            let mut e : encoder::Encoder = encoder::Encoder::new();

            if !e.initialise() {
                println!("Failed to initliase encoder");
            }

            loop {
                let captured_frame = capturer_channel_receiver.recv().unwrap();
                let requested = web_channel_receiver.recv().unwrap();
                if requested == 1 {
                    let encoded_frame = e.encode_frame(captured_frame).unwrap();
                    encoder_channel_sender.send(encoded_frame).unwrap();
                }
            }
        };
    }

    thread::spawn(encoder_thread);
    
    // Simple console readline to encode screen frame
    // let mut line = String::new();
    // let mut file_out = std::fs::File::create("video.bin").unwrap();
    // loop {
    //     let _ = std::io::stdin().read_line(&mut line).unwrap();
    //     web_channel_sender.send(1).unwrap();
    //     let encoded_frame = encoder_channel_receiver.recv().unwrap();
    //     println!("Encoded frame, writing to file");
    //     file_out.write(&encoded_frame.data).unwrap();
    //     file_out.flush().unwrap();
    // }
    
    // Create server and block on connections which are spawned into own thread
    println!("Running websocket server...");
    let server = TcpListener::bind("0.0.0.0:9001").expect("Unable to open on 0.0.0.0:9001");
    
    for stream in server.incoming() {
        
        let web_channel_sender = Arc::clone(&web_channel_sender);
        let encoder_channel_receiver = Arc::clone(&encoder_channel_receiver);
        thread::spawn (move || {
            
            let mut websocket = accept(stream.unwrap()).unwrap();
            
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() {
                    // If cmd 'f'
                    // Grab latest frame from channel

                    let d = msg.into_data();
                    if d[0] == 'f' as u8 { 
                        web_channel_sender.send(1).unwrap();
                        let encoded_frame = encoder_channel_receiver.recv().unwrap();
                        if websocket.can_write() {
                            let message = Message::Binary(encoded_frame.data);
                            websocket.write_message(message).expect("Unable to write frame to socket");
                        }
                    }
                }
            }
        });
    }
}
