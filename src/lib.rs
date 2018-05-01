#![feature(lang_items, asm, const_fn)]
#![no_std]

mod x86;
mod multiboot;
mod gdt;
mod memory;
#[macro_use]
mod vga;

use multiboot::*;
use gdt::*;
use vga::*;

use core::fmt::Write;

#[no_mangle]
pub extern fn kernel_entry(multiboot_infos: *mut MultibootInfo) {
    gdt_init();
    unsafe { SCREEN.clear(); }
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", (*multiboot_infos).mem_upper);
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}