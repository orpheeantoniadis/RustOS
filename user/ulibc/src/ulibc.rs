#![feature(lang_items)]
#![no_std]

extern crate common;

pub use common::Syscall;

extern "C" {
    pub fn syscall(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> i32;
}

#[macro_export]
macro_rules! puts {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::Puts, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0);
        }
    });
}

#[macro_export]
macro_rules! exec {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::Exec, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0)
        }
    });
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

#[macro_export]
macro_rules! file_open {
    ($fmt:expr) => ({
        unsafe {
            syscall(Syscall::FileOpen, concat!($fmt, "\0").as_ptr() as u32, 0, 0, 0)
        }
    });
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