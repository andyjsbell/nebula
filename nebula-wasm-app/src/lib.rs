mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, Event, EventTarget, SourceBuffer, MessageEvent, WebSocket, Blob, MediaSource, Url};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
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

#[derive(Debug)]
pub struct Player {
    pub media_source: MediaSource,
}

impl Player {

    // pub fn new() -> Player {
        
    // }
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    let sourceopen_callback = Closure::wrap(Box::new(move |e: Event| {

        console_log!("source open");

        let mime = "video/mp4; codecs=\"avc1.42E01E, mp4a.40.2\"";
        let et = e.target().unwrap();
        let media_source = JsCast::unchecked_ref::<MediaSource>(&et);
        let source_buffer = media_source.add_source_buffer(mime).unwrap();
        // get buffer from socket, wait...
        // TODO 
        // append buffer

        // Check updating
        let updating = source_buffer.updating();

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

pub fn start_websocket() -> Result<(), JsValue> {
    // Connect to an nebula server
    let ws = WebSocket::new("ws://localhost:9001/socket")?;
    {
        let cloned_ws = ws.clone();
        // create callback
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {

            // handle message
            let response = e.data();
            let blob = Blob::from(response);
            console_log!("message received {:?}", blob.size());
            let mut cmd : [u8;1] = ['f' as u8];
            match cloned_ws.send_with_u8_array(&mut cmd) {
                Ok(_) => console_log!("message successfully sent"),
                Err(err) => console_log!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(MessageEvent)>);
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