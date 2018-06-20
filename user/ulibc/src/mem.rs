// extern crate alloc;
use core::mem::size_of;
use rlibc::{memset,memcpy};
use io::*;

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
    heap_init();
    let aligned_size = align!(size);
    let mut addr = empty_block(aligned_size);
    let mut block = Header::from_ptr(addr as *mut u8);
    if block.size >= aligned_size && block.free {
        if block.next == 0 {
            block.insert_tail(addr, aligned_size);
        } else {
            block.insert(addr, aligned_size);
        }
        addr += size_of::<Header>() as u32;
        unsafe { memset(addr as *mut u8, 0, aligned_size); }
        return addr;
    }
    return 0;
}

pub fn free(addr: u32) {
    unsafe {
        let mut header_addr = addr - size_of::<Header>() as u32;
        let mut header = Header::from_ptr(header_addr as *const u8);
        if !header.free {
            let start = header_addr;
            let mut end = header.next;
            if header.previous != 0 {
                let previous = Header::from_ptr(header.previous as *const u8);
                if previous.free {
                    header_addr = header.previous;
                    header.previous = previous.previous;
                    header.size += previous.size + size_of::<Header>();
                }
            }
            if header.next != 0 {
                let mut next = Header::from_ptr(header.next as *const u8);
                if next.free {
                    header.next = next.next;
                    header.size += next.size + size_of::<Header>();
                }
                if header.next != 0 {
                    end = header.next;
                    next = Header::from_ptr(header.next as *const u8);
                    if next.previous != header_addr {
                        next.previous = header_addr;
                        memcpy(header.next as *mut u8, next.as_ptr(), size_of::<Header>());
                    }
                }
            }
            header.free = true;
            memcpy(header_addr as *mut u8, header.as_ptr(), size_of::<Header>());
            // free unused page tables
            let start_idx = start as usize / FRAME_SIZE;
            let mut end_idx = end as usize / FRAME_SIZE;
            if header.next == 0 {
                end_idx += 1;
            }
            for i in start_idx..end_idx {
                syscall(Syscall::FreeFrame, (i * FRAME_SIZE) as u32, 0, 0, 0);
            }
        }
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
        unsafe {
            let total_size = size + size_of::<Header>();
            if (addr as usize % FRAME_SIZE) + total_size > FRAME_SIZE {
                for _i in 1..(total_size / FRAME_SIZE + 1) {
                    let entry_addr = syscall(Syscall::AllocFrame, 0, 0, 0, 0) as u32;
                    memset(entry_addr as *mut u8, 0, FRAME_SIZE);
                }
            }
            self.free = false;
            if size == self.size {
                memcpy(addr as *mut u8, self.as_ptr(), size_of::<Header>());
            } else {
                let tmp = self.next;
                let mut tmp_header = Header::from_ptr(tmp as *const u8);
                
                self.size = size;
                self.next = addr + total_size as u32;
                let next_block_size = (tmp - self.next) as usize - size_of::<Header>();
                let mut next_header = Header::null(addr, next_block_size);
                next_header.next = tmp;
                tmp_header.previous = self.next;
                memcpy(addr as *mut u8, self.as_ptr(), size_of::<Header>());
                memcpy(self.next as *mut u8, next_header.as_ptr(), size_of::<Header>());
                memcpy(tmp as *mut u8, tmp_header.as_ptr(), size_of::<Header>());
            }
        }
    }
    
    fn insert_tail(&mut self, addr: u32, size: usize) {
        unsafe {
            let total_size = size + size_of::<Header>();
            self.size = size;
            self.free = false;
            self.next = addr + total_size as u32;
            // alloc new frames if need
            if (addr as usize % FRAME_SIZE) + total_size > FRAME_SIZE {
                for _i in 0..(total_size / FRAME_SIZE) {
                    let entry_addr = syscall(Syscall::AllocFrame, 0, 0, 0, 0) as u32;
                    memset(entry_addr as *mut u8, 0, FRAME_SIZE);
                }
            }
            let tail_size = (HEAP_END - self.next) as usize - size_of::<Header>();
            let mut tail = Header::null(addr, tail_size);
            memcpy(addr as *mut u8, self.as_ptr(), size_of::<Header>());
            memcpy(self.next as *mut u8, tail.as_ptr(), size_of::<Header>());
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