#![allow(dead_code)]
#![macro_use]

use core::mem::size_of;
use core::ops::{Index, IndexMut};
use rlibc::memset;

pub const KERNEL_BASE: u32 = 0xC0000000;
pub const KERNEL_PAGE_NUMBER: u32 = KERNEL_BASE >> 22;

const MAX_ENTRIES: usize = 1024;
const MMAP_SIZE: usize = 0x20000;
const FRAMES_NB: usize = 0x100000;
pub const FRAME_SIZE: usize = 0x1000;

pub const KERNEL_MODE: u32 = 0x0;
pub const USER_MODE: u32 = 0x4;

pub static mut KHEAP_ADDR: u32 = 0;
static mut INITIAL_MMAP: [u8;MMAP_SIZE] = [0;MMAP_SIZE];
pub static mut INITIAL_PD: PageDirectory = PageDirectory::null();
pub static mut CURRENT_PD: *mut PageDirectory = 0 as *mut PageDirectory;

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    pub tables: *mut PageTable,
    pub mmap: *mut [u8;MMAP_SIZE]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u32;MAX_ENTRIES]
}

extern "C" {
    pub fn load_directory(pd_addr: u32);
    pub fn get_cr3() -> u32;
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
        CURRENT_PD = &mut INITIAL_PD;
        INITIAL_PD.tables = get_kernel_page_directory() as *mut PageTable;
        INITIAL_PD.mmap = &mut INITIAL_MMAP as *mut [u8;MMAP_SIZE];
        INITIAL_PD.mmap_set_area(KERNEL_BASE, KHEAP_ADDR);
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

pub fn kfree(size: usize) {
    unsafe {
        if (KHEAP_ADDR & 0xfffff000) != KHEAP_ADDR {
            KHEAP_ADDR &= 0xfffff000;
        }
        KHEAP_ADDR -= size as u32;
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
            mmap: 0 as *mut [u8;MMAP_SIZE]
        }
    }
    
    pub fn new() -> PageDirectory {
        unsafe {
            PageDirectory {
                tables: INITIAL_PD.new_table() as *mut PageTable,
                mmap: INITIAL_PD.new_mmap() as *mut [u8;MMAP_SIZE]
            }
        }
    }
    
    pub fn alloc_frame(&mut self, mode: u32) -> u32 {
        let phys_addr = phys!(kmalloc(FRAME_SIZE));
        let virt_addr = if mode == USER_MODE {
            self.mmap_alloc_frame(0)
        } else {
            self.mmap_alloc_frame(virt!(phys_addr))
        };
        let frame_idx = virt_addr / FRAME_SIZE as u32;
        let table_idx = frame_idx as usize / MAX_ENTRIES;
        let entry_idx = frame_idx as usize % MAX_ENTRIES;
        
        if self[table_idx] == 0 {
            self[table_idx] = phys!(self.new_table()) | 0x3 | mode;
        }
        let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
        unsafe { 
            (*table_ptr)[entry_idx] = phys_addr | 0x3 | mode;
            memset(virt_addr as *mut u8, 0, FRAME_SIZE);
        }
        return virt_addr;
    }
    
    pub fn new_table(&mut self) -> u32 {
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
                tmp_table[entry_idx] = phys_addr | 0x3;
                self[table_idx] = phys!(tmp_table.as_ptr()) | 0x3;
                // now that the new table is mapped we can modify it
                let table_ptr = virt_addr as *mut PageTable;
                memset(virt_addr as *mut u8, 0, size_of::<PageTable>());
                (*table_ptr)[entry_idx] = phys_addr | 0x3;
                self[table_idx] = phys_addr | 0x3;
            } else {
                // page table already exists, just create new entry
                let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
                (*table_ptr)[entry_idx] = phys_addr | 0x3;
                memset(virt_addr as *mut u8, 0, size_of::<PageTable>());
            }
            return virt_addr;
        }
    }
    
    pub fn new_mmap(&mut self) -> u32 {
        unsafe {
            // check if the mmap fits in the current page table (the mmap must be contiguous)
            if (KHEAP_ADDR % 0x400000 + MMAP_SIZE as u32) > 0x400000 {
                let table_idx = (KHEAP_ADDR as usize / FRAME_SIZE) / MAX_ENTRIES;
                self[table_idx + 1] = phys!(self.new_table()) | 0x3;
            }
            let mmap_addr = self.alloc_frame(KERNEL_MODE);
            for _i in 0..(MMAP_SIZE / FRAME_SIZE - 1) {
                self.alloc_frame(KERNEL_MODE);
            }
            memset(mmap_addr as *mut u8, 0, MMAP_SIZE);
            return mmap_addr;
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
        unsafe { (*self.mmap)[mmap_id as usize] |= 1<<bit_offset; }
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
        unsafe { (*self.mmap)[mmap_id as usize] &= !(1<<bit_offset); }
    }

    fn mmap_frame_state(&mut self, frame_id: u32) -> u8 {
        let mmap_id = frame_id / 8;
        let bit_offset = frame_id % 8;
        unsafe { ((*self.mmap)[mmap_id as usize] >> bit_offset) as u8 & 1 }
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