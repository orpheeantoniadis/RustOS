#![feature(lang_items, asm, const_fn)]
#![no_std]

extern crate rlibc;

mod x86;
mod vga;
mod pio;
mod multiboot;
mod gdt;
mod pic;
mod idt;

#[cfg(test)]
mod test;

use vga::*;
use multiboot::*;
use gdt::gdt_init;
use pic::pic_init;
use idt::idt_init;

// exports
pub use idt::exception_handler;
pub use idt::irq_handler;

#[no_mangle]
pub extern fn kernel_entry(multiboot_infos: *mut MultibootInfo) {
    vga_init(Color::Black, Color::White);
    println!("Screen initialized.");
    gdt_init();
    println!("GDT initialized.");
    pic_init();
    println!("PIC initialized.");
    idt_init();
    println!("IDT initialized.");
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", (*multiboot_infos).mem_upper);    
    loop{}
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(details: ::core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    println!("panicked at {}, {}:{}:{}", details, file, line, column);
    loop{};
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}