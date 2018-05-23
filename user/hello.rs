#![feature(lang_items, no_core)]
#![no_core]

extern crate core;
extern crate common;

use common::Syscall;

extern "C" {
    fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
}

macro_rules! puts {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::Puts, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0);
        }
    });
}

#[no_mangle]
pub extern fn main() {
    puts!("Hello world!");
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn _ZN4core9panicking5panic17h7ce2d5c1853dff72E() {
    loop{}
}