#![no_std]

extern crate ulibc;
use ulibc::*;
use io::*;

#[no_mangle]
pub extern fn main() {
    println!("Hello world!");
}