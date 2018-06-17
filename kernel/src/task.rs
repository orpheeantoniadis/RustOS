#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use gdt::*;
use paging::*;
use fs::*;
use vga::*;
use kheap::*;
use common::*;

pub const TASKS_NB: usize = 8; 
pub const STACK_SIZE: usize = 0x10000;

pub static mut INITIAL_TSS: Tss = Tss::new();
pub static mut INITIAL_TSS_KERNEL_STACK: [u8;STACK_SIZE] = [0;STACK_SIZE];
pub static mut TASKS: [Task;TASKS_NB] = [Task::new();TASKS_NB];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Task {
    pub tss: Tss,
    pub tss_selector: u16,
    pub kernel_stack: [u8;STACK_SIZE],
    pub is_free: bool,
    pub pd: PageDirectory
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Tss {
	previous_task_link: u16, reserved0: u16,
	esp0: u32,
	ss0: u16, reserved1: u16,
	esp1: u32,
	ss1: u16, reserved2: u16,
	esp2: u32,
	ss2: u16, reserved3: u16,
	cr3: u32,
	eip: u32, eflags: u32, eax: u32, ecx: u32, edx: u32,
    ebx: u32, esp: u32, ebp: u32, esi: u32, edi: u32,
	es: u16, reserved4: u16,
	cs: u16, reserved5: u16,
	ss: u16, reserved6: u16,
	ds: u16, reserved7: u16,
	fs: u16, reserved8: u16,
	gs: u16, reserved9: u16,
	ldt_selector: u16, reserved10: u16,
	reserved11: u16,
	iomap_base_addr: u16  // adresse (relative to byte 0 of the TSS) of the IO permission bitmap
}

extern "C" {
    fn task_ltr(tss_selector: u16);
    fn task_switch(tss_selector: u16);
}

pub fn tasks_init() {
    unsafe {
        INITIAL_TSS.ss0 = GDT_KERNEL_DATA_SELECTOR as u16;
        INITIAL_TSS.esp0 = &INITIAL_TSS_KERNEL_STACK as *const _ as u32 + STACK_SIZE as u32;
        INITIAL_TSS.cr3 = phys!(INITIAL_PD.tables as u32);
        GDT[5] = GdtEntry::make_tss(&INITIAL_TSS as *const _ as u32, DPL_KERNEL);
        task_ltr(GDT[5].to_selector() as u16);
        
        for task in &mut TASKS {
            task.setup();
        }
    }
}

pub fn exec(filename: &str) -> i8 {
    let idx = free_task();
    if idx != -1 {
        unsafe {
            let fd = file_open(filename);
            if fd != -1 {
                let stat = Stat::new(filename);
                if file_type(fd) != TYPE_EXEC {
                    println!("exec: {}: not an executable", filename);
                    return -1;
                }
                // Create new directory using initial directory 
                let cr3 = get_cr3();
                if cr3 != phys!(INITIAL_PD.tables as u32) {
                    load_directory(phys!(INITIAL_PD.tables as u32));
                }
                TASKS[idx as usize].pd = INITIAL_PD.new_directory();
                *TASKS[idx as usize].pd.tables = (*INITIAL_PD.tables).clone();
                load_directory(phys!(TASKS[idx as usize].pd.tables as u32));
                
                // Alloc frames starting at address 0
                // Additional frames are allocated for the stack
                let code_addr = kmalloc(stat.size);
                let stack_addr = kmalloc(STACK_SIZE);
                file_read(fd, code_addr as *mut u8, stat.size);
                
                // Setup task with page directory previously allocated
                TASKS[idx as usize].is_free = false;
                TASKS[idx as usize].tss.eip = 0;
                TASKS[idx as usize].tss.esp = stack_addr;
                TASKS[idx as usize].tss.ebp = stack_addr;
                TASKS[idx as usize].tss.cr3 = phys!(TASKS[idx as usize].pd.tables as u32);
                
                // task_switch(TASKS[idx as usize].tss_selector as u16);
                // re-load original cr3 and free memory
                TASKS[idx as usize].is_free = true;
                load_directory(cr3);
                kfree(stack_addr);
                kfree(code_addr);
                TASKS[idx as usize].pd.free();
                return 0;
            } else {
                println!("exec: {}: not found", filename);
            }
        }
    } else {
        println!("exec: no free task slot found");
    }
    return -1;
}

fn free_task() -> i8 {
    unsafe {
        let mut cnt = 0;
        for task in TASKS.iter() {
            if task.is_free {
                return cnt;
            }
            cnt += 1;
        }
        return -1;
    }
}

impl Task {
    const fn new() -> Task {
        Task {
            tss: Tss::new(),
            tss_selector: 0,
            kernel_stack: [0;STACK_SIZE],
            is_free: true,
            pd: PageDirectory::null()
        }
    }
    
    unsafe fn setup(&mut self) {
        let idx = ((self as *mut _ as usize) - (&TASKS as *const _ as usize)) / size_of::<Task>();
        // Add the task's TSS to the GDT
        let tss = &self.tss as *const _ as u32;
    	GDT[GDT_SIZE + idx] = GdtEntry::make_tss(tss, DPL_KERNEL);

    	// Initialize the TSS fields
        self.tss_selector = GDT[GDT_SIZE + idx].to_selector() as u16;

    	// Code and data segment selectors are in the LDT
        let cs = (GDT_USER_CODE_SELECTOR | DPL_USER) as u32;
        let ds = (GDT_USER_DATA_SELECTOR | DPL_USER) as u32;
    	self.tss.cs = cs as u16;
    	self.tss.ds = ds as u16;
        self.tss.es = self.tss.ds;
        self.tss.fs = self.tss.ds;
        self.tss.gs = self.tss.ds;
        self.tss.ss = self.tss.ds;
    	self.tss.eflags = 512;  // Activate hardware interrupts (bit 9)

    	// Task's kernel stack
    	self.tss.ss0 = GDT_KERNEL_DATA_SELECTOR as u16;
    	self.tss.esp0 = (&self.kernel_stack as *const _ as usize + STACK_SIZE) as u32;
    }
}

impl Tss {
    const fn new() -> Tss {
        Tss {
            previous_task_link: 0, reserved0: 0,
        	esp0: 0,
        	ss0: 0, reserved1: 0,
        	esp1: 0,
        	ss1: 0, reserved2: 0,
        	esp2: 0,
        	ss2: 0, reserved3: 0,
        	cr3: 0,
        	eip: 0, eflags: 0, eax: 0, ecx: 0, edx: 0,
            ebx: 0, esp: 0, ebp: 0, esi: 0, edi: 0,
        	es: 0, reserved4: 0,
        	cs: 0, reserved5: 0,
        	ss: 0, reserved6: 0,
        	ds: 0, reserved7: 0,
        	fs: 0, reserved8: 0,
        	gs: 0, reserved9: 0,
        	ldt_selector: 0, reserved10: 0,
        	reserved11: 0,
        	iomap_base_addr: 0
        }
    }
}