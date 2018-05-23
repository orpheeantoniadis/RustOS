#![allow(dead_code)]

use keyboard::*;
use gdt::*;
use task::*;
use vga::*;
use fs::*;
use timer::get_ticks;
use common::Syscall;

extern "C" {
    pub fn _syscall_handler();
}

// System call handler: call the appropriate system call according to the nb argument.
// Called by the assembly code _syscall_handler
#[no_mangle]
pub unsafe extern fn syscall_handler(nb: Syscall, _arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32, caller_tss_selector: u32) -> i32 {
    let id = selector_to_gdt_index(caller_tss_selector) as usize - GDT_SIZE;
    let addr_space = &TASKS[id].addr_space as *const [u8;ADDR_SPACE_SIZE];
    match nb {
        Syscall::Puts => { SCREEN.write_str(bytes_to_str(&(*addr_space)[_arg1 as usize..])); return 0; }
        Syscall::Exec => 0,
        Syscall::Keypressed => keypressed() as i32,
        Syscall::Getc => getc() as i32,
        Syscall::FileStat => 0,
        Syscall::FileOpen => 0,
        Syscall::FileClose => 0,
        Syscall::FileRead => 0,
        Syscall::FileSeek => 0,
        Syscall::FileIterator => 0,
        Syscall::FileNext => 0,
        Syscall::GetTicks => get_ticks() as i32,
        Syscall::Sleep => 0
    }
}