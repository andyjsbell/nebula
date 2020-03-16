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
    pub sequence_number: u32,
    // pub source_buffer: &SourceBuffer,
}

#[wasm_bindgen]
impl State {
    pub fn new() -> State {
        State {
            initialised: false,
            sequence_number: 0,
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
                    size: size,
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
            // state.source_buffer = Some(media_source.add_source_buffer(&mime).unwrap());
            let mut v = mp4::init_segment(vec![video_track], 0xffffffff, 1000);

            // state.source_buffer.as_ref().unwrap().append_buffer_with_u8_array(&mut v).unwrap();
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
        if media_source.ready_state() == MediaSourceReadyState::Open {
            console_log!("writing to source buffer");
            // state.source_buffer.as_ref().unwrap().append_buffer_with_u8_array(&mut moof).unwrap();
        } else {
            console_log!("media source not open");
        }
        
        state.sequence_number = state.sequence_number + 1;
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

fn on_opensource(event: Event) {
    console_log!("on_opensource {:?}", event);

    let event_target : EventTarget = event.target().unwrap();
    match event_target.dyn_into::<MediaSource>() {
        Ok(media_source) => {
            console_log!("Creating websocket");
            
            let ws = WebSocket::new("ws://localhost:9001/socket").unwrap();
            ws.set_binary_type(BinaryType::Arraybuffer);

            let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                console_log!("error event: {:?}", e);
            }) as Box<dyn FnMut(ErrorEvent)>);
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();

            let cloned_ws = ws.clone();
            let onopen_callback = Closure::wrap(Box::new(move |_| {
                console_log!("Socket opened");
                let mut cmd : [u8;1] = ['f' as u8];
                match cloned_ws.send_with_u8_array(&mut cmd) {
                    Ok(_) => console_log!("message successfully sent"),
                    Err(err) => console_log!("error sending message: {:?}", err),
                }
            }) as Box<dyn FnMut(JsValue)>);
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();

            let cloned_ws = ws.clone();
            let mut state = State {
                initialised: false,
                sequence_number: 0,
                // source_buffer:None,
            };
            
            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {

                let response = e.data();
                console_log!("{:?}", response); 
                process_packet(&response, &media_source, &mut state);
                request_new_frame(&cloned_ws);

            }) as Box<dyn FnMut(MessageEvent)>);
            
            // Attach event
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
        }
        Err(event_target) => {

        }
    }
}

#[wasm_bindgen]
pub fn app(value: &JsValue) {
    
    let media_source : &MediaSource = JsCast::unchecked_ref::<MediaSource>(value);
    console_log!("media_source = {:?}", media_source);
    // Create MediaSource
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    // match document.query_selector("video").unwrap() {
    //     Some(video) => {
            // let media_source = MediaSource::new().unwrap();
            // let video_src = Url::create_object_url_with_source(&media_source).unwrap();
            // video.set_attribute("src", &video_src).unwrap();
            let sourceopen_callback = Closure::wrap(Box::new(|event: Event| on_opensource(event)) as Box<dyn FnMut(Event)>);    
            media_source.set_onsourceopen(Some(sourceopen_callback.as_ref().unchecked_ref()));
            sourceopen_callback.forget();
            
            let sourceclosed_callback = Closure::wrap(Box::new(|event: Event| {
                console_log!("media source closed");
            }) as Box<dyn FnMut(Event)>);    
            media_source.set_onsourceclosed(Some(sourceclosed_callback.as_ref().unchecked_ref()));
            sourceclosed_callback.forget();

    //     }
    //     None => {
    //         console_log!("No video element present");
    //     }
    // }
}

#[wasm_bindgen]
pub fn take_js_value_by_shared_ref(x: &JsValue) {
    console_log!("{:?}", x);


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
