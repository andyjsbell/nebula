extern crate libc;
#[macro_use]
extern crate crossbeam_channel;
extern crate timer;
extern crate chrono;

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

    let (frame_sender, fr) = bounded(0);
    let fps: u32 = 60;

    // let frame_receiver = Arc::new(fr);  // Create a referenced count

    // Create shared frame for capture and encoding
    // let captured_frame: grabber::Frame = grabber::Frame::new(1920, 1080, true);
    // let mutex = std::sync::Mutex::new(captured_frame);
    // let mutex_arc = std::sync::Arc::new(mutex);
    // //

    // Grabber scope
    let grabber;
    {   
        // let mutex_arc = mutex_arc.clone();
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
                    None => println!("Failed to get frame"),
                    Some(frame) => {
                        println!("Captured frame nv12={0} width={1} height={2} size={3}",
                                frame.nv12,
                                frame.width,
                                frame.height,
                                frame.data.len());

                        // let mut captured_frame = mutex_arc.lock().unwrap();
                        // captured_frame.data = frame.data;
                        // captured_frame.width = frame.width;
                        // captured_frame.height = frame.height;
                        // captured_frame.nv12 = frame.nv12;
                        
                        frame_sender.send(frame).unwrap();
                    }
                }

                let diff = now.elapsed().as_millis() as u32;
                
                if diff < 1000 / fps {
                    let sleep = 1000 / fps - diff;
                    thread::sleep_ms(sleep);
                }
            }
        }; 
    }

    // Start thread to grab screen frames
    thread::spawn(grabber);
    
    let mut encoder : encoder::Encoder = encoder::Encoder::new();

    if !encoder.initialise() {
        println!("Failed to initliase encoder");
    }

    let encoder_thread;
    {
        // let mutex_arc = mutex_arc.clone();
        encoder_thread = move || {
            
            loop {
                // let mut captured_frame = mutex_arc.lock().unwrap();
                let captured_frame = fr.recv().unwrap();
                
                println!("Encode frame nv12={0} width={1} height={2} size={3}",
                        captured_frame.nv12,
                        captured_frame.width,
                        captured_frame.height,
                        captured_frame.data.len());
                
                // thread::sleep_ms(1000 / fps);

                encoder.encode_frame();
            }
        };
    }

    thread::spawn(encoder_thread);

    // Create server and block on connections which are spawned into own thread
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    for stream in server.incoming() {
        
        // let frame_receiver = Arc::clone(&frame_receiver);
        
        thread::spawn (move || {
            
            let mut websocket = accept(stream.unwrap()).unwrap();
            
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() {
                    // If cmd 'f'
                    // Grab latest frame from channel
                    // Encode and send back in this thread
                    // let frames: Vec<grabber::Frame> = frame_receiver.iter().collect();
                    // let frame = frames.last();

                }
            }
        });
    }
}
