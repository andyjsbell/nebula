extern crate libc;
//use libc::{c_int, size_t};
//use std::time::{Instant};

#[link(name = "Nebula", kind = "static")]
extern {
    fn CreateManager(outWidth: u32, outHeight: u32) -> bool;
    fn IsSupported() -> bool;
    fn GetOutputRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOriginalRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOutputBits(buffer: &*mut u8, outLen: &u32, nv12: &bool) -> bool;
}

struct Frame {
    data: Vec<u8>,
    nv12: bool,
    width: u32,
    height: u32,
}

fn get_output_bits() -> Option<Frame> {
    let output_rect = get_output_rect();

    let mut dstlen = output_rect.2 * output_rect.3 * 4;

    let mut dst:Vec<u8> = Vec::with_capacity(dstlen as usize);
    let pdst = dst.as_mut_ptr();
    let nv12 = false;
    
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

fn get_original_rect() -> Rect {
    unsafe {
        let x: u32 = 0;
        let y: u32 = 0;
        let width: u32 = 0;
        let height: u32 = 0;

        GetOriginalRect(&x, &y, &width, &height);

        Rect(x, y, width, height)
    }
}

fn main() {
    
    println!("Initialised Nebula C");
    if !create_manager(1920, 1080) {
        println!("Failed to create manager");
        return;
    }

    if !is_supported() {
        println!("Not supported");
        return;
    }

    match get_output_bits(){
        None => println!("Failed to get frame"),
        Some(frame) => {
            println!("We have a frame nv12={0} width={1} height={2} size={3}",
                    frame.nv12,
                    frame.width,
                    frame.height,
                    frame.data.len());
        }
    }
}
