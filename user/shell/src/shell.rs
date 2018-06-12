#![no_std]

extern crate ulibc;
use ulibc::*;
use io::*;
use core::str::FromStr;

const MAX_CMD_LEN: usize = MAX_FILENAME_LENGTH;

fn cat(filename: &str) {
    let fd = file_open(filename);
    if fd != -1 {
        let mut data = [0;MAX_STR_LEN];
        while file_read(fd as u32, &mut data[0], MAX_STR_LEN as u32) != 0 {
            {
                let content = bytes_to_str(&data);
                if content == "\0" {
                    println!("cat: {}: Not a text file", filename);
                    break;
                } else {
                    println!("{}", content);
                }
            }
            data = [0;MAX_STR_LEN];
        }
        file_close(fd as u32);
    } else {
        println!("cat: {}: No such file or directory", filename);
    }
}

fn help() {
	puts("\n");
	puts("ls           : list files present in the file system\n");
	puts("cat <file>   : dump the content of <file> to the screen\n");
    puts("clear        : clear the screen\n");
	puts("<prog>       : execute the program <prog>.\n");
	puts("sleep <ms>   : sleep the specified number of milliseconds\n");
	puts("exit         : exit the shell\n");
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

fn read_cmd(cmd: *mut u8) {
    let mut key = 0;
    let mut cnt = 0;
    while key != b'\n' {
        if key == 0x8 {
            if cnt > 0 {
                cnt -= 1;
                unsafe { *cmd.offset(cnt as isize) = 0; }
                putc(key);
            }
        } else if key != b'\t' && key != b'\0' {
            unsafe { *cmd.offset(cnt as isize) = key; }
            cnt += 1;
            putc(key);
        }
        key = getc();
    }
}

#[no_mangle]
pub extern fn main() {
    loop {
        let mut cmd : [u8;MAX_CMD_LEN] = [0;MAX_CMD_LEN];
        print!("$ ");
        read_cmd(&mut cmd[0]);
        println!();
        let mut s = String::new(bytes_to_str(&cmd));
        let mut args = s.to_string().split_whitespace();
        match args.next() {
            Some(cmd) => {
                let arg = match args.next() {
                    Some(arg) => arg,
                    _ => ""
                };
                match cmd {
                    "cat"   => cat(arg),
                    "clear" => clear(),
                    "exit"  => break,
                    "help"  => help(),
                    "ls"    => ls(),
                    "sleep" => {
                        let ms = match u32::from_str(arg) {
                            Ok(num) => num,
                            Err(_) => {
                                println!("sleep: invalid time interval '{}'", arg);
                                continue;
                            }
                        };
                        println!("Sleeping for {}ms..", ms);
                        sleep(ms);
                    }
                    _ => { exec(cmd); }
                }
            }
            _ => continue
        }
    }
    println!("exit");
}