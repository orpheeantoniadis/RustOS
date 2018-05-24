use core::str;

pub fn bytes_to_str(bytes: &[u8]) -> &str {
    let mut cnt = 0;
    for &byte in bytes {
        if byte == 0 {
            break;
        }
        cnt += 1;
    }
    str::from_utf8(&bytes[0..cnt]).expect("Found invalid UTF-8")
}