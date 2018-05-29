#![feature(lang_items)]
#![no_std]

extern crate common;
pub use common::*;

extern crate rlibc;
pub use rlibc::*;

pub mod io;
pub mod curses;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}