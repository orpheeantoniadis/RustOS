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
mod keyboard;
mod ide;
mod fs;

#[cfg(test)]
mod test;

use x86::*;
use vga::*;
use multiboot::*;
use gdt::gdt_init;
use pic::pic_init;
use idt::idt_init;
use timer::*;
use keyboard::*;
use fs::*;

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
    timer_init(50);
    println!("PIT initialized.");
    
    file_exists("README.md");
    
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", (*multiboot_infos).mem_upper);
    loop{
        let key = getc();
        if key == 'Q' {
            println!("\nKernel stopped.");
            break;
        }
        print!("{}", key);
    }
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