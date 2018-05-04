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
mod timer;

#[cfg(test)]
mod test;

use x86::*;
use vga::*;
use multiboot::*;
use gdt::gdt_init;
use pic::pic_init;
use idt::idt_init;
use timer::*;

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
    sti();
    println!("Interrupts unmasked.");
    timer_init(MIN_FREQ);
    println!("PIT initialized.");
    sleep(1000);
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