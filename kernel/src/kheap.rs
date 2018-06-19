#![allow(dead_code)]

use core::mem::size_of;
use rlibc::{memset,memcpy};
use paging::*;
use vga::*;

pub static mut KHEAP_SIZE: usize = 0x1000000;
pub static mut KHEAP_ADDR: u32 = 0;
pub static mut KHEAP_END: u32 = 0;

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Header {
    previous: u32,
    next: u32,
    size: usize,
    free: bool
}

macro_rules! align {
    ($size:expr) => {
        if ($size & 0xfffff000) != $size {
            ($size & 0xfffff000) + 0x1000
        } else {
            $size
        };
    }
}

pub fn kheap_init(ram_size: u32) {
    unsafe {
        KHEAP_ADDR = get_kernel_end();
        KHEAP_SIZE = ((ram_size / 1000 - 1) * 0x100000 - phys!(KHEAP_ADDR)) as usize;
        KHEAP_END = KHEAP_ADDR + KHEAP_SIZE as u32;
        if (KHEAP_ADDR & 0xfffff000) != KHEAP_ADDR {
            KHEAP_ADDR &= 0xfffff000;
            KHEAP_ADDR += 0x1000;
        }
        let mut entry_addr = 0;
        INITIAL_PD.alloc_frame(&mut entry_addr, &mut phys!(KHEAP_ADDR), KERNEL_MODE);
        memset(entry_addr as *mut u8, 0, FRAME_SIZE);
        memcpy(entry_addr as *mut u8, Header::null(0, KHEAP_SIZE).as_ptr(), size_of::<Header>());
    }
}

pub fn kmalloc(size: usize) -> u32 {
    let aligned_size = align!(size) + align!(size_of::<Header>()) - size_of::<Header>();
    let mut addr = empty_block(aligned_size);
    if kmalloc_table_check(addr, aligned_size) {
        addr = empty_block(aligned_size);
    }
    let mut block = Header::from_ptr(addr as *mut u8);
    if block.size >= aligned_size && block.free {
        if block.next == 0 {
            block.new_tail(addr, aligned_size);
        } else {
            block.new_block(addr, aligned_size);
        }
        addr += size_of::<Header>() as u32;
        unsafe { memset(addr as *mut u8, 0, aligned_size); }
        return addr;
    }
    return 0;
}

pub fn kfree(addr: u32) {
    unsafe {
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
                let mut next = Header::from_ptr(header.next as *const u8);
                if next.free {
                    header.next = next.next;
                    header.size += next.size + size_of::<Header>();
                }
                if header.next != 0 {
                    next = Header::from_ptr(header.next as *const u8);
                    if next.previous != header_addr {
                        next.previous = header_addr;
                        memcpy(header.next as *mut u8, next.as_ptr(), size_of::<Header>());
                    }
                }
            }
            header.free = true;
            memcpy(header_addr as *mut u8, header.as_ptr(), size_of::<Header>());
        }
    }
}

pub fn umalloc(size: usize) -> u32 {
    unsafe {
        let aligned_size = align!(size);
        
        let pd_backup = if get_cr3() != phys!(INITIAL_PD.tables as u32) {
            switch_directory(&mut INITIAL_PD);
            USER_PD
        } else {
            &mut INITIAL_PD as *mut PageDirectory
        };
        umalloc_table_check(aligned_size);
        let mut phys_addr = phys!(kmalloc(size) + (FRAME_SIZE - size_of::<Header>()) as u32);
        switch_directory(pd_backup);
        
        let mut virt_addr = 0;
        let mut tmp = 0;
        (*USER_PD).alloc_frame(&mut virt_addr, &mut phys_addr, USER_MODE);
        for i in 1..(aligned_size / FRAME_SIZE) {
            (*USER_PD).alloc_frame(&mut tmp, &mut (phys_addr + (i * FRAME_SIZE) as u32), USER_MODE);
        }
        return virt_addr;
    }
}

pub fn ufree(addr: u32) {
    unsafe {
        let frame_idx = addr / FRAME_SIZE as u32;
        let table_idx = frame_idx as usize / TABLE_FSIZE;
        let entry_idx = frame_idx as usize % TABLE_FSIZE;
        let table_ptr = virt!((*USER_PD)[table_idx] &! 0xfff) as *mut PageTable;
        let entry_addr = virt!((*table_ptr)[entry_idx] &! 0xfff);
        kfree(entry_addr - (FRAME_SIZE - size_of::<Header>()) as u32);
    }
}

fn empty_block(size: usize) -> u32 {
    let mut addr = unsafe { KHEAP_ADDR };
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

fn kmalloc_table_check(addr: u32, size: usize) -> bool {
    unsafe {
        let total_size = size + size_of::<Header>();
        let mut alloc = false;
        let mut start_idx = addr as usize / TABLE_SIZE;
        if addr % TABLE_SIZE as u32 != 0 {
            start_idx += 1;
        }
        let mut end_idx = (addr as usize + total_size) / TABLE_SIZE;
        if Header::from_ptr(addr as *mut u8).next == 0 {
            end_idx += 1;
        }
        let aligned_size = align!(FRAME_SIZE + size_of::<Header>()) - size_of::<Header>();
        for i in start_idx..end_idx {
            let block_addr = empty_block(aligned_size);
            let mut block = Header::from_ptr(block_addr as *mut u8);
            if block.size >= aligned_size && block.free {
                let table_addr = INITIAL_PD.new_table(block_addr + FRAME_SIZE as u32);
                INITIAL_PD[i] = phys!(table_addr) | 0x3;
                if block.next == 0 {
                    block.new_tail(block_addr, aligned_size);
                } else {
                    block.new_block(block_addr, aligned_size);
                }
            }
            alloc = true;
        }
        return alloc;
    }
}

fn umalloc_table_check(size: usize) {
    unsafe {
        let frame_idx = (*USER_PD).mmap_get_free_area(size);
        let mut start_idx = frame_idx as usize / TABLE_FSIZE;
        if frame_idx as usize % TABLE_FSIZE == 0 {
            let table_addr = kmalloc(FRAME_SIZE) + (FRAME_SIZE - size_of::<Header>()) as u32;
            (*USER_PD)[start_idx] = phys!(table_addr) | 0x3 | USER_MODE;
            start_idx += 1;
        }
        if frame_idx as usize + size / FRAME_SIZE >= TABLE_FSIZE {
            let end_idx = (frame_idx as usize + size / FRAME_SIZE) / TABLE_FSIZE;
            for i in start_idx..end_idx {
                let table_addr = kmalloc(FRAME_SIZE) + (FRAME_SIZE - size_of::<Header>()) as u32;
                (*USER_PD)[i] = phys!(table_addr) | 0x3 | USER_MODE;
            }
        }
    }
}

pub fn print_kmalloc_list() {
    unsafe {
        let mut addr = KHEAP_ADDR;
        while addr != 0 {
            let block = Header::from_ptr(addr as *mut u8);
            addr = block.next;
            println!("{:x?}", block);
        }
        println!();
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
    
    fn new_block(&mut self, addr: u32, size: usize) {
        self.free = false;
        if size == self.size {
            unsafe {
                memcpy(addr as *mut u8, self.as_ptr(), size_of::<Header>());
            }
        } else {
            self.size = size;
            // if the block is not the tail, need to save the next block
            // before creating a new block at self.next
            let tmp = self.next;
            let mut tmp_header = Header::from_ptr(tmp as *const u8);
            
            self.next = addr + (size_of::<Header>() + size) as u32;
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
    }
    
    fn new_tail(&mut self, addr: u32, size: usize) {
        unsafe {
            self.size = size;
            self.free = false;
            self.next = addr + (size_of::<Header>() + size) as u32;
            // alloc new frames if need
            if (addr as usize % FRAME_SIZE) + size_of::<Header>() + size > FRAME_SIZE {
                for i in 1..((size_of::<Header>() + size) / FRAME_SIZE + 1) {
                    let mut phys_addr = phys!(addr + (i * FRAME_SIZE) as u32);
                    let mut tmp = 0;
                    INITIAL_PD.alloc_frame(&mut tmp, &mut phys_addr, KERNEL_MODE);
                }
            }
            let tail_size = (KHEAP_END - self.next) as usize - size_of::<Header>();
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