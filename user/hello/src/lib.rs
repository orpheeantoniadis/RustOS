#![feature(lang_items)]
#![no_std]

// extern crate rlibc;
// extern crate common;
// 
// use common::Syscall;
// 
// fn puts(string: &str) {
//     unsafe {
//         syscall(Syscall::Puts, string.as_ptr() as u32, 0, 0, 0);
//     }
// }
// 
// extern "C" {
//     fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
// }

#[no_mangle]
pub extern fn main() {
    // puts("Hello world!");
    let x = 1;
    let _y = x * 2;
    loop{};
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop{};
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}