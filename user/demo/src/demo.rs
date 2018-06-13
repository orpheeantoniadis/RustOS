#![no_std]

extern crate ulibc;
use ulibc::*;
use io::*;
use mem::*;

#[no_mangle]
pub extern fn main() {
    puts("Starting demo.\n");
    
    println!("\nIO demo :");
    println!("Executing hello app..");
    exec("hello");
    println!("Waiting on keypressed..");
    while keypressed() == 0 {}
    getc();
    println!("Waiting 1 sec..");
    sleep(1000);
    println!("Waiting on getc..");
    let key = getc();
    println!("{} pressed.", key as u8 as char);
    
    println!("\nFile system demo :");
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
    println!("Iterating all the files..", );
    let it = file_iterator();
    let mut bytes = [0;MAX_FILENAME_LENGTH];
    while file_next(&bytes[0], &it) != -1 {
        {
            let filename = bytes_to_str(&bytes);
            println!("{} {}", filename, file_stat(filename).size);
        }
        bytes = [0;MAX_FILENAME_LENGTH];
    }
    
    println!("\nMemory management demo :");
    unsafe {
        println!("Allocating 1M on the heap..", );
        let addr1 = malloc(0x100000);
        *(addr1 as *mut u8) = 42;
        println!("addr1 = 0x{:x}, [addr1] = 0x{:x}", addr1, *(addr1 as *mut u8));
        println!("Allocating 256 bytes on the heap..", );
        let addr2 = malloc(0x100);
        *(addr2 as *mut u8) = 0x42;
        println!("addr2 = 0x{:x}, [addr2] = 0x{:x}", addr2, *(addr2 as *mut u8));
        println!("Freeing all allocated memory..", );
        free(addr1);
        free(addr2);
    }
    println!("Ok.");
}