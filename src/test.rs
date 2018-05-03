#![allow(dead_code)]

use gdt::*;

#[test]
pub fn check_gdt_size() {
    use core::mem::size_of;
    assert_eq!(size_of::<GdtTable>(), 24);
}