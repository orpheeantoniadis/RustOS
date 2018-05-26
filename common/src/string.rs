use core::slice::from_raw_parts;
use core::str::from_utf8;

pub const MAX_STR_LEN : usize = 500;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct String {
    pub bytes_ptr: u32,
    pub len: usize
}

pub fn bytes_to_str(bytes: &[u8]) -> &str {
    let mut cnt = 0;
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        cnt += 1;
    }
    from_utf8(&bytes[0..cnt]).expect("Found invalid UTF-8")
}

impl String {
    pub fn new(s: &str) -> String {
        unsafe {
            String {
                bytes_ptr: &from_raw_parts(s.as_ptr(), s.len())[0] as *const u8 as u32,
                len: s.len()
            }
        }
    }
    
    pub fn to_string(&mut self) -> &str {
        unsafe {
            let addr = self.bytes_ptr as *const u8;
            let bytes = &*(addr as *const [u8;MAX_STR_LEN]);
            from_utf8(&bytes[0..self.len]).expect("Found invalid UTF-8")
        }
    }
    
    pub fn offset(&mut self, offset: u32) {
        self.bytes_ptr += offset;
    }
}