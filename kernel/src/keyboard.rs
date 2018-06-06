#![allow(dead_code)]

use pio::*;
use common::*;

// Keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATE_PORT: u16 = 0x64;

const CIRC_BUFFER_SIZE: usize = 30;

static mut BUFFER: CircBuffer = CircBuffer::new();
static mut SHIFT: bool = false;

struct CircBuffer {
    buffer: [i32;CIRC_BUFFER_SIZE],
    read: usize,
    write: usize,
    count: usize
}

pub fn keyboard_handler() {
    unsafe {
        let state = inb(KEYBOARD_STATE_PORT) & 1;
        if state == 1 {
            let mut key = inb(KEYBOARD_DATA_PORT);
            if key >> 7 == 0 {
                match key {
                    LEFT_SHIFT => SHIFT = true,
                    RIGHT_SHIFT => SHIFT = true,
                    _ => {
                        if SHIFT {
                            BUFFER.write(SHIFT_KEY_MAP[key as usize] as i32);
                        } else {
                            BUFFER.write(KEY_MAP[key as usize] as i32);
                        }
                    }
                }
            } else {
                key &= !(1<<7);
                if key == LEFT_SHIFT || key == RIGHT_SHIFT {
                    SHIFT = false;
                }
            }
        }
    }
}

pub fn getc() -> char {
    unsafe {
        let mut data = BUFFER.read();
        while data == -1 { 
            data = BUFFER.read();
        }
        return data as u8 as char;
    }
}

// Non-blocking call. Return 1 if a key is pressed
pub fn keypressed() -> bool {
    unsafe {
        BUFFER.count > 0
    }
}

impl CircBuffer {
    const fn new() -> CircBuffer {
        CircBuffer {
            buffer: [0;CIRC_BUFFER_SIZE],
            read: 0,
            write: 0,
            count: 0
        }
    }
    
    fn write(&mut self, data: i32) {
        self.buffer[self.write] = data;
        self.write = (self.write + 1) % CIRC_BUFFER_SIZE;
        if self.count < CIRC_BUFFER_SIZE {
            self.count += 1;
        }
    }
    
    fn read(&mut self) -> i32 {
        if self.count > 0 {
            let data = self.buffer[self.read];
            self.read = (self.read + 1) % CIRC_BUFFER_SIZE;
            self.count -= 1;
            return data;
        } else {
            return -1;
        }
    }
}