// Ascii charset hex
pub const NUL: char = '\0';
pub const BACKSPACE: char = 0x8 as char;
pub const NAK: char = 0x15 as char;
pub const E_ACUTE: char = 0x82 as char;
pub const A_GRAVE: char = 0x85 as char;
pub const C_CEDILLA: char = 0x87 as char;
pub const E_GRAVE: char = 0x8a as char;
pub const U_GRAVE: char = 0x97 as char;
pub const POUND: char = 0x9c as char;

// Keys codes
pub const CTRL: u8 = 29;
pub const LEFT_SHIFT: u8 = 42;
pub const RIGHT_SHIFT: u8 = 54;
pub const ALT: u8 = 56;
pub const CAPS_LOCK: u8 = 58;
pub const UP_KEY: u8 = 72;
pub const LEFT_KEY: u8 = 75;
pub const RIGHT_KEY: u8 = 77;
pub const DOWN_KEY: u8 = 80;
pub const LEFT_CMD: u8 = 91;
pub const RIGHT_CMD: u8 = 92;

pub const KEY_MAP: [char;93] = [
	NUL, NUL, '&', E_ACUTE, '"', '\'', '(', NAK, E_GRAVE, '!', C_CEDILLA, A_GRAVE, ')', '-', BACKSPACE,
    '\t', 'a', 'z', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '^', '$', '\n',
    NUL, 'q', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', U_GRAVE, '@',
    NUL, '`', 'w', 'x', 'c', 'v', 'b', 'n', ',', ';', ':', '=', NUL,
    NUL, NUL, ' ', NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, '<', NUL, NUL, NUL, NUL, NUL, NUL
];

pub const SHIFT_KEY_MAP: [char;93] = [
	NUL, NUL, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '°', '_', BACKSPACE,
    '\t', 'A', 'Z', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', NUL, '*', '\n',
    NUL, 'Q', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', '%', '#',
    NUL, POUND, 'W', 'X', 'C', 'V', 'B', 'N', '?', '.', '/', '+', NUL,
    NUL, NUL, ' ', NUL, ' ', NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL, NUL,
    NUL, NUL, NUL, NUL, NUL, '>', NUL, NUL, NUL, NUL, NUL, NUL
];