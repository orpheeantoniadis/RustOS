//! # RustOS
//!
//! `rust_os` is a kernel running on IA-32 architecture

#![feature(lang_items, asm, const_fn)]
#![no_std]

extern crate rlibc;
extern crate common;

pub mod x86;
pub mod vga;
pub mod pio;
pub mod multiboot;
pub mod gdt;
pub mod pic;
pub mod idt;
pub mod timer;
pub mod keyboard;
pub mod ide;
pub mod fs;
pub mod task;
pub mod syscall;
pub mod paging;

#[cfg(test)]
mod test;

use x86::*;
use vga::*;
use multiboot::*;
use gdt::gdt_init;
use pic::pic_init;
use idt::idt_init;
use timer::*;
use paging::*;
use common::Color;

// exports
pub use idt::exception_handler;
pub use idt::irq_handler;
pub use syscall::syscall_handler;

/// Entrypoint to the rust code. This function is called by the bootstrap code
/// contain in bootstrap_asm.s
#[no_mangle]
pub extern fn kernel_entry(multiboot_infos: *mut MultibootInfo) {
    let mboot = unsafe { (*multiboot_infos) };
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
    paging_init(multiboot_infos);
    println!("Paging initialized.");
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", mboot.mem_upper);
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