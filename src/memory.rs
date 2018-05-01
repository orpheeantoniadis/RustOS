#![allow(dead_code)]

pub unsafe fn memset(dst: *mut u8, src: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *dst.offset(i as isize) = src as u8;
    }
    return dst;
}