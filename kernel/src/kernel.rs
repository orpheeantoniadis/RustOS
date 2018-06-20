//! # RustOS
//!
//! `rust_os` is a kernel running on IA-32 architecture

#![feature(lang_items, asm, const_fn)]
#![no_std]

extern crate rlibc;
extern crate common;

pub mod x86;
pub mod multiboot;
pub mod vga;
pub mod pio;
pub mod paging;
pub mod kheap;
pub mod gdt;
pub mod pic;
pub mod idt;
pub mod timer;
pub mod keyboard;
pub mod ide;
pub mod fs;
pub mod task;
pub mod syscall;

use x86::*;
use multiboot::*;
use vga::*;
use pio::disable_cursor;
use paging::*;
use kheap::*;
use gdt::gdt_init; 
use pic::pic_init;
use idt::idt_init;
use timer::*;
use fs::*;
use task::*;
use common::Color;

// exports
pub use idt::exception_handler;
pub use idt::irq_handler;
pub use syscall::syscall_handler;

#[cfg(test)]
mod test;

/// Entrypoint to the rust code. This function is called by the bootstrap code
/// contain in bootstrap_asm.s
#[no_mangle]
pub extern fn kmain(_multiboot_magic: u32, multiboot_info: *mut MultibootInfo) {
    let mboot = unsafe { (*multiboot_info) };
    vga_init(Color::Black, Color::White);
    println!("Screen initialized.");
    paging_init();
    println!("Paging initialized.");
    kheap_init(mboot.mem_upper);
    println!("Heap initialized.");
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
    set_superblock();
    println!("Welcome to RustOS!");
    println!("Available Memory = {} kB", mboot.mem_upper);
    sleep(3000);
    exec("splash");
    exec("shell");
    print_kmalloc_list();
    disable_cursor();
    print!("\nKernel stopped.\nYou can turn off you computer.");
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(details: ::core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    println!("panicked at {}, {}:{}:{}", details, file, line, column);
    cli();
    halt();
    loop {}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    cli();
    halt();
    loop {}
}