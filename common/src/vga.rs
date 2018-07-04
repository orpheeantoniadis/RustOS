pub const BUFFER_HEIGHT : usize = 25;
pub const BUFFER_WIDTH  : usize = 80;
pub const FG_COLOR      : Color = Color::Black;
pub const BG_COLOR      : Color = Color::White;

#[repr(u8)]
#[derive(Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub struct ColorAttribute(u8);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Character {
    pub ascii: u8,
    pub attribute: ColorAttribute,
}

pub type FrameBuffer = [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT];

impl Color {
    pub fn from_u32(color: u32) -> Color {
        match color {
            0x0 => Color::Black,
            0x1 => Color::Blue,
            0x2 => Color::Green,
            0x3 => Color::Cyan,
            0x4 => Color::Red,
            0x5 => Color::Magenta,
            0x6 => Color::Brown,
            0x7 => Color::LightGray,
            0x8 => Color::DarkGray,
            0x9 => Color::LightBlue,
            0xa => Color::LightGreen,
            0xb => Color::LightCyan,
            0xc => Color::LightRed,
            0xd => Color::Pink,
            0xe => Color::Yellow,
            0xf => Color::White,
            _ => Color::Black
        }
    }
    
    pub fn to_u32(color: Color) -> u32 {
        match color {
            Color::Black => 0x0,
            Color::Blue => 0x1,
            Color::Green => 0x2,
            Color::Cyan => 0x3,
            Color::Red => 0x4,
            Color::Magenta => 0x5,
            Color::Brown => 0x6,
            Color::LightGray => 0x7,
            Color::DarkGray => 0x8,
            Color::LightBlue => 0x9,
            Color::LightGreen => 0xa,
            Color::LightCyan => 0xb,
            Color::LightRed => 0xc,
            Color::Pink => 0xd,
            Color::Yellow => 0xe,
            Color::White => 0xf,
        }
    }
}

impl ColorAttribute {
    pub const fn new(background: Color, foreground: Color) -> ColorAttribute {
        ColorAttribute((background as u8) << 4 | (foreground as u8))
    }
}

impl Character {
    pub const fn null() -> Character {
        Character {
            ascii: 0,
            attribute: ColorAttribute::new(BG_COLOR, FG_COLOR)
        }
    }
    
    pub const fn new(ascii: u8, attribute: ColorAttribute) -> Character {
        Character {
            ascii: ascii,
            attribute: attribute
        }
    }
}