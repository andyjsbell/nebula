mod utils;
mod mp4;
mod h264;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use web_sys::{ErrorEvent, Event, FileReader, EventTarget, SourceBuffer, MessageEvent, WebSocket, Blob, MediaSource, Url};
use once_cell::sync::Lazy; // 1.3.1
use std::sync::Mutex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
static INITIALIZED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let sourceopen_callback = Closure::wrap(Box::new(|e: Event| {

        console_log!("source open");
        let et = e.target().unwrap();
        let media_source = JsCast::unchecked_ref::<MediaSource>(&et);
        
        start_websocket(media_source).unwrap();

    }) as Box<dyn FnMut(Event)>);

    match document.query_selector("video").unwrap() {
        Some(video) => {
            let media_source = MediaSource::new().unwrap();
            let video_src = Url::create_object_url_with_source(&media_source).unwrap();
            video.set_attribute("src", &video_src).unwrap();
            // console_log!("{:?}", );
            media_source.set_onsourceopen(Some(sourceopen_callback.as_ref().unchecked_ref()));
        }
        None => {
            console_log!("No video element present");
        }
    }
    
    sourceopen_callback.forget();
    Ok(())
}

pub fn start_websocket(media_source: &MediaSource) -> Result<(), JsValue> {
    
    // Connect to an nebula server
    let ws = WebSocket::new("ws://localhost:9001/socket")?;
    {
        let cloned_ws = ws.clone();

        let onload_callback = Closure::wrap(Box::new(move |ev: Event| {
            let et = ev.target().unwrap();
            let file_reader = JsCast::unchecked_ref::<FileReader>(&et);
            console_log!("on load of file reader {:?}", file_reader);
            let a : Uint8Array = Uint8Array::new(&file_reader.result().expect("unable to read result from filereader"));
            console_log!("array length {}", a.length());
            
            // Break out into NAL UNITS
            if *INITIALIZED.lock().unwrap() {
                let t = mp4::Track::new();
                mp4::init_segment(vec![t], 0xffffffff, 1000);
                *INITIALIZED.lock().unwrap() = true;
            } else {
                let t = mp4::Track::new();
                let sequence_number = 0; // this needs to increase on each atom
                let decode_time = 0;
                let mut moof = mp4::moof(sequence_number, decode_time, &t);
                let mut mdat = mp4::mdat([0,0,0,0]);
                moof.append(&mut mdat);
            }
            // mp4::init_segment()
            // let mime = "video/mp4; codecs=\"avc1.42E01E\"";
            // let source_buffer = media_source.add_source_buffer(mime).unwrap();
            // source_buffer.append_buffer_with_array_buffer(&a.buffer()).unwrap();

            let mut cmd : [u8;1] = ['f' as u8];

            match cloned_ws.send_with_u8_array(&mut cmd) {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(Event)>);

        // create callback
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {

            // handle message
            let response = e.data();
            let blob = Blob::from(response);
            // Load blob into file reader and when ready write into source buffer
            let file_reader = FileReader::new().expect("Unable to create filereader");
            file_reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
            // forget the callback to keep it alive
            
            file_reader.read_as_array_buffer(&blob).expect("failed to read as array buffer");    

        }) as Box<dyn FnMut(MessageEvent)>);
        
        // Commented out as causing compilation issue, not sure if this is needed...
        // onload_callback.forget();            
        
        // set message event handler on WebSocket
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();
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

    Ok(())
}