#![allow(unused_attributes)]
#![feature(lang_items, no_core)]
#![no_std]
#![no_core]

extern crate core;
extern crate common;

mod ulibc;
pub use ulibc::*;

#[no_mangle]
pub extern fn main() {
    puts!("Hello world!");
}