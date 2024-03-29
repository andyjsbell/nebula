extern crate ffmpeg4_ffi;

use ffmpeg4_ffi::sys::AVPixelFormat;

#[derive(Debug)]
pub struct Rect(u32, u32, u32, u32);

impl Rect {
    pub fn size(self) -> Size {
        Size(self.2 - self.0, self.3 - self.0)
    }
}

#[derive(Debug)]
pub struct Size(pub u32, pub u32);

#[derive(Debug)]
pub struct Frame {
    pub data: Vec<u8>,
    pub resolution: Size,
    pub screen_resolution: Size,
    pub format: AVPixelFormat,
    pub time: u32,
}

impl Frame {
    pub fn new(resolution: Size, screen_resolution: Size, format: AVPixelFormat, time: u32) -> Frame {
        Frame {
            data: vec![0],
            resolution: resolution,
            screen_resolution: screen_resolution,
            format: format,
            time: time,
        }
    }
}

extern {
    fn CreateManager(outWidth: u32, outHeight: u32) -> bool;
    fn IsSupported() -> bool;
    fn GetOutputRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOriginalRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOutputBits(buffer: &*mut u8, outLen: &u32, nv12: &bool) -> bool;
}

pub fn get_output_bits() -> Option<Frame> {
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
                    resolution: output_rect.size(),
                    screen_resolution: Size(0, 0),
                    time: 0,
                    format: if nv12 { 
                        ffmpeg4_ffi::sys::AVPixelFormat_AV_PIX_FMT_NV12 
                    } else {
                        ffmpeg4_ffi::sys::AVPixelFormat_AV_PIX_FMT_BGRA
                    },
                });
        }
    }

    None
}

pub fn create_manager(out_width: u32, out_height: u32) -> bool {
    unsafe {
        return CreateManager(out_width, out_height);
    }
}

pub fn is_supported() -> bool {
    unsafe {
        IsSupported()
    }
}

pub fn get_output_rect() -> Rect {
    let x: u32 = 0;
    let y: u32 = 0;
    let width: u32 = 0;
    let height: u32 = 0;

    unsafe {       
        GetOutputRect(&x, &y, &width, &height);
    }

    Rect(x, y, width, height)
}

pub fn get_original_rect() -> Rect {
    let x: u32 = 0;
    let y: u32 = 0;
    let width: u32 = 0;
    let height: u32 = 0;

    unsafe {
        GetOriginalRect(&x, &y, &width, &height);
    }

    Rect(x, y, width, height)
}