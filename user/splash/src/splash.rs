#![no_std]

extern crate ulibc;
use ulibc::*;
use io::*;

#[no_mangle]
pub extern fn main() {
    cursor_disable(true);
    clear();
    set_cursor(22,10);
    let fd = file_open("splash.txt") as u32;
    let mut data = [0;MAX_STR_LEN];
    file_read(fd, &mut data[0], MAX_STR_LEN as u32);
    for byte in bytes_to_str(&data).bytes() {
        putc(byte);
        if byte == b'\n' {
            let cursor = (0,0);
            get_cursor(&cursor.0, &cursor.1);
            set_cursor(22,cursor.1);
        }
    }
    file_close(fd);
    sleep(5000);
    clear();
    cursor_disable(false);
}