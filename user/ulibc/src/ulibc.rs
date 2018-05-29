#![feature(lang_items)]
#![no_std]

extern crate common;
pub use common::*;

extern crate rlibc;
pub use rlibc::*;

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

pub fn clear() {
    for _i in 0..BUFFER_HEIGHT {
        putc(b'\n');
    }
    set_cursor(0, 0);
}

pub fn puts(s: &str) {
    unsafe {
        syscall(Syscall::Puts, String::new(s).as_ptr() as u32,  0, 0, 0);
    }
}

pub fn putc(byte: u8) {
    unsafe {
        syscall(Syscall::Putc, byte as u32,  0, 0, 0);
    }
}

pub fn exec(s: &str) -> i32 {
    unsafe { 
        syscall(Syscall::Exec, String::new(s).as_ptr() as u32,  0, 0, 0)
    }
}

pub fn keypressed() -> i32 {
    unsafe {
        syscall(Syscall::Keypressed, 0, 0, 0, 0)
    }
}

pub fn getc() -> u8 {
    unsafe {
        syscall(Syscall::Getc, 0, 0, 0, 0) as u8
    }
}

pub fn file_stat(s: &str) -> Stat {
    let mut stat = Stat::null();
    unsafe {
        syscall(Syscall::FileStat, String::new(s).as_ptr() as u32, stat.as_ptr() as u32, 0, 0);
    }
    return stat;
}

pub fn file_open(s: &str) -> i32 {
    unsafe {
        syscall(Syscall::FileOpen, String::new(s).as_ptr() as u32, 0, 0, 0)
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

pub fn file_iterator() -> FileIterator {
    let mut it = FileIterator::null();
    unsafe {
        syscall(Syscall::FileIterator, it.as_ptr() as u32, 0, 0, 0);
    }
    return it;
}

pub fn file_next(bytes: *const u8, it: *const FileIterator) -> i32 {
    unsafe {
        syscall(Syscall::FileNext, bytes as u32, it as u32, 0, 0)
    }
}

pub fn get_ticks() -> u32 {
    unsafe {
        syscall(Syscall::GetTicks, 0, 0, 0, 0) as u32
    }
}

pub fn sleep(ms: u32) {
    unsafe {
        syscall(Syscall::Sleep, ms, 0, 0, 0);
    }
}

pub fn set_cursor(x: u32, y: u32) {
    unsafe {
        syscall(Syscall::SetCursor, x, y, 0, 0);
    }
}

pub fn get_cursor(x: *const u32, y: *const u32) {
    unsafe {
        syscall(Syscall::GetCursor, x as u32, y as u32, 0, 0);
    }
}

pub fn cursor_disable(cd: bool) {
    unsafe {
        if cd {
            syscall(Syscall::CursorDisable, 1, 0, 0, 0);
        } else {
            syscall(Syscall::CursorDisable, 0, 0, 0, 0);
        }
    }
}

pub fn set_color(background: Color, foreground: Color) {
    unsafe {
        syscall(Syscall::SetColor, Color::to_u32(background), Color::to_u32(foreground), 0, 0);
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