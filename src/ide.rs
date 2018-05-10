#![allow(dead_code)]

/**
 * Simple IDE read/write routines using PIO mode.
 * This code is very CPU intensive and not efficient
 * (if that's what you're after, use DMA mode instead).
 * Reference: http://wiki.osdev.org/ATA_PIO_Mode
 * ATA disk0, I/O ports: 0x1f0-0x1f7, 0x3f6
 * ATA disk1, I/O ports: 0x170-0x177, 0x376
 */

use pio::*;

// IDE ports
const IDE_CMD : u16 = 0x1f7;
const IDE_DATA : u16 = 0x1f0;

const SECTOR_SIZE : usize = 512;

/**
 * Wait for the disk drive to be ready.
 */
pub fn wait_drive() {
    unsafe { while(inb(IDE_CMD) & 192) != 64 { } }
}

/**
 * Prepare the disk drive for read/write at the specified sector in LBA mode.
 * @param sector the sector to read or write (0-indexed).
 */
fn pio_prepare(sector: u32) {
    unsafe {
    	wait_drive();
    	outb(0x1f2, 1);                                        // 1 sector
    	outb(0x1f3, (sector & 0xff) as u8);                    // send bits 0-7 of LBA
    	outb(0x1f4, ((sector >> 8) & 0xff) as u8);             // send bits 8-15 of LBA
    	outb(0x1f5, ((sector >> 16) & 0xff) as u8);            // send bits 16-23 of LBA
    	outb(0x1f6, ((sector >> 24) & 0x0f) as u8 | 0xe0);     // send bits 24-27 of LBA + set LBA mode; 0xe0 = 11100000b;
    }
}

/**
 * Read sectors from the first disk.
 * @param sector first sector to read (0-indexed)
 * @param dst address to store to read data
 * Based on the assembly code at http://wiki.osdev.org/ATA_read/write_sectors
 */
fn read_sector(sector: u32, dst: *mut u16) {
    unsafe {
    	pio_prepare(sector);

    	outb(IDE_CMD, 0x20);  // read with retry
    	wait_drive();

        for i in 0..(SECTOR_SIZE/2) {
            *dst.offset(i as isize) = inw(IDE_DATA);
        }
    }
}

/**
 * Write sectors from the first disk.
 * @param sector first sector to write (0-indexed)
 * @param src address of the data to be written
 */
fn write_sector(sector: u32, src: *mut u16) {
    unsafe {
    	pio_prepare(sector);

    	outb(IDE_CMD, 0x30);  // write with retry
    	wait_drive();

        for i in 0..(SECTOR_SIZE/2) {
            outw(IDE_DATA, *src.offset(i as isize));
        }
    }
}