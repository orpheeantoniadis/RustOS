#![allow(dead_code)]

use core::mem::size_of;
use rlibc::{memset,memcpy};
use paging::*;
use vga::*;

const HEAP_SIZE: usize = KHEAP_SIZE;

static mut HEAP_START: u32 = 0;
static mut HEAP_END: u32 = 0;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
struct Header {
    previous: u32,
    next: u32,
    size: usize,
    free: bool
}

pub fn heap_init() {
    unsafe {
        HEAP_START = phys!(KHEAP_ADDR) + KHEAP_SIZE as u32;
        HEAP_START = (HEAP_START & 0xfffff000) + 0x1000;
        HEAP_END = HEAP_START + HEAP_SIZE as u32;
        
        for _i in page!(HEAP_START)..page!(HEAP_END) {
            // INITIAL_PD[i as usize] = phys!(INITIAL_PD.new_table()) | 0x3 | USER_MODE;
        }
        let entry_addr = INITIAL_PD.mmap_alloc_frame(HEAP_START);
        let frame_idx = entry_addr / FRAME_SIZE as u32;
        let table_idx = frame_idx as usize / TABLE_SIZE;
        let entry_idx = frame_idx as usize % TABLE_SIZE;
        let table_ptr = virt!(INITIAL_PD[table_idx] &! 0xfff) as *mut PageTable;
        (*table_ptr)[entry_idx] = entry_addr | 0x3 | USER_MODE;
        memset(entry_addr as *mut u8, 0, FRAME_SIZE);
        memcpy(entry_addr as *mut u8, Header::null(0, HEAP_SIZE).as_ptr(), size_of::<Header>());
    }
}

pub fn malloc(size: usize) -> u32 {
    unsafe {
        // align size at 0x10 if not already aligned
        let aligned_size = if (size & 0xfffff000) != size {
            (size & 0xfffffff0) + 0x10
        } else {
            size
        };
        // finding an empty block
        let mut addr = HEAP_START;
        let mut block = Header::from_ptr(addr as *mut u8);
        while block.next != 0 {
            if block.size >= aligned_size && block.free {
                break;
            }
            addr = block.next;
            block = Header::from_ptr(addr as *mut u8);
        }
        // if no block found return 0
        if block.size >= aligned_size && block.free {
            block.size = aligned_size;
            block.free = false;
            // if the block is the last (at the tail), creating new tail
            if block.next == 0 {
                block.next = addr + (size_of::<Header>() + aligned_size) as u32;
                // alloc a new frame if need
                if (addr as usize % FRAME_SIZE) + size_of::<Header>() + aligned_size > FRAME_SIZE {
                    let entry_addr = INITIAL_PD.mmap_alloc_frame(block.next);
                    let frame_idx = entry_addr / FRAME_SIZE as u32;
                    let table_idx = frame_idx as usize / TABLE_SIZE;
                    let entry_idx = frame_idx as usize % TABLE_SIZE;
                    let table_ptr = virt!(INITIAL_PD[table_idx] &! 0xfff) as *mut PageTable;
                    (*table_ptr)[entry_idx] = entry_addr | 0x3 | USER_MODE;
                    memset(entry_addr as *mut u8, 0, FRAME_SIZE);
                }
                let next_block_size = (HEAP_END - block.next) as usize - size_of::<Header>();
                let mut next_header = Header::null(addr, next_block_size);
                memcpy(addr as *mut u8, block.as_ptr(), size_of::<Header>());
                memcpy(block.next as *mut u8, next_header.as_ptr(), size_of::<Header>());
            } else {
                // if the block is not the tail, need to save the next block
                // before creating a new block at block.next
                let tmp = block.next;
                let mut tmp_header = Header::from_ptr(tmp as *const u8);
                
                block.next = addr + (size_of::<Header>() + aligned_size) as u32;
                let next_block_size = (tmp - block.next) as usize - size_of::<Header>();
                let mut next_header = Header::null(addr, next_block_size);
                next_header.next = tmp;
                tmp_header.previous = block.next;
                
                memcpy(addr as *mut u8, block.as_ptr(), size_of::<Header>());
                memcpy(block.next as *mut u8, next_header.as_ptr(), size_of::<Header>());
                memcpy(tmp as *mut u8, tmp_header.as_ptr(), size_of::<Header>());
            }
            return addr + size_of::<Header>() as u32;
        }
        return 0;
    }
}

pub fn free(addr: u32) {
    let mut header_addr = addr - size_of::<Header>() as u32;
    let mut header = Header::from_ptr(header_addr as *const u8);
    if !header.free {
        if header.previous != 0 {
            let previous = Header::from_ptr(header.previous as *const u8);
            if previous.free {
                header_addr = header.previous;
                header.previous = previous.previous;
                header.size += previous.size + size_of::<Header>();
            }
        }
        if header.next != 0 {
            let next = Header::from_ptr(header.next as *const u8);
            if next.free {
                header.next = next.next;
                header.size += next.size + size_of::<Header>();
            }
        }
        header.free = true;
        unsafe {
            memcpy(header_addr as *mut u8, header.as_ptr(), size_of::<Header>());
        }
    }
}

pub fn print_alloc_list() {
    unsafe {
        let mut addr = HEAP_START;
        while addr != 0 {
            let block = Header::from_ptr(addr as *mut u8);
            addr = block.next;
            println!("{:x?}", block);
        }
    }
}

impl Header {
    fn null(previous: u32, size: usize) -> Header {
        Header {
            previous: previous,
            next: 0,
            size: size,
            free: true
        }
    }
    
    fn as_ptr(&mut self) -> *const u8 {
        self as *const Header as *const u8
    }
    
    fn from_ptr(ptr: *const u8) -> Header {
        unsafe {
            *(ptr as *const Header)
        }
    }
}