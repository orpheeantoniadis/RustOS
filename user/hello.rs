#![feature(lang_items, no_core, core_panic)]
#![no_core]

extern crate core;

#[repr(u8)]
pub enum Syscall {
    Puts            = 0x0,
    Exec            = 0x1,
    Keypressed      = 0x2,
    Getc            = 0x3,
    FileStat        = 0x4,
    FileOpen        = 0x5,
    FileClose       = 0x6,
    FileRead        = 0x7,
    FileSeek        = 0x8,
    FileIterator    = 0x9,
    FileNext        = 0xa,
    GetTicks        = 0xb,
    Sleep           = 0xc
}

macro_rules! puts {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::Puts, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0);
        }
    });
}

extern "C" {
    fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
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