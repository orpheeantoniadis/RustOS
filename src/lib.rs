#![feature(lang_items, asm, const_fn)]
#![no_std]

mod x86;
mod vga;
mod multiboot;
mod gdt;
mod memory;

use multiboot::*;
use gdt::*;
use vga::*;

#[no_mangle]
pub extern fn kernel_entry(multiboot_infos: *mut MultibootInfo) {
    clear_screen();
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", (*multiboot_infos).mem_upper);
    println!("Multiboot Info : {:?}", (*multiboot_infos));
    gdt_init();
    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}