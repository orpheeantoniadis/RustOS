#![no_std]
#![macro_use]

extern crate ulibc;
use ulibc::*;

#[no_mangle]
pub extern fn main() {
    // puts!("Starting demo.\n");
    let test = 42;
    print!("Coucou {}", test);
    // puts!("Executing hello app..\n");
    // exec!("hello");
    // puts!("\nWaiting on keypressed..\n");
    // while keypressed() == 0 {}
    // puts!("Waiting on getc..\n");
    // let _ket = getc();
    // puts!("Opening file splash.txt..\n");
    // let fd = file_open!("splash.txt");
    // puts!("Reading file splash.txt..\n");
    // let mut data = [0;300];
    // file_read(fd as u32, &mut data[0], 300);
    // println!("{}", bytes_to_str(&data));
    // puts!("Closing file splash.txt..\n");
    // file_close(fd as u32);
    // puts!("Ok.")
}