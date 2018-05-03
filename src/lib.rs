#![feature(lang_items, asm, const_fn, ptr_internals)]
#![no_std]

extern crate rlibc;
extern crate spin;

mod x86;
mod vga;
mod pio;
mod multiboot;
mod gdt;

use multiboot::*;
use gdt::*;
use vga::*;

#[no_mangle]
pub extern fn kernel_entry(multiboot_infos: *mut MultibootInfo) {
    vga_init(Color::Black, Color::White);
    println!("Screen initialized.");
    gdt_init();
    println!("GDT initialized.");
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", unsafe { (*multiboot_infos).mem_upper });
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}