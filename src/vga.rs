#![allow(dead_code)]
#![macro_use]

use core::fmt;
pub use core::fmt::Write;

pub static mut SCREEN: Screen = Screen {
    buffer: 0xb8000 as *mut _,
    attribute: ColorAttribute::new(Color::Black, Color::White),
    pos : 0
};

pub const BUFFER_HEIGHT: usize =    25;
pub const BUFFER_WIDTH: usize =     80;

macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe { 
            match SCREEN.write_fmt(format_args!($($arg)*)) {
                Ok(_) => (),
                Err(_) => ()
            }
        };
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    unsafe { SCREEN.clear() }
}

type FrameBuffer = [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT];

#[repr(u8)]
pub enum Color {
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGray  = 0x7,
    DarkGray   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xa,
    LightCyan  = 0xb,
    LightRed   = 0xc,
    Pink       = 0xd,
    Yellow     = 0xe,
    White      = 0xf,
}

#[derive(Clone, Copy)]
pub struct ColorAttribute(u8);
impl ColorAttribute {
    pub const fn new(background: Color, foreground: Color) -> ColorAttribute {
        ColorAttribute((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Character {
    ascii: u8,
    attribute: ColorAttribute,
}
impl Character {
    pub fn new(ascii: u8, attribute: ColorAttribute) -> Character {
        Character {
            ascii: ascii,
            attribute: attribute
        }
    }
}

pub struct Screen {
    buffer: *mut FrameBuffer,
    attribute: ColorAttribute,
    pos: usize
}
impl Screen {
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..BUFFER_HEIGHT {
                for j in 0..BUFFER_WIDTH {
                    (*self.buffer)[i][j] = Character::new(0, self.attribute);
                }
            }
        }
    }
    
    pub fn write(&mut self, buf: &str) {
        unsafe {
            for byte in buf.bytes() {
                if byte == b'\n' {
                    self.shift_up();
                    self.pos = 0;
                } else {
                    if self.pos >= BUFFER_WIDTH {
                        self.shift_up();
                        self.pos = 0;
                    }
                    (*self.buffer)[BUFFER_HEIGHT-1][self.pos] = Character::new(byte, self.attribute);
                    self.pos+=1;
                }
            }
        }
    }
    
    pub fn shift_up(&mut self) {
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
}
impl fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}