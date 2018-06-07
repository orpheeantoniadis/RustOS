#![allow(dead_code)]

use core::mem::size_of;
use core::ops::{Index, IndexMut};
use rlibc::memset;
use vga::*;

const KERNEL_BASE: u32 = 0xC0000000;
const KERNEL_PAGE_NUMBER: u32 = KERNEL_BASE >> 22;

const MAX_ENTRIES: usize = 1024;
const MMAP_SIZE: usize = 0x20000;
const FRAMES_NB: usize = 0x100000;
pub const FRAME_SIZE: usize = 0x1000;

static mut KHEAP_ADDR: u32 = 0;
pub static mut PD_ADDR: u32 = 0;
pub static mut MMAP: [u8;MMAP_SIZE] = [0;MMAP_SIZE];

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u32;MAX_ENTRIES]
}

extern "C" {
    fn load_directory(page_directory_ptr: *const PageTable);
    fn get_kernel_start() -> u32;
    fn get_kernel_end() -> u32;
    fn get_kernel_page_directory() -> u32;
    fn get_kernel_page_table() -> u32;
}

pub fn paging_init() {
    unsafe {
        KHEAP_ADDR = get_kernel_end();
        PD_ADDR = get_kernel_page_directory();
        set_mmap_area(KERNEL_BASE, KHEAP_ADDR);
    }
}

pub fn kmalloc(size: usize) -> u32 {
    unsafe {
        if (KHEAP_ADDR & 0xfffff000) != KHEAP_ADDR {
            KHEAP_ADDR &= 0xfffff000;
            KHEAP_ADDR += 0x1000;
        }
        let addr = KHEAP_ADDR;
        KHEAP_ADDR += size as u32;
        return addr;
    }
}

pub fn alloc_frame() -> u32 {
    let frame = get_free_frame();
    if frame != 0 {
        set_mmap_frame(frame)
    }
    return frame * FRAME_SIZE as u32;
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

impl Index<usize> for PageTable {
    type Output = u32;

    fn index(&self, index: usize) -> &u32 {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut u32 {
        &mut self.entries[index]
    }
}

impl PageTable {
    fn new() -> PageTable {
        PageTable {
            entries: [0;MAX_ENTRIES]
        }
    }
    
    pub fn from_ptr(addr: u32) -> *mut PageTable {
        addr as *mut PageTable
    }
    
    pub fn as_ptr(&mut self) -> u32 {
        self as *const PageTable as u32
    }
    
    pub fn set_page(&mut self, idx: u32) {
        unsafe {
            let table_idx = idx as usize / MAX_ENTRIES;
            let entry_idx = idx as usize % MAX_ENTRIES;
            if self[table_idx] == 0 {
                // create a new table on the heap
                let table_addr = kmalloc(FRAME_SIZE);
                let table_frame_addr = alloc_frame();
                let table_frame_idx = table_frame_addr / FRAME_SIZE as u32;
                // create a temporary table
                let mut tmp_table = PageTable::new();
                tmp_table[table_frame_idx as usize % MAX_ENTRIES] = table_addr | 0x3;
                self[769] = tmp_table.as_ptr() | 0x3;
                // now that the new table is mapped we can modify it
                println!("Table idx = 0x{:x}", table_idx);
                println!("Tmp table addr = 0x{:x}", tmp_table.as_ptr());
                println!("New table phys addr = 0x{:x}", table_addr);
                println!("New table addr = 0x{:x}", table_frame_addr);
                println!("New table frame idx = 0x{:x}", table_frame_idx);
                println!("Directory entry = 0x{:x}", self[table_idx]);
                println!("Table entry = 0x{:x}", (*PageTable::from_ptr(self[table_idx]&!3))[2]);
                memset(0xC0402000 as *mut u8, 0, size_of::<PageTable>());
                // (*(table_addr as *mut PageTable))[entry_idx] = (idx * FRAME_SIZE as u32) | 0x3;
            } else {
                let table_addr = self[table_idx] &! 0x3;
                (*(table_addr as *mut PageTable))[entry_idx] = (idx * FRAME_SIZE as u32) | 0x3;
            }
        }
    }
}