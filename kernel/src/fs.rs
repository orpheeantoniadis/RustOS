#![allow(dead_code)]
#![allow(unused_variables)]

use core::str;
use core::mem;
use vga::*;
use ide::*;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Stat<'a> {
    name: &'a str,
    size: u32,
    entry_offset: u16,
    start: u16
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct FileIterator {
    
}

pub fn file_stat(filename: &str, stat: Stat) -> i32 {
    unimplemented!()
}

pub fn file_exists(filename: &str) -> bool {
    let mut sector : [u16;SECTOR_SIZE/2] = [0;SECTOR_SIZE/2];
    read_sector(0, &mut sector[0] as *mut u16);
    let superblock = unsafe {
        mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
    };
    let label = bytes_to_str(&superblock[0x52..0x59]);
    let block_size = superblock[13] as usize * SECTOR_SIZE;
    let fat_size = superblock[0x24] as usize;
    let root_entry = superblock[0x2c];
    let entries_size = fat_size * 32; // entry size = 32
    println!("\nReading {}...", label);
    println!("Block size = {} bytes", block_size);
    println!("FAT size = {} bytes", fat_size);
    println!("Root entry = block number {}", root_entry);
    println!("Entries-sector size = {} bytes", entries_size);
    
    read_sector(root_entry as u32, &mut sector[0] as *mut u16);
    let entries = unsafe {
        mem::transmute::<[u16;SECTOR_SIZE/2], [u8;SECTOR_SIZE]>(sector)
    };
    let mut cnt = 0;
    println!("Files :");
    while cnt < SECTOR_SIZE {
        if entries[cnt] != 0 {
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
        } else {
            break;
        }
        cnt += 32;
    }
    
    println!("");
    return true;
}

pub fn file_open(filename: &str) -> i32 {
    unimplemented!()
}

pub fn file_read(fd: i32, buf: *mut (), count: u32) -> i32 {
    unimplemented!()
}

pub fn file_seek(fd: i32, offset: u32) -> i32{
    unimplemented!()
}

pub fn file_close(fd: i32) {
    unimplemented!()
}

pub fn file_iterator() -> FileIterator {
    unimplemented!()
}

pub fn file_has_next(it: FileIterator) -> bool {
    unimplemented!()
}

pub fn file_next(filename: *mut str, it: FileIterator) {
    unimplemented!()
}

fn bytes_to_str(bytes: &[u8]) -> &str {
    let mut cnt = 0;
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        cnt += 1;
    }
    str::from_utf8(&bytes[0..cnt]).expect("Found invalid UTF-8")
}