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
    
    {
        let mut raw_filename = [0;MAX_FILENAME_LENGTH];
        let mut it = FileIterator::new();
        while it.has_next() {
            it.next(&mut raw_filename[0]);
            let filename = bytes_to_str(&raw_filename);
            let stat = Stat::new(filename);
            println!("{} ({} bytes)", filename, stat.size);
        }
        
        let mut data = [0;50];
        let fd = file_open("README.md");
        file_seek(fd, 37);
        file_read(fd, &mut data[0], 37);
        println!("{}", bytes_to_str(&data));
        
        let mut data = [0;80];
        file_read(fd, &mut data[0], 10);
        println!("{}", bytes_to_str(&data));
        file_read(fd, &mut data[0], 10);
        println!("{}", bytes_to_str(&data));
        file_read(fd, &mut data[0], 15);
        println!("{}", bytes_to_str(&data));
    }
    
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