#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
// #[macro_use]
// extern crate crossbeam_channel;
//use libc::{c_int, size_t};
//use std::time::{Instant};
// use std::net::TcpListener;
// use tungstenite::server::accept;
// use crossbeam_channel::bounded;
// use std::sync::Arc;
// use std::thread;
// https://rust-lang.github.io/rust-bindgen/tutorial-4.html

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern {
    fn CreateManager(outWidth: u32, outHeight: u32) -> bool;
    fn IsSupported() -> bool;
    fn GetOutputRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    // fn GetOriginalRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOutputBits(buffer: &*mut u8, outLen: &u32, nv12: &bool) -> bool;
}

// #[link(name = "avcodec-58", kind = "dynamic")]
// extern {
//     fn avcodec_find_encoder(codec: u32) -> 
// }

struct Frame {
    data: Vec<u8>,
    nv12: bool,
    width: u32,
    height: u32,
}

fn get_output_bits() -> Option<Frame> {
    let output_rect = get_output_rect();

    let dstlen = output_rect.2 * output_rect.3 * 4;

    let mut dst:Vec<u8> = Vec::with_capacity(dstlen as usize);
    let pdst = dst.as_mut_ptr();
    let nv12 = true;
    
    unsafe {
        
        if GetOutputBits(&pdst, &dstlen, &nv12) {
            dst.set_len(dstlen as usize);
            return 
                Some(Frame {
                    data: dst,
                    nv12: nv12,
                    width: output_rect.2,
                    height: output_rect.3,
                });
        }
    }

    None
}

fn create_manager(out_width: u32, out_height: u32) -> bool {
    unsafe {
        return CreateManager(out_width, out_height);
    }
}

fn is_supported() -> bool {
    unsafe {
        IsSupported()
    }
}

#[derive(Debug)]
struct Rect(u32, u32, u32, u32);

fn get_output_rect() -> Rect {
    unsafe {
        let x: u32 = 0;
        let y: u32 = 0;
        let width: u32 = 0;
        let height: u32 = 0;

        GetOutputRect(&x, &y, &width, &height);

        Rect(x, y, width, height)
    }
}

// fn get_original_rect() -> Rect {
//     unsafe {
//         let x: u32 = 0;
//         let y: u32 = 0;
//         let width: u32 = 0;
//         let height: u32 = 0;

//         GetOriginalRect(&x, &y, &width, &height);

//         Rect(x, y, width, height)
//     }
// }

fn main() {
    
    println!("Launching Nebula Server");

    if !create_manager(1920, 1080) {
        println!("Failed to create manager");
        return;
    }

    if !is_supported() {
        println!("Not supported");
        return;
    }

    // unsafe {
    //     let codec = avcodec_find_encoder(AVCodecID_AV_CODEC_ID_H264);
    //     // let config = avcodec_configuration();
    //     // println!("{:?}", config);
    // }
    
    // let (frame_sender, fr) = bounded(10);  // 10 frame capacity

    // let frame_receiver = Arc::new(fr);  // Create a referenced count

    // let grabber = move || {
    //     match get_output_bits() {
    //         None => println!("Failed to get frame"),
    //         Some(frame) => {
    //             println!("We have a frame nv12={0} width={1} height={2} size={3}",
    //                     frame.nv12,
    //                     frame.width,
    //                     frame.height,
    //                     frame.data.len());

    //             frame_sender.send(frame).unwrap();
    //         }
    //     }    
    // }; 

    // // Start thread to grab screen frames
    // thread::spawn(grabber);
    
    // // Create server and block on connections which are spawned into own thread
    // let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    
    // for stream in server.incoming() {
        
    //     let frame_receiver = Arc::clone(&frame_receiver);
        
    //     thread::spawn (move || {
            
    //         let mut websocket = accept(stream.unwrap()).unwrap();
            
    //         loop {
    //             let msg = websocket.read_message().unwrap();

    //             if msg.is_binary() {
    //                 // If cmd 'f'
    //                 // Grab latest frame from channel
    //                 // Encode and send back in this thread
    //                 let frames: Vec<Frame> = frame_receiver.iter().collect();
    //                 let frame = frames.last();

    //             }
    //         }
    //     });
    // }
}
