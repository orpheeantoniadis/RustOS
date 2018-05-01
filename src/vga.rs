#![allow(dead_code)]

pub static mut BUFFER: *mut FrameBuffer = 0xb8000 as *mut _;

pub const BUFFER_HEIGHT: usize =    25;
pub const BUFFER_WIDTH: usize =     80;

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

pub struct ColorAttribute(u8);
impl ColorAttribute {
    pub fn new(background: Color, foreground: Color) -> ColorAttribute {
        ColorAttribute((background as u8) << 4 | (foreground as u8))
    }
}

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
