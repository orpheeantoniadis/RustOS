#![no_std]
#![macro_use]

extern crate ulibc;
use ulibc::*;

#[no_mangle]
pub extern fn main() {
    puts("Starting demo.\n");
    println!("Executing hello app..");
    exec("hello");
    // println!("Waiting on keypressed..");
    // while keypressed() == 0 {}
    println!("Waiting on getc..");
    let _ket = getc();
    println!("Opening file splash.txt..");
    let fd = file_open("splash.txt");
    if fd != -1 {
        println!("Reading file splash.txt..");
        let mut data = [0;MAX_STR_LEN];
        file_read(fd as u32, &mut data[0], MAX_STR_LEN as u32);
        println!("{}", bytes_to_str(&data));
        println!("Closing file splash.txt..");
        file_close(fd as u32);
    }
    println!("Ok.")
}