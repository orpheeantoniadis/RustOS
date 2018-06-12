#![allow(dead_code)]

use core::mem::size_of;
use rlibc::{memset,memcpy};
use paging::*;
// use vga::*;

const HEAP_SIZE: usize = KHEAP_SIZE;

static mut HEAP_START: u32 = 0;
static mut HEAP_END: u32 = 0;
static mut HEAP_ADDR: u32 = 0;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
struct Header {
    previous: u32,
    next: u32,
    size: u32,
    free: bool
}

pub fn heap_init() {
    unsafe {
        HEAP_START = phys!(KHEAP_ADDR) + KHEAP_SIZE as u32;
        HEAP_START = (HEAP_START & 0xfffff000) + 0x1000;
        HEAP_END = HEAP_START + HEAP_SIZE as u32;
        HEAP_ADDR = HEAP_START;
        
        for i in page!(HEAP_START)..page!(HEAP_END) {
            INITIAL_PD[i as usize] = phys!(INITIAL_PD.new_table()) | 0x3 | USER_MODE;
        }
        let frame_idx = HEAP_ADDR / FRAME_SIZE as u32;
        let table_idx = frame_idx as usize / TABLE_SIZE;
        let entry_idx = frame_idx as usize % TABLE_SIZE;
        let table_ptr = virt!(INITIAL_PD[table_idx] &! 0xfff) as *mut PageTable;
        (*table_ptr)[entry_idx] = HEAP_ADDR | 0x3 | USER_MODE;
        memset(HEAP_ADDR as *mut u8, 0, FRAME_SIZE);
        memcpy(HEAP_ADDR as *mut u8, Header::null(HEAP_ADDR).as_ptr(), size_of::<Header>());
    }
}

pub fn malloc(_size: usize) {
    unimplemented!()
}

pub fn free() {
    unimplemented!()
}

impl Header {
    fn null(previous: u32) -> Header {
        Header {
            previous: previous,
            next: 0,
            size: 0,
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