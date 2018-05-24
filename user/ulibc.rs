#![allow(dead_code)]
#![macro_use]

use core::*;
pub use common::Syscall;

extern "C" {
    pub fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
}

macro_rules! puts {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::Puts, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0);
        }
    });
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