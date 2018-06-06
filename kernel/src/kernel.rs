//! # RustOS
//!
//! `rust_os` is a kernel running on IA-32 architecture

#![feature(lang_items, asm, const_fn)]
#![no_std]

/// Entrypoint to the rust code. This function is called by the bootstrap code
/// contain in bootstrap_asm.s
#[no_mangle]
pub extern fn kmain() {
    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_addr: u32 = 0xC00B8000;
    let buffer_ptr = (buffer_addr + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop{}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(_details: ::core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
    loop{};
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}