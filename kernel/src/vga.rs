#![allow(dead_code)]
#![macro_use]

use core::fmt::{Error, Write, Arguments};
use pio::*;
use common::*;

const TAB_SIZE: usize = 4;

static mut SCREEN: Screen = Screen {
    buffer: 0xC00B8000 as *mut _,
    attribute: ColorAttribute::new(BG_COLOR, FG_COLOR),
    cursor_x: 0,
    cursor_y: 0
};

struct Screen {
    buffer: *mut FrameBuffer,
    attribute: ColorAttribute,
    cursor_x: usize,
    cursor_y: usize
}

macro_rules! print {
    ($($arg:tt)*) => (vga_write_fmt(format_args!($($arg)*)));
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn vga_init(background: Color, foreground: Color) {
    unsafe {
        SCREEN.set_color(background, foreground);
        SCREEN.clear();
    }
}

pub fn vga_write_byte(byte: u8) {
    unsafe {
        SCREEN.write_byte(byte);
    }
}

pub fn vga_write_str(s: &str) {
    unsafe {
        SCREEN.write_str(s);
    }
}

pub fn vga_write_fmt(args: Arguments) {
    unsafe {
        SCREEN.write_fmt(args).ok();
    }
}

pub fn vga_clear() {
    unsafe { SCREEN.clear(); }
}

pub fn vga_set_cursor(x: usize, y: usize) {
    unsafe { SCREEN.set_cursor(x, y); }
}

pub fn vga_get_cursor() -> (usize, usize) {
    unsafe {
        return (SCREEN.cursor_x, SCREEN.cursor_y);
    }
}

pub fn vga_set_color(background: Color, foreground: Color) {
    unsafe { SCREEN.set_color(background, foreground); }
}

pub fn vga_copy_scr(scr: *const FrameBuffer) {
    unsafe {
        for i in 0..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                (*SCREEN.buffer)[i][j] = (*scr)[i][j];
            }
        }
    }
}

impl Write for Screen {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.write_str(s);
        Ok(())
    }
}

impl Screen {    
    fn clear(&mut self) {
        unsafe {
            for i in 0..BUFFER_HEIGHT {
                for j in 0..BUFFER_WIDTH {
                    (*self.buffer)[i][j] = Character::new(0, self.attribute);
                }
            }
            self.set_cursor(0, 0);
        }
    }
    
    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.cursor_x >= BUFFER_WIDTH {
            if self.cursor_y == BUFFER_HEIGHT-1 {
                self.shift_up();
                self.cursor_x = 0;
            } else {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
        }
        match byte {
            b'\0' => return,
            b'\n' => (),
            b'\t' => {
                for _i in 0..TAB_SIZE {
                    self.write_byte(b' ');
                }
            }
            0x8 => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = BUFFER_WIDTH-1;
                }
                unsafe { (*self.buffer)[self.cursor_y][self.cursor_x] = Character::new(b'\0', self.attribute); }
            }
            _ => {
                unsafe { (*self.buffer)[self.cursor_y][self.cursor_x] = Character::new(byte, self.attribute); }
                self.cursor_x += 1;
            }
        }
        move_cursor(self.get_pos());
    }
    
    fn write_str(&mut self, buf: &str) {
        for byte in buf.bytes() {
            self.write_byte(byte);
        }
    }
    
    fn shift_up(&mut self) {
        unsafe {
            for i in 0..BUFFER_HEIGHT-1 {
                for j in 0..BUFFER_WIDTH {
                    (*self.buffer)[i][j] = (*self.buffer)[i+1][j];
                }
            }
            for j in 0..BUFFER_WIDTH {
                (*self.buffer)[BUFFER_HEIGHT-1][j] = Character::new(0, self.attribute);
            }
        }
    }
    
    fn get_pos(&mut self) -> u16 {
        (self.cursor_y*BUFFER_WIDTH+self.cursor_x) as u16
    }
    
    fn get_color(&mut self) -> ColorAttribute {
        return self.attribute;
    }
    
    fn set_color(&mut self, background: Color, foreground: Color) {
        self.attribute = ColorAttribute::new(background, foreground);
    }
    
    fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
        move_cursor(self.get_pos());
    }
}