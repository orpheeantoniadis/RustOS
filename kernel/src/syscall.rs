#![allow(dead_code)]

use vga::*;
use gdt::*;
use timer::*;
use keyboard::*;
use fs::*;
use task::*;
use common::*;

extern "C" {
    pub fn _syscall_handler();
}

// System call handler: call the appropriate system call according to the nb argument.
// Called by the assembly code _syscall_handler
#[no_mangle]
pub unsafe extern fn syscall_handler(nb: Syscall, _arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32, caller_tss_selector: u32) -> i32 {
    let idx = (selector_to_gdt_index(caller_tss_selector) as usize - GDT_SIZE) / 2;
    let addr = TASKS[idx].addr_space;
    match nb {
        Syscall::Puts => {syscall_puts(addr, _arg1)}
        Syscall::Exec => syscall_exec(addr, _arg1),
        Syscall::Keypressed => syscall_keypressed(),
        Syscall::Getc => syscall_getc(),
        Syscall::FileStat => syscall_file_stat(addr, _arg1, addr + _arg2),
        Syscall::FileOpen => syscall_file_open(addr, _arg1),
        Syscall::FileClose => syscall_file_close(_arg1),
        Syscall::FileRead => syscall_file_read(_arg1, addr + _arg2, _arg3),
        Syscall::FileSeek => syscall_file_seek(_arg1, _arg2),
        Syscall::FileIterator => syscall_file_iterator(addr + _arg1),
        Syscall::FileNext => syscall_file_next(addr, _arg1, addr + _arg2),
        Syscall::GetTicks => syscall_get_ticks(),
        Syscall::Sleep => syscall_sleep(_arg1)
    }
}

unsafe fn syscall_puts(base_addr: u32, string_offset: u32) -> i32 {
    let mut string = *((base_addr + string_offset) as *mut String);
    string.offset(base_addr);
    SCREEN.write_str(string.to_string());
    return 0;
}

unsafe fn syscall_exec(base_addr: u32, string_offset: u32) -> i32 {
    let mut string = *((base_addr + string_offset) as *mut String);
    string.offset(base_addr);
    exec(string.to_string()) as i32
}

unsafe fn syscall_keypressed() -> i32 {
    keypressed() as i32
}

unsafe fn syscall_getc() -> i32 {
    getc() as i32
}

unsafe fn syscall_file_stat(base_addr: u32, string_offset: u32, stat_addr: u32) -> i32 {
    let mut string = *((base_addr + string_offset) as *mut String);
    string.offset(base_addr);
    let stat = stat_addr as *mut Stat;
	*stat = Stat::new(string.to_string());
    if (*stat).start == 0 {
        return -1;
    }
    return 0;
}

unsafe fn syscall_file_open(base_addr: u32, string_offset: u32) -> i32 {
    let mut string = *((base_addr + string_offset) as *mut String);
    string.offset(base_addr);
    file_open(string.to_string())
}

unsafe fn syscall_file_close(fd: u32) -> i32 {
    file_close(fd as i32)
}

unsafe fn syscall_file_read(fd: u32, buf_addr: u32, n: u32) -> i32 {
    file_read(fd as i32, buf_addr as *mut u8, n as usize)
}

unsafe fn syscall_file_seek(fd: u32, offset: u32) -> i32 {
    file_seek(fd as i32, offset as usize)
}

unsafe fn syscall_file_iterator(it_addr: u32) -> i32 {
    let it = it_addr as *mut FileIterator;
    *it = FileIterator::new();
    return 0;
}

unsafe fn syscall_file_next(base_addr: u32, string_offset: u32, it_addr: u32) -> i32 {
    let bytes = (base_addr + string_offset) as *mut u8;
    let it = it_addr as *mut FileIterator;
    (*it).next(bytes) as i32
}

unsafe fn syscall_get_ticks() -> i32 {
    get_ticks() as i32
}

unsafe fn syscall_sleep(ms: u32) -> i32 {
    sleep(ms);
    return 0;
}