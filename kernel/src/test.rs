#![allow(dead_code)]

use gdt::*;
use timer::*;

#[test]
pub fn check_gdt_size() {
    use core::mem::size_of;
    assert_eq!(size_of::<Gdt>(), 24);
}

#[test]
pub fn check_timer_init() {
    timer_init(0);
    assert_eq!(get_freq(), MIN_FREQ);
    timer_init(42);
    assert_eq!(get_freq(), 42);
    timer_init(MAX_FREQ+1);
    assert_eq!(get_freq(), MAX_FREQ);
}