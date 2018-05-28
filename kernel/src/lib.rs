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

#[cfg(test)]
mod test;

use x86::*;
use vga::*;
use pio::*;
use multiboot::*;
use gdt::gdt_init;
use pic::pic_init;
use idt::idt_init;
use timer::*;
use fs::*;
use task::*;
use common::bytes_to_str;

// exports
pub use idt::exception_handler;
pub use idt::irq_handler;
pub use syscall::syscall_handler;

/// Displays the splash screen of the kernel
fn splash_screen() {
    sleep(3000);
    vga_clear();
    vga_set_cursor(22,10);
    let fd = file_open("splash.txt");
    let mut data = [0;300];
    file_read(fd, &mut data[0], 300);
    for c in bytes_to_str(&data).chars() {
        print!("{}", c);
        if c == '\n' {
            let cursor = vga_get_cursor();
            vga_set_cursor(22,cursor.1);
        }
    }
    file_close(fd);
    disable_cursor();
    sleep(5000);
    enable_cursor();
    vga_clear();
}

/// Entrypoint to the rust code. This function is called by the bootstrap code
/// contain in bootstrap_asm.s
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
    set_superblock();
    println!("Available Memory = {} kB", unsafe { (*multiboot_infos) }.mem_upper);
    splash_screen();
    exec("shell");
    disable_cursor();
    print!("\nKernel stopped.\nYou can turn off you computer.");
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