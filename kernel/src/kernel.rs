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
pub mod gdt;
pub mod pic;
pub mod idt;
pub mod timer;
pub mod keyboard;
pub mod ide;
pub mod fs;
pub mod task;
pub mod syscall;

use x86::sti;
use multiboot::*;
use vga::*;
use pio::{enable_cursor,disable_cursor};
use paging::*;
use gdt::gdt_init; 
use pic::pic_init;
use idt::idt_init;
use timer::{timer_init,sleep};
use keyboard::getc;
use fs::*;
use task::*;
use common::{Color,bytes_to_str};

// exports
pub use idt::exception_handler;
pub use idt::irq_handler;
pub use syscall::syscall_handler;

#[cfg(test)]
mod test;

#[allow(dead_code)]
fn splash_screen() {
    disable_cursor();
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
    sleep(5000);
    enable_cursor();
}

/// Entrypoint to the rust code. This function is called by the bootstrap code
/// contain in bootstrap_asm.s
#[no_mangle]
pub extern fn kmain(_multiboot_magic: u32, multiboot_info: *mut MultibootInfo) {
    let mboot = unsafe { (*multiboot_info) };
    vga_init(Color::Black, Color::White);
    println!("Screen initialized.");
    paging_init();
    println!("Paging initialized.");
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
    exec("hello");
    unsafe {
        let frame = INIT_PD.alloc_frame(KERNEL_MODE);
        *(frame as *mut u8) = 0x42;
    }
    // sleep(3000);
    // splash_screen();
    // vga_clear();
    loop {
        let key = getc() as u8;
        if key == b'Q' {
            break;
        }
        vga_write_byte(key);
    }
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