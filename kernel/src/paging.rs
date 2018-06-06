#![allow(dead_code)]

use core::mem::size_of;
use rlibc::memset;
// use vga::*;

const KERNEL_BASE: u32 = 0xC0000000;
const MAX_ENTRIES: usize = 1024;
const MMAP_SIZE: usize = 0x20000;
const FRAMES_NB: usize = 0x100000;
const FRAME_SIZE: usize = 0x1000;

pub static mut MMAP: [u8;MMAP_SIZE] = [0;MMAP_SIZE];

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u32;MAX_ENTRIES]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    tables: [u32;MAX_ENTRIES]
}

extern "C" {
    fn load_directory(page_directory_ptr: *const PageDirectory);
    fn get_kernel_start() -> u32;
    fn get_kernel_end() -> u32;
    fn get_kernel_page_directory() -> u32;
}

pub fn mmap_init() {
    unsafe {
        set_mmap_area(KERNEL_BASE, get_kernel_end());
    }
}

fn alloc_frame() -> u32 {
    let addr = get_free_frame();
    if addr != 0 {
        set_mmap_frame(addr)
    }
    return addr * FRAME_SIZE as u32;
}

fn get_free_frame() -> u32 {
    // keep the first frame free
    for i in 1..FRAMES_NB {
        if mmap_frame_state(i as u32) == 0 {
            return i as u32;
        }
    }
    return 0;
}

fn set_mmap_frame(frame_id: u32) {
    unsafe {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        MMAP[mmap_id as usize] |= 1<<bit_offset;
    }
}

fn set_mmap_area(start: u32, end: u32) {
    let start_frame = start / FRAME_SIZE as u32;
    let mut end_frame = end / FRAME_SIZE as u32;
    if end % 0x1000 != 0 {
        end_frame += 1;
    }
    for i in start_frame..end_frame {
        set_mmap_frame(i);
    }
}

fn reset_mmap_frame(frame_id: u32) {
    unsafe {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        MMAP[mmap_id as usize] &= !(1<<bit_offset);
    }
}

fn mmap_frame_state(frame_id: u32) -> u8 {
    unsafe {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        (MMAP[mmap_id as usize] >> bit_offset) as u8 & 1
    }
}

impl PageDirectory {    
    fn set_page(&mut self, idx: u32) {
        unsafe {
            let table_idx = idx as usize / MAX_ENTRIES;
            let entry_idx = idx as usize % MAX_ENTRIES;
            if self.tables[table_idx] == 0 {
                let table_addr = alloc_frame();
                memset(table_addr as *mut u8, 0, size_of::<PageTable>());
                (*(table_addr as *mut PageTable)).entries[entry_idx] = (idx * FRAME_SIZE as u32) | 0x3;
                self.tables[table_idx] = table_addr | 0x3;
            } else {
                let table_addr = self.tables[table_idx] &! 0x3;
                (*(table_addr as *mut PageTable)).entries[entry_idx] = (idx * FRAME_SIZE as u32) | 0x3;
            }
        }
    }
}