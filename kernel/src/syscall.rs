#![allow(dead_code)]

use vga::*;
use pio::*;
use timer::*;
use keyboard::*;
use fs::*;
use task::*;
use paging::FRAME_SIZE;
use kheap::*;
use common::*;

extern "C" {
    pub fn _syscall_handler();
}

/// System call handler: call the appropriate system call according to the nb argument.
/// Called by the assembly code _syscall_handler
#[no_mangle]
pub unsafe extern fn syscall_handler(nb: Syscall, _arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32) -> i32 {
    let addr = 0;
    match nb {
        Syscall::Puts => syscall_puts(addr, _arg1),
        Syscall::Putc => syscall_putc(_arg1),
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
        Syscall::Sleep => syscall_sleep(_arg1),
        Syscall::SetCursor => syscall_set_cursor(_arg1, _arg2),
        Syscall::GetCursor => syscall_get_cursor(addr + _arg1, addr + _arg2),
        Syscall::CursorDisable => syscall_cursor_disable(_arg1),
        Syscall::CopyScr => syscall_copy_scr(addr + _arg1),
        Syscall::AllocFrame => syscall_alloc_frame(),
        Syscall::FreeFrame => syscall_free_frame(_arg1),
    }
}

unsafe fn syscall_puts(base_addr: u32, string_offset: u32) -> i32 {
    let mut string = *((base_addr + string_offset) as *mut String);
    string.offset(base_addr);
    vga_write_str(string.to_string());
    return 0;
}

unsafe fn syscall_putc(c: u32) -> i32 {
    vga_write_byte(c as u8);
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

unsafe fn syscall_set_cursor(x: u32, y: u32) -> i32 {
    vga_set_cursor(x as usize, y as usize);
    return 0;
}

unsafe fn syscall_get_cursor(x_addr: u32, y_addr: u32) -> i32 {
    let cursor = vga_get_cursor();
    *(x_addr as *mut u32) = cursor.0 as u32;
    *(y_addr as *mut u32) = cursor.1 as u32;
    return 0;
}

unsafe fn syscall_cursor_disable(cd: u32) -> i32 {
    if cd == 0 {
        enable_cursor();
    } else {
        disable_cursor();
    }
    return 0;
}

unsafe fn syscall_copy_scr(scr_addr: u32) -> i32 {
    vga_copy_scr(scr_addr as *const FrameBuffer);
    return 0;
}

unsafe fn syscall_alloc_frame() -> i32 {
    umalloc(FRAME_SIZE) as i32
}

unsafe fn syscall_free_frame(addr: u32) -> i32 {
    ufree(addr);
    return 0;
}