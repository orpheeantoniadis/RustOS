#![allow(dead_code)]

use pio::*;

// Keyboard ports
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATE_PORT: u16 = 0x64;

const CIRC_BUFFER_SIZE: usize = 30;

// Ascii charset hex
const NUL: char = '\0';
const BACKSPACE: char = 0x8 as char;
const NAK: char = 0x15 as char;
const E_ACUTE: char = 0x82 as char;
const A_GRAVE: char = 0x85 as char;
const C_CEDILLA: char = 0x87 as char;
const E_GRAVE: char = 0x8a as char;
const U_GRAVE: char = 0x97 as char;
const POUND: char = 0x9c as char;

// Keys codes
const CTRL: u8 = 29;
const LEFT_SHIFT: u8 = 42;
const RIGHT_SHIFT: u8 = 54;
const ALT: u8 = 56;
const CAPS_LOCK: u8 = 58;
const UP_KEY: u8 = 72;
const LEFT_KEY: u8 = 75;
const RIGHT_KEY: u8 = 77;
const DOWN_KEY: u8 = 80;
const LEFT_CMD: u8 = 91;
const RIGHT_CMD: u8 = 92;

const KEY_MAP: [char;93] = [
	NUL, NUL, '&', E_ACUTE, '"', '\'', '(', NAK, E_GRAVE, '!', C_CEDILLA, A_GRAVE, ')', '-', BACKSPACE,
    '\t', 'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '^', '$', '\n',
    NUL, 'q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', U_GRAVE, '@',
    NUL, '`', 'w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '=', NUL,
    NUL, NUL, ' ', NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, '<', NUL, NUL, NUL, NUL, NUL, NUL
];

const SHIFT_KEY_MAP: [char;93] = [
	NUL, NUL, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '°', '_', BACKSPACE,
    '\t', 'A', 'Z', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', NUL, '*', '\n',
    NUL, 'Q', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', '%', '#',
    NUL, POUND, 'W', 'X', 'C', 'V', 'B', 'N', '?', '.', '/', '+', NUL,
    NUL, NUL, ' ', NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, '>', NUL, NUL, NUL, NUL, NUL, NUL
];

static mut BUFFER: CircBuffer = CircBuffer::new();
static mut SHIFT: bool = false;

pub fn keyboard_handler() {
    unsafe {
        if keypressed() {
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
unsafe fn keypressed() -> bool {
    let state = inb(KEYBOARD_STATE_PORT) & 1;
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