#![allow(dead_code)]

// CRTC ports
const CRTC_CMD: u16 = 0x3d4;
const CRTC_DATA: u16 = 0x3d5;
// CRTC registers
const CRTC_LOCATION_MSB: u8 = 0xe;
const CRTC_LOCATION_LSB: u8 = 0xf;

extern "C" {
    pub fn outb(port: u16, data: u8);
    pub fn outw(port: u16, data: u16);
    pub fn inb(port: u16) -> u8;
    pub fn inw(port: u16) -> u16;
}

pub fn move_cursor(position: u16) {
    unsafe {
        let pos_msb = ((position >> 8) & 0xff) as u8;
        let pos_lsb = (position & 0xff) as u8;
        outb(CRTC_CMD, CRTC_LOCATION_MSB);
        outb(CRTC_DATA, pos_msb);
        outb(CRTC_CMD, CRTC_LOCATION_LSB);
        outb(CRTC_DATA, pos_lsb);
    }
}