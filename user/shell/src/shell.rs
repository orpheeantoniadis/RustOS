#![no_std]
#![macro_use]

extern crate ulibc;
use ulibc::*;

const MAX_CMD_LEN: usize = MAX_FILENAME_LENGTH;

fn cat(filename: &str) {
    let fd = file_open(filename);
    if fd != -1 {
        let mut data = [0;MAX_STR_LEN];
        file_read(fd as u32, &mut data[0], MAX_STR_LEN as u32);
        println!("{}", bytes_to_str(&data));
        file_close(fd as u32);
    }
}

fn ls() {
    let it = file_iterator();
    let mut bytes = [0;MAX_FILENAME_LENGTH];
    while file_next(&bytes[0], &it) != -1 {
        {
            let filename = bytes_to_str(&bytes);
            println!("{} {}", filename, file_stat(filename).size);
        }
        bytes = [0;MAX_FILENAME_LENGTH];
    }
}

fn help() {
	puts("\n");
	puts("ls        : list files present in the file system\n");
	puts("cat FILE  : dump the content of FILE to the screen\n");
	puts("run PROG  : execute the program PROG.\n");
	puts("sleep N   : sleep the specified number of milliseconds\n");
	puts("exit      : exit the shell\n");
}

fn read_cmd(cmd: *mut u8) {
    let mut key = 0;
    let mut cnt = 0;
    while key != b'\n' {
        if key == 0x8 {
            if cnt > 0 {
                cnt -= 1;
                print!("{}", key as char);
            }
        } else if key != b'\t' && key != b'\0' {
            unsafe { *cmd.offset(cnt as isize) = key; }
            cnt += 1;
            print!("{}", key as char);
        }
        key = getc() as u8;
    }
}

#[no_mangle]
pub extern fn main() {
    let mut cmd : [u8;MAX_CMD_LEN] = [0;MAX_CMD_LEN];
    loop {
        print!("$ ");
        read_cmd(&mut cmd[0]);
        println!();
        let mut s = String::new(bytes_to_str(&cmd));
        println!("{}", s.to_string());
        cmd = [0;MAX_CMD_LEN]
    }
}