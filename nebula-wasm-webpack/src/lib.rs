extern crate nebula_mp4;
mod utils;

use nebula_mp4::{
    h264,
    mp4
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use web_sys::{ErrorEvent, Event, FileReader, EventTarget, SourceBuffer, MessageEvent, Blob, WebSocket, BinaryType, MediaSource, Url, MediaSourceReadyState};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct State {
    pub initialised: bool,
    sequence_number: u32,
    data: Option<Vec<u8>>,
    source_buffer: Option<SourceBuffer>,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        State {
            initialised: false,
            sequence_number: 0,
            data: None,
            source_buffer: None,
        }
    }
}

#[wasm_bindgen]
pub fn process_packet(packet: &JsValue, media_source: &MediaSource, state: &mut State) {
    let a : Uint8Array = Uint8Array::new(packet);
    
    let mut data = vec![0; a.length() as usize];
    a.copy_to(&mut data[..]);

    let nalus = h264::parse_to_nalu(data);
    let mut size = 0;
    // console_log!("nalus size: {}", nalus.len());
    let mut video_track = mp4::Track::new();
    let mut units : Vec<h264::Nalu> = Vec::new();
    for nalu in nalus {
        size = size + nalu.get_size();
        // console_log!("nalu type: {:?}", nalu.ntype);
        match nalu.ntype {

            h264::NalType::SPS => {
                units.push(nalu.clone());
                video_track.parse_sps(nalu.payload);
                console_log!("parsing sps, width: {} height: {}", video_track.width, video_track.height);
            },
            h264::NalType::PPS => {
                units.push(nalu.clone());
                video_track.parse_pps(nalu.payload);
            },
            h264::NalType::IDR | h264::NalType::NDR => {
                units.push(nalu.clone());
                
                video_track.samples.push(mp4::Sample {
                    cts: 0,
                    duration: 30,  // We are assuming 30ish frames per second, TODO - need to add to protocol duration
                    flags: mp4::Flags {
                        is_leading: 0,
                        is_depended_on: 0,
                        has_redundancy: 0,
                        degrad_prio: 0,
                        is_non_sync: 0,
                        depends_on: 0,
                        padding_value: 0,
                    },
                    nalus: vec![nalu],
                    size,
                });
            },
            _ => {
                ();
            }
        }
    }
    
    if !state.initialised {

        let mime = format!("video/mp4; codecs=\"{}\"", video_track.codec);
        let mime_supported = MediaSource::is_type_supported(&mime);
        if mime_supported {
            state.source_buffer = Some(media_source.add_source_buffer(&mime).unwrap());
            let mut v = mp4::init_segment(vec![video_track], 0xffffffff, 1000);
            console_log!("segment length = {:?}", v.len());
            match state.data {
                None => {
                    console_log!("write first packet");
                    state.data = Some(v);
                },
                _ => {
                    console_log!("append data");
                    state.data.as_mut().unwrap().append(&mut v);
                }
            }
            
            state.initialised = true;
        } else {
            console_log!("unsupported mime: {}", mime);
        }

    } else {
        let decode_time = 0;
        let mut moof = mp4::moof(state.sequence_number, decode_time, &video_track);
        let mut mdat = mp4::mdat(&video_track.samples[0].nalus[0].payload);
        moof.append(&mut mdat);
        
        console_log!("media source state = {:?}", media_source.ready_state());
        state.data.as_mut().unwrap().append(&mut moof);
        
        state.sequence_number = state.sequence_number + 1;
    }
}

#[wasm_bindgen]
pub fn write_to_buffer(state: &mut State) {
    match state.source_buffer {
        None => (),
        _ => {
            if !state.source_buffer.as_ref().unwrap().updating() && state.data.as_ref().unwrap().len() > 0 {
                console_log!("append buffer with length = {}", state.data.as_ref().unwrap().len());
                let mut v = state.data.as_mut().unwrap();
                state.source_buffer.as_ref().unwrap().append_buffer_with_u8_array(v);
                state.data.as_mut().unwrap().clear();
            } else {
                console_log!("updating source buffer so no write");
            }
        }
    }
}

#[wasm_bindgen]
pub fn request_new_frame(ws: &WebSocket) {
    // Grab next frame
    let mut cmd : [u8;1] = ['f' as u8];

    match ws.send_with_u8_array(&mut cmd) {
        Ok(_) => (),
        Err(err) => console_log!("error sending message: {:?}", err),
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}
