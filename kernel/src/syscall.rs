#![allow(dead_code)]

use vga::*;
use gdt::*;
use timer::*;
use keyboard::*;
use fs::*;
use task::*;
use common::Syscall;

extern "C" {
    pub fn _syscall_handler();
}

// System call handler: call the appropriate system call according to the nb argument.
// Called by the assembly code _syscall_handler
#[no_mangle]
pub unsafe extern fn syscall_handler(nb: Syscall, _arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32, caller_tss_selector: u32) -> i32 {
    let idx = selector_to_gdt_index(caller_tss_selector) as usize - GDT_SIZE;
    let addr = TASKS[idx].addr_space;
    match nb {
        Syscall::Puts => syscall_puts(addr + _arg1),
        Syscall::Exec => syscall_exec(addr + _arg1),
        Syscall::Keypressed => syscall_keypressed(),
        Syscall::Getc => syscall_getc(),
        Syscall::FileStat => syscall_file_stat(addr + _arg1, addr + _arg2),
        Syscall::FileOpen => syscall_file_open(addr + _arg1),
        Syscall::FileClose => syscall_file_close(_arg1),
        Syscall::FileRead => syscall_file_read(_arg1, addr + _arg2, _arg3),
        Syscall::FileSeek => syscall_file_seek(_arg1, _arg2),
        Syscall::FileIterator => syscall_file_iterator(addr + _arg1),
        Syscall::FileNext => syscall_file_next(addr + _arg1, addr + _arg2),
        Syscall::GetTicks => syscall_get_ticks(),
        Syscall::Sleep => syscall_sleep(_arg1)
    }
}

unsafe fn syscall_puts(fmt_addr: u32) -> i32 {
    let bytes = fmt_addr as * const [u8; ADDR_SPACE_SIZE];
    SCREEN.write_str(bytes_to_str(&*bytes));
    return 0;
}

unsafe fn syscall_exec(filename_addr: u32) -> i32 {
    let bytes = filename_addr as * const [u8; ADDR_SPACE_SIZE];
    exec(bytes_to_str(&*bytes)) as i32
}

unsafe fn syscall_keypressed() -> i32 {
    keypressed() as i32
}

unsafe fn syscall_getc() -> i32 {
    getc() as i32
}

unsafe fn syscall_file_stat(filename_addr: u32, stat_addr: u32) -> i32 {
    let bytes = filename_addr as * const [u8; ADDR_SPACE_SIZE];
    let stat = stat_addr as *mut Stat;
	*stat = Stat::new(bytes_to_str(&*bytes));
    if (*stat).start == 0 {
        return -1;
    }
    return 0;
}

unsafe fn syscall_file_open(filename_addr: u32) -> i32 {
    let bytes = filename_addr as * const [u8; ADDR_SPACE_SIZE];
    file_open(bytes_to_str(&*bytes)) as i32
}

unsafe fn syscall_file_close(fd: u32) -> i32 {
    file_close(fd as i8) as i32
}

unsafe fn syscall_file_read(fd: u32, buf_addr: u32, n: u32) -> i32 {
    file_read(fd as i8, buf_addr as *mut u8, n as usize) as i32
}

unsafe fn syscall_file_seek(fd: u32, offset: u32) -> i32 {
    file_seek(fd as i8, offset as usize) as i32
}

unsafe fn syscall_file_iterator(it_addr: u32) -> i32 {
    let it = it_addr as *mut FileIterator;
    *it = FileIterator::new();
    return 0;
}

unsafe fn syscall_file_next(filename_addr: u32, it_addr: u32) -> i32 {
    let it = it_addr as *mut FileIterator;
    (*it).next(filename_addr as *mut u8) as i32
}

unsafe fn syscall_get_ticks() -> i32 {
    get_ticks() as i32
}

unsafe fn syscall_sleep(ms: u32) -> i32 {
    sleep(ms);
    return 0;
}