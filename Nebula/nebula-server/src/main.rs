extern crate libc;
//use libc::{c_int, size_t};
//use std::time::{Instant};

#[link(name = "Nebula", kind = "static")]

extern {
    fn CreateManager(outWidth: u32, outHeight: u32) -> bool;
    fn IsSupported() -> bool;
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
fn main() {
    println!("Initialised Nebula C");
    println!("{}", create_manager(1920, 1080));
    println!("{}", is_supported());
}
