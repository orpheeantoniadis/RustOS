#![allow(dead_code)]

use core::mem::size_of;
use vga::*;
use multiboot::*;

const MAX_ENTRIES: usize = 1024;
const MMAP_SIZE: usize = 0x20000;
const FRAMES_NB: usize = 0x100000;

pub static mut INITIAL_PAGE_DIRECTORY: PageDirectory = PageDirectory::new();
pub static mut INITIAL_PAGE_TABLE: PageTable = PageTable::new();
pub static mut MMAP: [u8;MMAP_SIZE] = [0;MMAP_SIZE];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PageEntry(u32);

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageEntry;MAX_ENTRIES]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    tables: [PageEntry;MAX_ENTRIES]
}

extern "C" {
    fn enable_paging(page_directory_ptr: *const PageDirectory);
    fn get_kernel_start() -> u32;
    fn get_kernel_end() -> u32;
}

pub fn paging_init(multiboot_infos: *mut MultibootInfo) {
    unsafe {
        mmap_init();
        INITIAL_PAGE_TABLE.set();
        INITIAL_PAGE_DIRECTORY.set_table(0, &INITIAL_PAGE_TABLE);
        // enable_paging(&INITIAL_PAGE_DIRECTORY);
    }
}

fn mmap_init() {
    // Reserved section set to used
    for i in 0..0x100 {
        set_mmap_frame(i);
    }
    let kernel_start_frame = get_kernel_start() / 0x1000;
    let kernel_end_frame = get_kernel_end() / 0x1000;
    for i in kernel_start_frame..kernel_end_frame {
        set_mmap_frame(i);
    }
    let mboot_start_frame = (multiboot_infos as u32) / 0x1000;
    let mboot_end_frame = (multiboot_infos as u32 + size_of::<MultibootInfo>() as u32) / 0x1000;
    for i in mboot_start_frame..mboot_end_frame {
        set_mmap_frame(i);
    }
    let mmap_start_frame = (&MMAP as *const [u8;MMAP_SIZE] as u32) / 0x1000;
    let mmap_end_frame = ((&MMAP as *const [u8;MMAP_SIZE] as u32) + MMAP_SIZE as u32) / 0x1000;
    for i in mmap_start_frame..mmap_end_frame {
        set_mmap_frame(i);
    }
}

fn kalloc(size: usize) {
    
}

fn get_free_frame() -> u32 {
    for i in 0..FRAMES_NB {
        if mmap_frame_state(i) == 1 {
            return i;
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

impl PageEntry {
    const fn null() -> PageEntry {
        PageEntry(0x2) // Read/Write
    }
    
    const fn new() -> PageEntry {
        PageEntry(0x3) // Present + Read/Write
    }
}

impl PageTable {
    const fn new() -> PageTable {
        PageTable { entries: [PageEntry::new();MAX_ENTRIES] }
    }
    
    fn set(&mut self) {
        for i in 0..MAX_ENTRIES {
            self.entries[i] = PageEntry((i as u32 * 0x1000) | 0x3);
        }
    }
}

impl PageDirectory {
    const fn new() -> PageDirectory {
        PageDirectory { tables: [PageEntry::null();MAX_ENTRIES] }
    }
    
    fn set_table(&mut self, index: usize, table: *const PageTable) {
        self.tables[index] = PageEntry(table as u32 | 0x3);
    }
}