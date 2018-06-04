#![allow(dead_code)]

use core::mem::size_of;
use rlibc::memset;
use vga::*;
use multiboot::*;

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
    fn enable_paging(page_directory_ptr: *const PageDirectory);
    fn get_kernel_start() -> u32;
    fn get_kernel_end() -> u32;
}

pub fn paging_init(multiboot_infos: *mut MultibootInfo) {
    unsafe {
        mmap_init(multiboot_infos);
        let dir_addr = alloc_frame();
        let mut initial_page_directory = *(dir_addr as *const PageDirectory);
        memset(dir_addr as *mut u8, 0, size_of::<PageDirectory>());
        
        for i in 0..0x100 {
            initial_page_directory.set_page(i);
        }
        
        let kernel_start_frame = get_kernel_start() / FRAME_SIZE as u32;
        let mut kernel_end_frame = get_kernel_end() / FRAME_SIZE as u32;
        if kernel_end_frame % 0x1000 != 0 {
            kernel_end_frame += 1;
        }
        for i in kernel_start_frame..kernel_end_frame {
            initial_page_directory.set_page(i);
        }
        
        enable_paging(&initial_page_directory);
    }
}

fn mmap_init(multiboot_infos: *mut MultibootInfo) {
    unsafe {
        // Reserved section set to used
        for i in 0..0x100 {
            set_mmap_frame(i);
        }
        
        set_mmap_area(get_kernel_start(), get_kernel_end());
        set_mmap_area(multiboot_infos as u32, multiboot_infos as u32 + size_of::<MultibootInfo>() as u32);
    }
}

fn alloc_frame() -> u32 {
    let addr = get_free_frame();
    if addr != 0 {
        set_mmap_frame(addr)
    }
    return addr * FRAME_SIZE as u32;
}

fn alloc_area(size: usize) -> u32 {
    let start_addr = get_free_area(size) * FRAME_SIZE as u32;
    if start_addr != 0 {
        set_mmap_area(start_addr, start_addr + (size * FRAME_SIZE) as u32)
    }
    return start_addr;
}

fn get_free_frame() -> u32 {
    for i in 0..FRAMES_NB {
        if mmap_frame_state(i as u32) == 0 {
            return i as u32;
        }
    }
    return 0;
}

fn get_free_area(size: usize) -> u32 {
    let mut cnt = 0;
    let mut start_frame = 0;
    for i in 0..FRAMES_NB {
        if mmap_frame_state(i as u32) == 0 {
            if cnt == 0 {
                start_frame = i;
            }
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt == size {
            break;
        }
    }
    return start_frame as u32;
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