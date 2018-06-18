// extern crate alloc;
use io::*;

// #[global_allocator]
// static mut ALLOCATOR: Allocator = Allocator {};

// #[derive(Debug)]
// pub struct Allocator {}

pub fn malloc(_size: usize) -> u32 {
    unsafe {
        syscall(Syscall::AllocFrame, 0, 0, 0, 0) as u32
    }
}

pub fn free(addr: u32) {
    unsafe {
        syscall(Syscall::FreeFrame, addr, 0, 0, 0);
    }
}

// unsafe impl<'a> alloc::heap::Alloc for &'a Allocator {
//     unsafe fn alloc(&mut self, layout:  alloc::heap::Layout) -> Result<*mut u8,  alloc::heap::AllocErr> {
//         let addr = malloc(layout.size());
// 
//         if addr > 0 {
//             Ok(addr as *mut u8)
//         } else {
//             Err(alloc::heap::AllocErr::Exhausted{ request: layout })
//         }
//     }
// 
//     unsafe fn dealloc(&mut self, ptr: *mut u8, layout: alloc::heap::Layout) {
//         free(ptr as u32);
//     }
// }