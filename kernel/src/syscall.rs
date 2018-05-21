#![allow(dead_code)]
#![allow(unused_variables)]

use common::Syscall;

// System call handler: call the appropriate system call according to the nb argument.
// Called by the assembly code _syscall_handler
#[no_mangle]
pub extern fn syscall_handler(nb: Syscall, arg1: u32, arg2: u32, arg3: u32, arg4: u32, caller_tss_selector: u32) {

}