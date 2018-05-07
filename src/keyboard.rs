#![allow(dead_code)]

use pio::*;

// Keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATE_PORT: u16 = 0x64;

const CIRC_BUFFER_SIZE: usize = 30;

// Keys codes
const NUL: char = '\0';
const BACKSPACE: char = 8 as char;

const KEY_MAP: [char;66] = [
	NUL, '@', '&', 'é', '"', '\'', '(', NUL, 'è', '!', 'ç', 'à', ')', '-', BACKSPACE,
    '\t', 'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '^', '$', '\n',
    NUL, 'q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'ù', '`',
    NUL, '<', 'w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '=', NUL,
    NUL, NUL, NUL, NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL
];

const SHIFT_KEY_MAP: [char;66] = [
	NUL, '#', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '°', '_', BACKSPACE,
    '\t', 'A', 'Z', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '¨', '*', '\n',
    NUL, 'Q', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', '%', '£',
    NUL, '>', 'W', 'X', 'C', 'V', 'B', 'N', '?', '.', '/', '+', NUL,
    NUL, NUL, NUL, NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL
];

static mut BUFFER: CircBuffer = CircBuffer::new();

pub fn keyboard_handler() {
    if keypressed() {
        let key = unsafe { inb(KEYBOARD_DATA_PORT) };
        if key >> 7 == 0 {
            unsafe { BUFFER.write(KEY_MAP[key as usize] as i32); }
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
fn keypressed() -> bool {
    let state = unsafe { inb(KEYBOARD_STATE_PORT) & 1 };
    if state == 1 {
        true
    } else {
        false
    }
}

struct CircBuffer {
    buffer: [i32;CIRC_BUFFER_SIZE],
    read: usize,
    write: usize,
    count: usize
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