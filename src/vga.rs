#![allow(dead_code)]
#![macro_use]

pub use core::fmt::Write;
use core::fmt;
use core::ptr::Unique;
use spin::Mutex;
use pio::*;

pub static SCREEN: Mutex<Screen> = Mutex::new(Screen {
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
    attribute: ColorAttribute::new(Color::Black, Color::White),
    cursor_x: 0,
    cursor_y: 0
});

pub const BUFFER_HEIGHT: usize =    25;
pub const BUFFER_WIDTH: usize =     80;

macro_rules! print {
    ($($arg:tt)*) => ({
        SCREEN.lock().write_fmt(format_args!($($arg)*)).expect("Error while formatting !");
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn vga_init(background: Color, foreground: Color) {
    SCREEN.lock().set_color(background, foreground);
    SCREEN.lock().clear();
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
    buffer: Unique<FrameBuffer>,
    attribute: ColorAttribute,
    cursor_x: usize,
    cursor_y: usize
}
impl Screen {    
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..BUFFER_HEIGHT {
                for j in 0..BUFFER_WIDTH {
                    self.buffer.as_mut()[i][j] = Character::new(0, self.attribute);
                }
            }
            self.set_cursor(0, 0);
        }
    }
    
    pub fn write(&mut self, buf: &str) {
        for byte in buf.bytes() {
            if self.cursor_y == BUFFER_HEIGHT-1 {    
                if byte == b'\n' {
                    self.shift_up();
                    self.cursor_x = 0;
                } else {
                    if self.cursor_x >= BUFFER_WIDTH {
                        self.shift_up();
                        self.cursor_x = 0;
                    }
                    unsafe { self.buffer.as_mut()[self.cursor_y][self.cursor_x] = Character::new(byte, self.attribute); }
                    self.cursor_x += 1;
                }
            } else {
                if byte == b'\n' {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                } else {
                    if self.cursor_x >= BUFFER_WIDTH {
                        self.cursor_x = 0;
                        self.cursor_y += 1;
                    }
                    unsafe { self.buffer.as_mut()[self.cursor_y][self.cursor_x] = Character::new(byte, self.attribute); }
                    self.cursor_x += 1;
                }
            }
        }
        unsafe { move_cursor(self.get_pos()); }
    }
    
    pub fn shift_up(&mut self) {
        unsafe {
            for i in 0..BUFFER_HEIGHT-1 {
                for j in 0..BUFFER_WIDTH {
                    self.buffer.as_mut()[i][j] = self.buffer.as_mut()[i+1][j];
                }
            }
            for j in 0..BUFFER_WIDTH {
                self.buffer.as_mut()[BUFFER_HEIGHT-1][j] = Character::new(0, self.attribute);
            }
        }
    }
    
    pub fn get_pos(&mut self) -> u16 {
        (self.cursor_y*BUFFER_WIDTH+self.cursor_x) as u16
    }
    
    pub fn get_color(&mut self) -> ColorAttribute {
        return self.attribute;
    }
    
    pub fn set_color(&mut self, background: Color, foreground: Color) {
        self.attribute = ColorAttribute::new(background, foreground);
    }
    
    pub fn set_cursor(&mut self, x: usize, y: usize) {
        unsafe {
            self.cursor_x = x;
            self.cursor_y = y;
            move_cursor(self.get_pos());
        }
    }
}
impl fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}