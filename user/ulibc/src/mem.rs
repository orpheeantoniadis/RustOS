// extern crate alloc;
use core::mem::size_of;
use rlibc::{memset,memcpy};
use io::*;

// #[global_allocator]
// static mut ALLOCATOR: Allocator = Allocator {};

// #[derive(Debug)]
// pub struct Allocator {}

const FRAME_SIZE: usize = 0x1000;
const HEAP_END: u32 = 0xffffffff;

static mut HEAP_START: u32 = 0;
static mut HEAP_SIZE: usize = 0;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
struct Header {
    previous: u32,
    next: u32,
    size: usize,
    free: bool
}

macro_rules! align {
    ($size:expr) => {
        if ($size & 0xfffffff0) != $size {
            ($size & 0xfffffff0) + 0x10
        } else {
            $size
        };
    }
}

fn heap_init() {
    unsafe {
        if HEAP_START == 0 {
            HEAP_START = syscall(Syscall::AllocFrame, 0, 0, 0, 0) as u32;
            HEAP_SIZE = (HEAP_END - HEAP_START) as usize;
            memset(HEAP_START as *mut u8, 0, FRAME_SIZE);
            memcpy(HEAP_START as *mut u8, Header::null(0, HEAP_SIZE).as_ptr(), size_of::<Header>());
        }
    }
}

pub fn malloc(size: usize) -> u32 {
    unsafe {
        heap_init();
        let aligned_size = align!(size);
        let total_size = aligned_size + size_of::<Header>();
        let addr = empty_block(aligned_size);
        let mut block = Header::from_ptr(addr as *mut u8);
        
        // continue if data fits in the block
        if block.size >= aligned_size && block.free {
            block.size = aligned_size;
            block.free = false;
            
            // if the block is the last (at the tail), creating new tail
            if block.next == 0 {
                block.next = addr + total_size as u32;
                // alloc a new frame if need
                if (addr as usize % FRAME_SIZE) + total_size > FRAME_SIZE {
                    for _i in 0..(total_size / FRAME_SIZE) {
                        let entry_addr = syscall(Syscall::AllocFrame, 0, 0, 0, 0) as u32;
                        memset(entry_addr as *mut u8, 0, FRAME_SIZE);
                    }
                }
                let tail_size = (HEAP_END - block.next) as usize - size_of::<Header>();
                let mut tail = Header::null(addr, tail_size);
                memcpy(addr as *mut u8, block.as_ptr(), size_of::<Header>());
                memcpy(block.next as *mut u8, tail.as_ptr(), size_of::<Header>());
                
            // if the block is not the tail, need to save the next block
            // before creating a new block at block.next
            } else {
                let tmp = block.next;
                let mut tmp_header = Header::from_ptr(tmp as *const u8);
                
                block.next = addr + total_size as u32;
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
        
        // if no block found return 0
        return 0;
    }
}

fn empty_block(size: usize) -> u32 {
    let mut addr = unsafe { HEAP_START };
    let mut block = Header::from_ptr(addr as *mut u8);
    while block.next != 0 {
        if block.size >= size && block.free {
            break;
        }
        addr = block.next;
        block = Header::from_ptr(addr as *mut u8);
    }
    return addr;
}

pub fn free(addr: u32) {
    unsafe {
        syscall(Syscall::FreeFrame, addr, 0, 0, 0);
    }
}

pub fn print_kmalloc_list() {
    let mut addr = unsafe { HEAP_START };
    while addr != 0 {
        let block = Header::from_ptr(addr as *mut u8);
        addr = block.next;
        println!("{:x?}", block);
    }
    println!();
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
    
    fn insert(&mut self, addr: u32, size: usize) {
        let total_size = size + size_of::<Header>();
        let tmp = self.next;
        let mut tmp_header = Header::from_ptr(tmp as *const u8);
        
        self.next = addr + total_size as u32;
        let next_block_size = (tmp - self.next) as usize - size_of::<Header>();
        let mut next_header = Header::null(addr, next_block_size);
        next_header.next = tmp;
        tmp_header.previous = self.next;
        
        unsafe {
            memcpy(addr as *mut u8, self.as_ptr(), size_of::<Header>());
            memcpy(self.next as *mut u8, next_header.as_ptr(), size_of::<Header>());
            memcpy(tmp as *mut u8, tmp_header.as_ptr(), size_of::<Header>());
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

// unsafe impl<'a> alloc::heap::Alloc for &'a Allocator {
//     unsafe fn alloc(&mut self, layout:  alloc::heap::Layout) -> Result<*mut u8,  alloc::heap::AllocErr> {
//         let addr = malloc(layout.size());
// 
//         if addr > 0 {
//             Ok(addr as *mut u8)
//         } else {
//             Err(alloc::heap::AllocErr::Exhausted{ request: layout })
//         }
//     }
// 
//     unsafe fn dealloc(&mut self, ptr: *mut u8, layout: alloc::heap::Layout) {
//         free(ptr as u32);
//     }
// }