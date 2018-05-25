use core::str::from_utf8;

const MAX_STR_LEN : usize = 100;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct String {
    pub s: *const u8,
    pub len: usize
}

impl String {
    pub fn new(s: &str, len: usize) -> String {
        String {s: s.as_ptr(), len: len}
    }
    
    pub fn to_string(&mut self) -> &str {
        unsafe {
            let bytes = &*(self.s as *const [u8;MAX_STR_LEN]);
            from_utf8(&bytes[0..self.len]).expect("Found invalid UTF-8")
        }
    }
}