#![feature(lang_items)]
#![no_std]

extern crate common;
pub use common::*;
use core::fmt::{Error, Write, Arguments};

extern "C" {
    pub fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
}

pub struct Stdout {}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        unsafe { syscall(Syscall::Puts, &String::new(s) as *const String as u32,  0, 0, 0); }
        Ok(())
    }
}

pub fn write_fmt(args: Arguments) {
    Stdout {}.write_fmt(args).ok();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (write_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn puts(s: &str) {
    unsafe {
        syscall(Syscall::Puts, &String::new(s) as *const String as u32,  0, 0, 0);
    }
}

pub fn exec(s: &str) {
    unsafe { 
        syscall(Syscall::Exec, &String::new(s) as *const String as u32,  0, 0, 0);
    }
}

pub fn keypressed() -> i32 {
    unsafe {
        syscall(Syscall::Keypressed, 0, 0, 0, 0)
    }
}

pub fn getc() -> i32 {
    unsafe {
        syscall(Syscall::Getc, 0, 0, 0, 0)
    }
}

pub fn file_open(s: &str) -> i32 {
    unsafe {
        syscall(Syscall::FileOpen, &String::new(s) as *const String as u32, 0, 0, 0)
    }
}

pub fn file_close(fd: u32) -> i32 {
    unsafe {
        syscall(Syscall::FileClose, fd, 0, 0, 0)
    }
}

pub fn file_read(fd: u32, buf: *mut u8, n: u32) -> i32 {
    unsafe {
        syscall(Syscall::FileRead, fd, buf as u32, n, 0)
    }
}

pub fn file_seek(fd: u32, offset: u32) -> i32 {
    unsafe {
        syscall(Syscall::FileSeek, fd, offset, 0, 0)
    }
}

pub fn get_ticks() -> i32 {
    unsafe {
        syscall(Syscall::GetTicks, 0, 0, 0, 0)
    }
}

pub fn sleep(ms: u32) {
    unsafe {
        syscall(Syscall::Sleep, ms, 0, 0, 0);
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}