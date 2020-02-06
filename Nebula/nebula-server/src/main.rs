extern crate libc;
//use libc::{c_int, size_t};
//use std::time::{Instant};

#[link(name = "Nebula", kind = "static")]
extern {
    fn CreateManager(outWidth: u32, outHeight: u32) -> bool;
    fn IsSupported() -> bool;
    fn GetOutputRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
    fn GetOriginalRect(x: &u32, y: &u32, width: &u32, height: &u32) -> bool;
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
    println!("{}", create_manager(1920, 1080));
    println!("{}", is_supported());
    println!("{:?}", get_output_rect());
    println!("{:?}", get_original_rect());
}
