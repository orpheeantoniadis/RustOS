#![allow(dead_code)]
#![allow(unused_variables)]

use core::str;
use core::mem;
use rlibc::memcpy;
use vga::*;
use ide::*;

const FDT_SIZE : usize = 128;
const ENTRY_SIZE : usize = 32;
pub const MAX_FILENAME_LENGTH: usize = 26;
pub static mut FDT: Fdt = [FdtEntry::null();FDT_SIZE];

pub type Fdt = [FdtEntry; FDT_SIZE];

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FdtEntry {
    pub stat: Stat,
    pub pos: usize
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Stat {
    pub name: [u8;MAX_FILENAME_LENGTH],
    pub size: u32,
    pub entry_offset: u16,
    pub start: u16
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FileIterator {
    pub sector: u32,
    pub offset: usize
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
        if FDT[fd as usize].stat.start == 0 {
            return -1;
        }
        
        let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
        read_sector(1, &mut sector[0] as *mut u16);
        let fat = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
        
        let mut cnt = 0;
        let mut block = FDT[fd as usize].stat.start as usize;
        let size = if FDT[fd as usize].pos + n > FDT[fd as usize].stat.size as usize {
            FDT[fd as usize].stat.size as usize
        } else {
            FDT[fd as usize].pos + n
        };
        
        for i in 0..(size / SECTOR_SIZE) {
            if i >= FDT[fd as usize].pos / SECTOR_SIZE {
                read_sector(block as u32, &mut sector[0] as *mut u16);
                let data = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                memcpy(buf.offset(cnt as isize), &data[0], SECTOR_SIZE);
                FDT[fd as usize].pos += SECTOR_SIZE;
                cnt += SECTOR_SIZE;
            }
            block = fat[block] as usize;
        }
        
        if FDT[fd as usize].pos >= size {
            return 0;
        } else {
            read_sector(block as u32, &mut sector[0] as *mut u16);
            let data = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
            memcpy(buf.offset(cnt as isize), &data[FDT[fd as usize].pos % SECTOR_SIZE], size % SECTOR_SIZE);
            FDT[fd as usize].pos += size % SECTOR_SIZE;
            return n as i32;
        }
    }
}

pub fn file_seek(fd: i32, offset: u32) -> i32{
    unimplemented!()
}

pub fn file_close(fd: i32) {
    unimplemented!()
}

pub fn bytes_to_str(bytes: &[u8]) -> &str {
    let mut cnt = 0;
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        cnt += 1;
    }
    str::from_utf8(&bytes[0..cnt]).expect("Found invalid UTF-8")
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

fn read_super_block() -> (usize, usize, u32) {
    let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
    read_sector(0, &mut sector[0] as *mut u16);
    let superblock = unsafe {
        mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
    };
    let block_size = superblock[13] as usize * SECTOR_SIZE;
    let fat_size = superblock[0x24] as usize;
    let root_entry = superblock[0x2c] as u32;
    
    return (block_size, fat_size, root_entry);
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

impl Stat {
    pub fn new(filename: &str) -> Stat {
        let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
        let mut raw_filename = [0;MAX_FILENAME_LENGTH];
        let mut it = FileIterator::new();
        while it.has_next() {
            it.next(&mut raw_filename[0]);
            if filename == bytes_to_str(&raw_filename) {
                read_sector(it.sector as u32, &mut sector[0] as *mut u16);
                unsafe {
                    let offset = it.offset - ENTRY_SIZE;
                    let entries = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                    let start = [entries[offset+26], entries[offset+27]];
                    let start = mem::transmute::<[u8;2], u16>(start);
                    let size = [entries[offset+28], entries[offset+29], entries[offset+30], entries[offset+31]];
                    let size = mem::transmute::<[u8;4], u32>(size);
                    return Stat {
                        name: raw_filename,
                        size: size,
                        entry_offset: offset as u16,
                        start: start
                    }
                }
            }
        }
        Stat { name: raw_filename, size: 0, entry_offset: 0, start: 0 }
    }
}

impl FileIterator {
    pub fn new() -> FileIterator {
        let superblock = read_super_block();
        FileIterator {
            sector: superblock.2,
            offset: 0
        }
    }
    
    pub fn has_next(&mut self) -> bool {
        let superblock = read_super_block();
        
        if self.offset < superblock.0 {
            let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
            read_sector(self.sector, &mut sector[0] as *mut u16);
            let entries = unsafe {
                mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
            };
            let bytes = &entries[self.offset..self.offset+MAX_FILENAME_LENGTH];
            if entries[self.offset] != 0 {
                return true;
            }
        }
        return false;
    }
    
    pub fn next(&mut self, filename: *mut u8) {
        unsafe {
            if self.has_next() {
                let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
                read_sector(self.sector, &mut sector[0] as *mut u16);
                let entries = mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector);
                memcpy(filename, &entries[self.offset], MAX_FILENAME_LENGTH);
                self.offset += ENTRY_SIZE;
            }
        }
    }
}

pub fn fs_test(filename: &str) {
    // superblock read
    let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
    
    let superblock = read_super_block();
    println!("Block size = {} bytes", superblock.0);
    println!("FAT size = {} bytes", superblock.1);
    println!("Root entry = block number {}", superblock.2);
    
    // read fat
    read_sector(1, &mut sector[0] as *mut u16);
    let fat = unsafe {
        mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
    };
    
    // entries read
    read_sector(superblock.2 as u32, &mut sector[0] as *mut u16);
    let entries = unsafe {
        mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
    };
    let mut cnt = 0;
    println!("Files :");
    while cnt < SECTOR_SIZE {
        if entries[cnt] == 0 {
            break;
        }
        let name = bytes_to_str(&entries[cnt..cnt+26]);
        let start = unsafe {
            mem::transmute::<[u8;2], u16>([entries[cnt+26], entries[cnt+27]])
        };
        let size = unsafe {
            mem::transmute::<[u8;4], u32>([entries[cnt+28], entries[cnt+29], entries[cnt+30], entries[cnt+31]])
        };
        println!("\n{}", name);
        println!("Start at block {}", start);
        println!("{} bytes", size);
    
        // data read
        let mut block = start as usize;
        loop {
            read_sector(block as u32, &mut sector[0] as *mut u16);
            let data = unsafe {
                mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
            };
            println!("{:?}", &data[..]);
            block = fat[block] as usize;
            if block == 0 {
                break;
            }
        }
        cnt += ENTRY_SIZE;
    }
    
    println!("");
}