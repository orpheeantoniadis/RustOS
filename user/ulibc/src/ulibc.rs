#![feature(lang_items)]
// #![feature(alloc, global_allocator, allocator_api)]
#![no_std]

extern crate common;
pub use common::*;

extern crate rlibc;
pub use rlibc::*;

pub mod io;
pub mod curses;
pub mod mem;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}