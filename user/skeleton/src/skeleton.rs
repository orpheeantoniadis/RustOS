#![no_std]
#![macro_use]

extern crate ulibc;
use ulibc::*;

#[no_mangle]
pub extern fn main() {
    println!("Hello world!");
}