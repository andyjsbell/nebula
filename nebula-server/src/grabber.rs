#[derive(Debug)]
pub struct Rect(u32, u32, u32, u32);

#[derive(Debug)]
pub struct Frame {
    pub data: Vec<u8>,
    pub nv12: bool,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    pub fn new(width: u32, height: u32, nv12: bool) -> Frame {
        Frame {
            data: vec![0],
            nv12: nv12,
            width: width,
            height: height,
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
                    nv12: nv12,
                    width: output_rect.2,
                    height: output_rect.3,
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