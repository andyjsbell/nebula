#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate ffmpeg4_ffi;
extern crate num_cpus;

use std::ffi::{CString, CStr};
use std::ptr;

use ffmpeg4_ffi::sys::{
    self,
    AVFrame,
    AVDictionary,
    AVCodec,
    AVCodecContext,
    AVStream,
    AVPacket,
    AVFormatContext,
    AVOutputFormat,
    AVCodecParameters,
    AVCodecParserContext,
    AVMediaType,
    AVMediaType_AVMEDIA_TYPE_UNKNOWN as AVMEDIA_TYPE_UNKNOWN,
    AVMediaType_AVMEDIA_TYPE_VIDEO as AVMEDIA_TYPE_VIDEO,
    AVMediaType_AVMEDIA_TYPE_AUDIO as AVMEDIA_TYPE_AUDIO,
    AVMediaType_AVMEDIA_TYPE_DATA as AVMEDIA_TYPE_DATA,
    AVMediaType_AVMEDIA_TYPE_SUBTITLE as AVMEDIA_TYPE_SUBTITLE,
    AVMediaType_AVMEDIA_TYPE_ATTACHMENT as AVMEDIA_TYPE_ATTACHMENT,
    AVMediaType_AVMEDIA_TYPE_NB as AVMEDIA_TYPE_NB,
    AVFMT_NOFILE,
    AVIO_FLAG_WRITE,
    AVRounding_AV_ROUND_NEAR_INF as AV_ROUND_NEAR_INF,
    AVRounding_AV_ROUND_PASS_MINMAX as AV_ROUND_PASS_MINMAX,
    AVCodecID_AV_CODEC_ID_H264 as AV_CODEC_ID_H264,
    AV_INPUT_BUFFER_PADDING_SIZE,
};

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

fn number_of_threads(width: u32, height: u32, number_of_cores:u8) -> i32 {
    if width * height >= 1920 * 1080 && number_of_cores > 8 {
        return 8;  // 8 threads for 1080p on high perf machines.
    } else if width * height > 1280 * 960 && number_of_cores >= 6 {
        return 3;  // 3 threads for 1080p.
    } else if width * height > 640 * 480 && number_of_cores >= 3 {
        return 2;  // 2 threads for qHD/HD.
    } else {
        return 1;  // 1 thread for VGA or less.
    }
}

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

    unsafe {
        let version = CStr::from_ptr(ffmpeg4_ffi::sys::av_version_info());
        println!("ffmpeg version: {}", version.to_string_lossy().into_owned());

        let configuration = CStr::from_ptr(ffmpeg4_ffi::sys::avcodec_configuration());
        println!("configuration: {}", configuration.to_string_lossy().into_owned());
    
        let codec = ffmpeg4_ffi::sys::avcodec_find_encoder(ffmpeg4_ffi::sys::AVCodecID_AV_CODEC_ID_H264);
        let codec_name = CStr::from_ptr((*codec).name);
        println!("loaded codec: {}", codec_name.to_string_lossy().into_owned());
    
        let codecContext = ffmpeg4_ffi::sys::avcodec_alloc_context3( codec );
     
        (*codecContext).rc_max_rate = 5000 * 1000;
        //vbv-bufsize=1000
        (*codecContext).rc_buffer_size = 1000 * 1000;
        //ref=1
        (*codecContext).refs = 1;

        (*codecContext).width = 1920;
        (*codecContext).height = 1080;
        (*codecContext).time_base.num = 1;
        (*codecContext).time_base.den = 30;

        // keyint=30;
        (*codecContext).gop_size = 3000;

        let number_of_cores = num_cpus::get() as u8;
        (*codecContext).thread_count = number_of_threads(1920, 1080, number_of_cores);
        
        // AV_PIX_FMT_BGRA - capture without NV12 - TODO
		// {
		// 	// We scale to YUV420p if we get RGB32 frames
		// 	(*codecContext).pix_fmt = AVPixelFormat_AV_PIX_FMT_YUV420P;
        // }
        
        // NV12
        (*codecContext).pix_fmt = ffmpeg4_ffi::sys::AVPixelFormat_AV_PIX_FMT_NV12;
        
        // // X264 options
        let mut codec_options: *mut AVDictionary = ptr::null_mut();

        // // -preset veryfast
        sys::av_dict_set(  &mut codec_options, 
                            CString::new("preset").unwrap().as_ptr(), 
                            CString::new("superfast").unwrap().as_ptr(), 0);
        // -tune zerolatency
        sys::av_dict_set(  &mut codec_options, 
                            CString::new("tune").unwrap().as_ptr(), 
                            CString::new("zerolatency").unwrap().as_ptr(), 0);
        
        // crf=10
        sys::av_opt_set_double(    (*codecContext).priv_data, 
                                    CString::new("crf").unwrap().as_ptr(), 
                                    10 as f64, 0);

        // intra-refresh=1
        sys::av_opt_set_int(   (*codecContext).priv_data, 
                                CString::new("intra-refresh").unwrap().as_ptr(), 
                                1, 0);

        // slice-max-size=1500
        sys::av_opt_set_int(   (*codecContext).priv_data, 
                                CString::new("slice-max-size").unwrap().as_ptr(), 
                                1500 * 1000, 0);

        let opened = sys::avcodec_open2(codecContext, codec, &mut codec_options);
        
        if opened < 0 {
            println!("Failed to open codec ;-(");
        } else {
            println!("Opened codec :-)");
        }
    }
    
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
