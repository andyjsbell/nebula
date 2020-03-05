mod utils;
mod mp4;
mod h264;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use web_sys::{ErrorEvent, Event, FileReader, EventTarget, SourceBuffer, MessageEvent, WebSocket, Blob, MediaSource, Url};
use std::sync::Arc;

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

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
struct VideoPlayer {
    pub initialised : bool,

}

#[wasm_bindgen]
impl VideoPlayer {
    
    pub fn new(&self) -> VideoPlayer {
        VideoPlayer{
            initialised: false,
        }
    }

    pub fn is_initialised(&self) -> bool {
        self.initialised
    }
    
    // Pass our media source which is already attaced to the video element for playback
    // At the moment we are taking ownership of self to be able to use self within the callback, TODO use references in callback 'lifetimes'
    pub fn start(&self, value: JsValue) {
   
        // Connect to server
        let ws = WebSocket::new("ws://localhost:9001/socket").unwrap();
        {
            let vp = self.clone();  // Create a referenced count
        
            let cloned_ws = ws.clone();
            /////////////////////////////////////////////////
            // Create file reader to handle the packets coming in from the websocket
            ///////////////////////////////////////////////// 
            let onload_callback = Closure::wrap(Box::new(move |e: Event| {
                
                let et = e.target().unwrap();
                let file_reader = JsCast::unchecked_ref::<FileReader>(&et);
                let a : Uint8Array = Uint8Array::new(&file_reader.result().expect("unable to read result from filereader"));
        
                let mut data = vec![0; a.length() as usize];
                a.copy_to(&mut data[..]);

                let nalus = h264::parse_to_nalu(data);
                let mut samples : Vec<mp4::Sample> = Vec::new();
                let mut size = 0;
                let v = &mut Vec::<h264::Nalu>::new();

                for nalu in nalus {
                    size = size + nalu.get_size();
                    v.push(nalu);
                
                    if v.last().unwrap().ntype == h264::IDR || v.last().unwrap().ntype == h264::NDR {
                    
                        let sample: mp4::Sample = mp4::Sample {
                            cts: 0,
                            duration: 30 * v.len() as u32,  // We are assuming 30ish frames per second, TODO - need to add to protocol duration
                            flags: mp4::Flags {
                                is_leading: 0,
                                is_depended_on: 0,
                                has_redundancy: 0,
                                degrad_prio: 0,
                                is_non_sync: 0,
                                depends_on: 0,
                                padding_value: 0,
                            },
                            nalus: v.to_vec(),
                            size: size,
                        };

                        samples.push(sample);
                    }
                }

                let video_track = mp4::Track::new();
                
                if vp.initialised {
                    let mime = format!("video/mp4; codecs=\"{}\"", video_track.codec);
                    let media_source = JsCast::unchecked_ref::<MediaSource>(&value);
                    let source_buffer = media_source.add_source_buffer(&mime).unwrap();
                    source_buffer.append_buffer_with_array_buffer(&a.buffer()).unwrap();
                    mp4::init_segment(vec![video_track], 0xffffffff, 1000);
                    // self.initialised = true;
                } else {
                    let sequence_number = 0; // this needs to increase on each atom
                    let decode_time = 0;
                    let mut moof = mp4::moof(sequence_number, decode_time, &video_track);
                    let mut mdat = mp4::mdat([0,0,0,0]);
                    moof.append(&mut mdat);
                }

                // Grab next frame
                let mut cmd : [u8;1] = ['f' as u8];

                match cloned_ws.send_with_u8_array(&mut cmd) {
                    Ok(_) => console_log!("message successfully sent"),
                    Err(err) => console_log!("error sending message: {:?}", err),
                }

            }) as Box<dyn FnMut(Event)>);

            // The filereader
            let file_reader = FileReader::new().expect("Unable to create filereader");
            // Attach to event
            file_reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
            onload_callback.forget();
            /////////////////////////////////////////////////
            // End
            /////////////////////////////////////////////////          

            /////////////////////////////////////////////////
            // Handle the packets coming in on the web socket
            /////////////////////////////////////////////////
            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {

                let response = e.data();
                let blob = Blob::from(response);
                // Load blob into file reader and when ready write into source buffer
                
                file_reader.read_as_array_buffer(&blob).expect("failed to read as array buffer");    

            }) as Box<dyn FnMut(MessageEvent)>);

            // Attach event
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();
            /////////////////////////////////////////////////
            // End
            /////////////////////////////////////////////////
        }

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console_log!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("socket opened");
            let mut cmd : [u8;1] = ['f' as u8];
            match cloned_ws.send_with_u8_array(&mut cmd) {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }
}

