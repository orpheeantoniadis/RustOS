#![allow(dead_code)]

use core::str;
use core::mem;
use rlibc::memcpy;
use ide::*;
use vga::*;
use common::*;

const FDT_SIZE : usize = 128;
const ENTRY_SIZE : usize = 32;

pub static mut FDT: Fdt = [FdtEntry::null();FDT_SIZE];
pub static mut SB : Superblock = Superblock::null();

pub type Fdt = [FdtEntry; FDT_SIZE];

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FdtEntry {
    pub stat: Stat,
    pub pos: usize
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Superblock {
    pub block_size: usize,
    pub fat_size: usize,
    pub root_entry: usize
}

pub trait StatBuilder {
    fn new(filename: &str) -> Self;
}

pub trait FileIteratorBuilder {
    fn new() -> Self;
    fn has_next(&mut self) -> bool;
    fn next(&mut self, filename: *mut u8) -> i8;
}

pub fn file_exists(filename: &str) -> bool {
    let mut raw_filename = [0;MAX_FILENAME_LENGTH];
    let mut it = FileIterator::new();
    while it.has_next() {
        it.next(&mut raw_filename[0]);
        let it_filename = bytes_to_str(&raw_filename);
        if it_filename == filename {
            return true;
        }
    }
    return false;
}

pub fn file_open(filename: &str) -> i32 {
    unsafe {
        if file_exists(filename) {
            let fd = free_fd();
            FDT[fd as usize].stat = Stat::new(filename);
            return fd;
        }
        return -1;
    }
}

pub fn file_read(fd: i32, buf: *mut u8, n: usize) -> i32 {
    unsafe {
        if fd < 0 || FDT[fd as usize].stat.start == 0 {
            return -1;
        }
        
        let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
        read_sector(1, &mut sector[0] as *mut u16);
        let fat = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
        
        let mut cnt = 0;
        let mut block = FDT[fd as usize].stat.start;
        let mut sector_id = block * (SB.block_size / SECTOR_SIZE);
        let size = if FDT[fd as usize].pos + n > FDT[fd as usize].stat.size {
            FDT[fd as usize].stat.size
        } else {
            FDT[fd as usize].pos + n
        };
        
        for i in 0..(size / SECTOR_SIZE) {
            if i >= FDT[fd as usize].pos / SECTOR_SIZE {
                sector_id += i % (SB.block_size / SECTOR_SIZE);
                read_sector(sector_id as u32, &mut sector[0] as *mut u16);
                let data = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                memcpy(buf.offset(cnt as isize), &data[0], SECTOR_SIZE);
                FDT[fd as usize].pos += SECTOR_SIZE;
                cnt += SECTOR_SIZE;
            }
            if FDT[fd as usize].pos % SB.block_size == 0 {
                block = fat[block] as usize;
                sector_id = block * (SB.block_size / SECTOR_SIZE);
            }
        }
        
        if FDT[fd as usize].pos >= size {
            return 0;
        } else {
            read_sector(sector_id as u32, &mut sector[0] as *mut u16);
            let data = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
            memcpy(buf.offset(cnt as isize), &data[FDT[fd as usize].pos % SECTOR_SIZE], n % SECTOR_SIZE);
            FDT[fd as usize].pos += n % SECTOR_SIZE;
            return n as i32;
        }
    }
}

pub fn file_seek(fd: i32, offset: usize) -> i32 {
    unsafe {
        if FDT[fd as usize].pos + offset > FDT[fd as usize].stat.size {
            FDT[fd as usize].pos = FDT[fd as usize].stat.size;
            return -1;
        } else {
            FDT[fd as usize].pos += offset;
            return 0;
        }
    }
}

pub fn file_close(fd: i32) -> i32 {
    if fd < 0 || unsafe { FDT[fd as usize].stat.start } == 0 {
        println!("fd {} does not exist.", fd);
        return -1;
    } else {
        unsafe { FDT[fd as usize] = FdtEntry::null() };
        return 0;
    }
}

fn free_fd() -> i32 {
    unsafe {
        let mut cnt = 0;
        for entry in FDT.iter() {
            if entry.stat.start == 0 {
                return cnt;
            }
            cnt += 1;
        }
        return -1;
    }
}

pub fn set_superblock() {
    unsafe { SB = Superblock::new(); }
}

impl FdtEntry {
    const fn null() -> FdtEntry {
        FdtEntry {
            stat: Stat {
                name: [0;MAX_FILENAME_LENGTH],
                size: 0,
                entry_offset: 0,
                start: 0
            },
            pos: 0
        }
    }
}

impl StatBuilder for Stat {
    fn new(filename: &str) -> Stat {
        let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
        let mut raw_filename = [0;MAX_FILENAME_LENGTH];
        let mut it = FileIterator::new();
        while it.has_next() {
            it.next(&mut raw_filename[0]);
            if filename == bytes_to_str(&raw_filename) {
                read_sector(it.sector, &mut sector[0] as *mut u16);
                unsafe {
                    let offset = it.offset - ENTRY_SIZE;
                    let entries = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                    let start = [entries[offset+26], entries[offset+27]];
                    let start = mem::transmute::<[u8;2], u16>(start);
                    let size = [entries[offset+28], entries[offset+29], entries[offset+30], entries[offset+31]];
                    let size = mem::transmute::<[u8;4], u32>(size);
                    return Stat {
                        name: raw_filename,
                        size: size as usize,
                        entry_offset: offset as u16,
                        start: start as usize
                    }
                }
            }
        }
        Stat { name: raw_filename, size: 0, entry_offset: 0, start: 0 }
    }
}

impl FileIteratorBuilder for FileIterator {
    fn new() -> FileIterator {
        FileIterator {
            sector: (unsafe { SB.root_entry * SB.block_size } / SECTOR_SIZE) as u32,
            offset: 0
        }
    }
    
    fn has_next(&mut self) -> bool {    
        if self.sector < self.sector + unsafe { SB.block_size / SECTOR_SIZE } as u32 {
            let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
            read_sector(self.sector, &mut sector[0] as *mut u16);
            let entries = unsafe {
                mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
            };
            if entries[self.offset] != 0 {
                return true;
            }
        }
        return false;
    }
    
    fn next(&mut self, filename: *mut u8) -> i8 {
        unsafe {
            if self.has_next() {
                let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
                read_sector(self.sector, &mut sector[0] as *mut u16);
                let entries = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                memcpy(filename, &entries[self.offset], MAX_FILENAME_LENGTH);
                self.offset = (self.offset + ENTRY_SIZE) % SECTOR_SIZE;
                if self.offset == 0 {
                    self.sector += 1;
                }
                return 0;
            }
            return -1;
        }
    }
}

impl Superblock {
    const fn null() -> Superblock {
        Superblock { block_size: 0, fat_size: 0, root_entry: 0 }
    }
    
    fn new() -> Superblock {
        let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
        read_sector(0, &mut sector[0] as *mut u16);
        let raw_sb = unsafe {
            mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
        };
        let label = bytes_to_str(&raw_sb[0x52..0x59]);
        let block_size = raw_sb[13] as usize * SECTOR_SIZE;
        let fat_size = unsafe {
            mem::transmute::<[u8;4], u32>([raw_sb[0x24], raw_sb[0x25], raw_sb[0x26], raw_sb[0x27]])
        };
        let root_entry = raw_sb[0x2c];
        println!("\n{} ready.", label);
        println!("Block size = {} bytes", block_size);
        println!("FAT size = {} bytes", fat_size);
        println!("Root entry = block number {}\n", root_entry);
        
        Superblock {
            block_size: block_size,
            fat_size: fat_size as usize,
            root_entry: root_entry as usize
        }
    }
}