#![no_std]
#![macro_use]

extern crate ulibc;
pub use ulibc::*;

#[no_mangle]
pub extern fn main() {
    puts!("Hello world!");
}