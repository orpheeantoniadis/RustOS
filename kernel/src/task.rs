#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use gdt::*;
use fs::*;

pub const TASKS_NB: usize = 8; 
pub const ADDR_SPACE: usize = 0x100000;
pub const STACK_SIZE: usize = 0x10000;

pub static mut INITIAL_TSS: Tss = Tss::new();
pub static mut INITIAL_TSS_KERNEL_STACK: [u8;STACK_SIZE] = [0;STACK_SIZE];
pub static mut TASKS: [Task;TASKS_NB] = [Task::new();TASKS_NB];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Task {
    pub tss: Tss,
    pub ldt: [GdtEntry;2],
    pub tss_selector: u16,
    pub addr: [u8;ADDR_SPACE],
    pub kernel_stack: [u8;STACK_SIZE],
    pub free: bool
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
        GDT[3] = GdtEntry::make_tss(&INITIAL_TSS as *const _ as u32, DPL_KERNEL);
        task_ltr(GDT[3].to_selector() as u16);
        
        for task in &mut TASKS {
            task.setup();
        }
    }
}

pub fn exec(filename: &str) -> i8 {
    let idx = free_task();
    if idx != -1 {
        unsafe {
            let idx = idx as usize;
            let fd = file_open(filename);
            if fd != -1 {
                let stat = Stat::new(filename);
                if file_read(fd, &mut TASKS[idx].addr[0], stat.size) != -1 {
                    TASKS[idx].tss.eip = 0;
                    TASKS[idx].tss.esp = ADDR_SPACE as u32;
                    TASKS[idx].tss.ebp = ADDR_SPACE as u32;
                    TASKS[idx].free = false;
                    task_switch(TASKS[idx].tss_selector as u16);
                    TASKS[idx].free = true;
                    return 0;
                }
            }
        }
    }
    return -1;
}

fn free_task() -> i8 {
    unsafe {
        let mut cnt = 0;
        for task in TASKS.iter() {
            if task.free {
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
            ldt: [GdtEntry::null();2],
            tss_selector: 0,
            addr: [0;ADDR_SPACE],
            kernel_stack: [0;STACK_SIZE],
            free: true
        }
    }
    
    unsafe fn setup(&mut self) {
        let idx = ((self as *mut _ as usize) - (&TASKS as *const _ as usize)) / size_of::<Task>();
        // Add the task's TSS and LDT to the GDT
    	GDT[GDT_SIZE + idx * 2] = GdtEntry::make_tss(&self.tss as *const _ as u32, DPL_KERNEL);
    	GDT[GDT_SIZE + idx * 2 + 1] = GdtEntry::make_ldt(
            &self.ldt as *const _ as u32, (size_of::<[GdtEntry;2]>() - 1) as u32, DPL_KERNEL
        );

    	// Define code and data segments in the LDT; both segments are overlapping
    	self.ldt[0] = GdtEntry::make_code_segment(
            &self.addr as *const _ as u32, ADDR_SPACE as u32 / 4096, DPL_USER
        );
    	self.ldt[1] = GdtEntry::make_data_segment(
            &self.addr as *const _ as u32, ADDR_SPACE as u32 / 4096, DPL_USER
        );

    	// Initialize the TSS fields
    	// The LDT selector must point to the task's LDT
        self.tss_selector = GDT[GDT_SIZE + idx * 2].to_selector() as u16;
    	self.tss.ldt_selector = GDT[GDT_SIZE + idx * 2 + 1].to_selector() as u16;

    	// Setup code and stack pointers
    	self.tss.eip = 0;
    	self.tss.esp = ADDR_SPACE as u32;
        self.tss.ebp = ADDR_SPACE as u32;

    	// Code and data segment selectors are in the LDT
        let cs = gdt_index_to_selector(0) | (DPL_USER | LDT_SELECTOR) as u32;
        let ds = gdt_index_to_selector(8) | (DPL_USER | LDT_SELECTOR) as u32;
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