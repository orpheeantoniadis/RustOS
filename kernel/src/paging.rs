#![allow(dead_code)]
#![macro_use]

use core::mem::size_of;
use core::ops::{Index, IndexMut};
use rlibc::memset;
use vga::*;
use kheap::*;

pub const KERNEL_BASE: u32 = 0xC0000000;
pub const KERNEL_PAGE_NUMBER: u32 = KERNEL_BASE >> 22;

const MMAP_SIZE: usize = 0x20000;
const MEMORY_FSIZE: usize = 0x100000;
pub const TABLE_FSIZE: usize = 0x400;
pub const TABLE_SIZE: usize = 0x400000;
pub const FRAME_SIZE: usize = 0x1000;

pub const KERNEL_MODE: u32 = 0x0;
pub const USER_MODE: u32 = 0x4;

static mut INITIAL_MMAP: [u8;MMAP_SIZE] = [0;MMAP_SIZE];
pub static mut INITIAL_PD: PageDirectory = PageDirectory::null();
pub static mut USER_PD: *mut PageDirectory = 0 as *mut PageDirectory;

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    pub tables: *mut PageTable,
    pub mmap: *mut [u8;MMAP_SIZE]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [u32;TABLE_FSIZE]
}

extern "C" {
    pub fn load_directory(pd_addr: u32);
    pub fn get_cr3() -> u32;
    pub fn get_kernel_start() -> u32;
    pub fn get_kernel_end() -> u32;
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
        INITIAL_PD.tables = get_kernel_page_directory() as *mut PageTable;
        INITIAL_PD.mmap = &mut INITIAL_MMAP as *mut [u8;MMAP_SIZE];
        INITIAL_PD.mmap_set_area(KERNEL_BASE, KHEAP_ADDR);
    }
}

pub fn switch_directory(pd_ptr: *mut PageDirectory) {
    unsafe {
        if pd_ptr as u32 != &INITIAL_PD as *const _ as u32 {
            USER_PD = pd_ptr;
            (*USER_PD).update();
        }
        load_directory(phys!((*pd_ptr).tables as u32));
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
    
    pub fn alloc_frame(&mut self, virt: *mut u32, phys: *mut u32, mode: u32) -> i32 {
        unsafe {
            if *phys < phys!(get_kernel_end()) {
                println!("alloc_frame: corrupted address");
                return -1;
            }
            *virt = if mode == USER_MODE {
                self.mmap_alloc_frame(0)
            } else {
                self.mmap_alloc_frame(virt!(*phys))
            };
            let frame_idx = *virt / FRAME_SIZE as u32;
            let table_idx = frame_idx as usize / TABLE_FSIZE;
            let entry_idx = frame_idx as usize % TABLE_FSIZE;
            
            let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
            (*table_ptr)[entry_idx] = *phys | 0x3 | mode;
            memset(*virt as *mut u8, 0, FRAME_SIZE);
            return 0;
        }
    }
    
    pub fn free_frame(&mut self, addr: u32) {
        unsafe {
            if addr < get_kernel_end() {
                println!("free_frame: corrupted address");
                return;
            }
            let frame_idx = addr / FRAME_SIZE as u32;
            let table_idx = frame_idx as usize / TABLE_FSIZE;
            let entry_idx = frame_idx as usize % TABLE_FSIZE;
            self.mmap_reset_frame(frame_idx);
            let table_ptr = virt!(self[table_idx] &! 0xfff) as *mut PageTable;
            (*table_ptr)[entry_idx] = 0;
        }    
    }
    
    pub fn new_directory(&mut self) -> PageDirectory {
        PageDirectory {
            tables: (kmalloc(FRAME_SIZE) + (FRAME_SIZE - size_of::<Header>()) as u32) as *mut PageTable,
            mmap: kmalloc(MMAP_SIZE) as *mut [u8;MMAP_SIZE]
        }
    }
    
    pub fn free(&mut self) {
        kfree(self.tables as u32 - (FRAME_SIZE - size_of::<Header>()) as u32);
        kfree(self.mmap as u32);
    }
    
    pub fn update(&mut self) {
        for i in KERNEL_PAGE_NUMBER as usize..TABLE_FSIZE {
            if self[i] == 0 {
                self[i] = unsafe { INITIAL_PD[i] };
            }
        }
    }
    
    pub fn new_table(&mut self, addr: u32) -> u32 {
        unsafe {
            let phys_addr = phys!(addr);
            let virt_addr = virt!(phys_addr);
            let frame_idx = virt_addr / FRAME_SIZE as u32;
            let table_idx = frame_idx as usize / TABLE_FSIZE;
            let entry_idx = frame_idx as usize % TABLE_FSIZE;
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
                memset(virt_addr as *mut u8, 0, size_of::<PageTable>());
            }
            return virt_addr;
        }
    }

    pub fn mmap_alloc_frame(&mut self, addr: u32) -> u32 {
        let frame = if addr == 0 {
            self.mmap_get_free_frame()
        } else {
            addr / FRAME_SIZE as u32
        };
        self.mmap_set_frame(frame);
        return frame * FRAME_SIZE as u32;
    }

    pub fn mmap_get_free_frame(&mut self) -> u32 {
        for i in 0..MEMORY_FSIZE {
            if self.mmap_frame_state(i as u32) == 0 {
                return i as u32;
            }
        }
        return 0;
    }
    
    pub fn mmap_get_free_area(&mut self, size: usize) -> u32 {
        let mut cnt = 0;
        let mut start_frame = 0;
        for i in 0..MEMORY_FSIZE {
            if self.mmap_frame_state(i as u32) == 0 {
                if cnt == 0 {
                    start_frame = i as u32;
                }
                cnt += 1;
            } else {
                cnt = 0;
            }
            if cnt == size / FRAME_SIZE {
                break;
            }
        }
        return start_frame;
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
            entries: [0;TABLE_FSIZE]
        }
    }
    
    pub fn from_ptr(addr: u32) -> *mut PageTable {
        addr as *mut PageTable
    }
    
    pub fn as_ptr(&mut self) -> u32 {
        self as *const PageTable as u32
    }
}