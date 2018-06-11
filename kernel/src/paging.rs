#![allow(dead_code)]
#![macro_use]

use core::mem::size_of;
use core::ops::{Index, IndexMut};
use rlibc::memset;
// use vga::*;

pub const KERNEL_BASE: u32 = 0xC0000000;
pub const KERNEL_PAGE_NUMBER: u32 = KERNEL_BASE >> 22;

const MAX_ENTRIES: usize = 1024;
const MMAP_SIZE: usize = 0x20000;
const FRAMES_NB: usize = 0x100000;
pub const FRAME_SIZE: usize = 0x1000;

pub const KERNEL_MODE: u32 = 0x0;
pub const USER_MODE: u32 = 0x4;

static mut KHEAP_ADDR: u32 = 0;
pub static mut INIT_PD: PageDirectory = PageDirectory::null();

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    pub tables: *mut PageTable,
    pub mmap: [u8;MMAP_SIZE]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u32;MAX_ENTRIES]
}

extern "C" {
    pub fn load_directory(pd_addr: u32);
    fn get_kernel_start() -> u32;
    fn get_kernel_end() -> u32;
    fn get_kernel_page_directory() -> u32;
    fn get_kernel_page_table() -> u32;
}

macro_rules! phys {
    ($addr:expr) => ($addr - KERNEL_BASE);
}

macro_rules! virt {
    ($addr:expr) => ($addr + KERNEL_BASE);
}

pub fn paging_init() {
    unsafe {
        KHEAP_ADDR = get_kernel_end();
        INIT_PD.tables = get_kernel_page_directory() as *mut PageTable;
        INIT_PD.mmap_set_area(KERNEL_BASE, KHEAP_ADDR);
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

impl Index<usize> for PageDirectory {
    type Output = u32;

    fn index(&self, index: usize) -> &u32 {
        unsafe { &(*self.tables)[index] }
    }
}

impl IndexMut<usize> for PageDirectory {
    fn index_mut(&mut self, index: usize) -> &mut u32 {
        unsafe { &mut (*self.tables)[index] }
    }
}

impl PageDirectory {
    pub const fn null() -> PageDirectory {
        PageDirectory {
            tables: 0 as *mut PageTable,
            mmap: [0;MMAP_SIZE]
        }
    }
    
    pub fn new(tables_addr: u32) -> PageDirectory {
        PageDirectory {
            tables: tables_addr as *mut PageTable,
            mmap: [0;MMAP_SIZE]
        }
    }
    
    pub fn alloc_frame(&mut self, mode: u32) -> u32 {
        let frame = self.mmap_alloc_frame(0);
        self.set_page(frame / FRAME_SIZE as u32, mode);
        return frame;
    }
    
    pub fn set_page(&mut self, idx: u32, mode: u32) {
        unsafe {
            let table_idx = idx as usize / MAX_ENTRIES;
            let entry_idx = idx as usize % MAX_ENTRIES;
            let entry_addr = phys!(kmalloc(FRAME_SIZE));
            if self[table_idx] == 0 {
                self[table_idx] = phys!(self.new_table(mode)) | 0x3 | mode;
            }
            let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
            (*table_ptr)[entry_idx] = entry_addr | 0x3 | mode;
        }
    }
    
    pub fn new_table(&mut self, mode: u32) -> u32 {
        unsafe {
            // create a new table on the heap
            let phys_addr = phys!(kmalloc(FRAME_SIZE));
            let virt_addr = self.mmap_alloc_frame(virt!(phys_addr));
            let frame_idx = virt_addr / FRAME_SIZE as u32;
            let table_idx = frame_idx as usize / MAX_ENTRIES;
            let entry_idx = frame_idx as usize % MAX_ENTRIES;
            if self[table_idx] == 0 {
                // create a temporary table
                let mut tmp_table = PageTable::null();
                tmp_table[entry_idx] = phys_addr | 0x3 | mode;
                self[table_idx] = phys!(tmp_table.as_ptr()) | 0x3 | mode;
                // now that the new table is mapped we can modify it
                let table_ptr = virt_addr as *mut PageTable;
                memset(virt_addr as *mut u8, 0, size_of::<PageTable>());
                (*table_ptr)[entry_idx] = phys_addr | 0x3 | mode;
                self[table_idx] = phys_addr | 0x3 | mode;
            } else {
                // page table already exists, just create new entry
                let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
                (*table_ptr)[entry_idx] = phys_addr | 0x3 | mode;
                memset(virt_addr as *mut u8, 0, size_of::<PageTable>());
            }
            return virt_addr;
        }
    }
    
    fn mmap_alloc_frame(&mut self, addr: u32) -> u32 {
        let frame = if addr == 0 {
            self.mmap_get_free_frame()
        } else {
            addr / FRAME_SIZE as u32
        };
        self.mmap_set_frame(frame);
        return frame * FRAME_SIZE as u32;
    }

    fn mmap_get_free_frame(&mut self) -> u32 {
        for i in 0..FRAMES_NB {
            if self.mmap_frame_state(i as u32) == 0 {
                return i as u32;
            }
        }
        return 0;
    }

    fn mmap_set_frame(&mut self, frame_id: u32) {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        self.mmap[mmap_id as usize] |= 1<<bit_offset;
    }

    fn mmap_set_area(&mut self, start: u32, end: u32) {
        let start_frame = start / FRAME_SIZE as u32;
        let mut end_frame = end / FRAME_SIZE as u32;
        if end % 0x1000 != 0 {
            end_frame += 1;
        }
        for i in start_frame..end_frame {
            self.mmap_set_frame(i);
        }
    }

    fn mmap_reset_frame(&mut self, frame_id: u32) {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        self.mmap[mmap_id as usize] &= !(1<<bit_offset);
    }

    fn mmap_frame_state(&mut self, frame_id: u32) -> u8 {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        (self.mmap[mmap_id as usize] >> bit_offset) as u8 & 1
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
    fn null() -> PageTable {
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
}