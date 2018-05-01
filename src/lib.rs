#![feature(lang_items, asm, const_fn)]
#![no_std]

mod x86;
mod multiboot;
mod gdt;
mod memory;
mod vga;

use multiboot::*;
use gdt::*;
use vga::*;

#[no_mangle]
pub extern fn kernel_entry(_multiboot_infos: *mut MultibootInfo) {
    gdt_init();
    
    let hello = "Hello World!";
    let mut i = 34;
    for byte in hello.bytes() {
        unsafe {
            (*BUFFER)[BUFFER_HEIGHT/2][i] = Character::new(byte, ColorAttribute::new(Color::Black, Color::White));
        }
        i+=1;
    }

    loop{}
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}