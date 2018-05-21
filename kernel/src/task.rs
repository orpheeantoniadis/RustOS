#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use gdt::*;

const TASKS_NB: usize = 8; 
const ADDR_SPACE: usize = 0x100000;
const STACK_SIZE: usize = 65536;

static mut tasks: [Task;TASKS_NB] = [Task::new();TASKS_NB];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Task {
    tss: Tss,
    ldt: [GdtEntry;2],
    gdt_tss_sel: u32,
    gdt_ldt_sel: u32,
    addr_space: [u8;ADDR_SPACE],
    ldt_code_idx: usize,
    ldt_data_idx: usize,
    limit: u32,
    kernel_stack: [u8;STACK_SIZE]
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

// Setup GDT index 4 to store the task's TSS descriptor
// Setup GDT index 5 to store the task's LDT descriptor
pub fn task_init(idx: usize) {
	// Add the task's TSS and LDT to the GDT
    unsafe {
    	GDT[4] = GdtEntry::make_tss(&tasks[idx].tss, DPL_KERNEL);
    	GDT[5] = GdtEntry::make_ldt(&tasks[idx].ldt as *const _ as u32, (size_of::<[GdtEntry;2]>() - 1) as u32, DPL_KERNEL);
    	tasks[idx].gdt_tss_sel = GDT[4].to_selector();
    	tasks[idx].gdt_ldt_sel = GDT[5].to_selector();

    	// Define code and data segments in the LDT; both segments are overlapping
    	tasks[idx].ldt[tasks[idx].ldt_code_idx] = GdtEntry::make_code_segment(
            &ADDR_SPACE as *const _ as u32, tasks[idx].limit / 4096, DPL_USER
        );
    	tasks[idx].ldt[tasks[idx].ldt_data_idx] = GdtEntry::make_data_segment(
            &ADDR_SPACE as *const _ as u32, tasks[idx].limit / 4096, DPL_USER
        );

    	// Initialize the TSS fields
    	// The LDT selector must point to the task's LDT
    	tasks[idx].tss.ldt_selector = tasks[idx].gdt_ldt_sel as u16;

    	// Setup code and stack pointers
    	tasks[idx].tss.eip = 0;
    	tasks[idx].tss.esp = tasks[idx].limit;
        tasks[idx].tss.ebp = tasks[idx].limit;

    	// Code and data segment selectors are in the LDT
    	tasks[idx].tss.cs = (gdt_index_to_selector(tasks[idx].ldt_code_idx as u32) | (DPL_USER | LDT_SELECTOR) as u32) as u16;
    	tasks[idx].tss.ds = (gdt_index_to_selector(tasks[idx].ldt_data_idx as u32) | (DPL_USER | LDT_SELECTOR) as u32) as u16;
        tasks[idx].tss.es = tasks[idx].tss.ds;
        tasks[idx].tss.fs = tasks[idx].tss.ds;
        tasks[idx].tss.gs = tasks[idx].tss.ds;
        tasks[idx].tss.ss = tasks[idx].tss.ds;
    	tasks[idx].tss.eflags = 512;  // Activate hardware interrupts (bit 9)

    	// Task's kernel stack
    	tasks[idx].tss.ss0 = GDT_KERNEL_DATA_SELECTOR as u16;
    	tasks[idx].tss.esp0 = (&tasks[idx].kernel_stack as *const _ as usize + size_of::<[u8;8192]>()) as u32;
    }
}

impl Task {
    const fn new() -> Task {
        Task {
            tss: Tss::new(),
            ldt: [GdtEntry::null();2],
            gdt_tss_sel: 0,
            gdt_ldt_sel: 0,
            addr_space: [0;ADDR_SPACE],
            ldt_code_idx: 0,            // Index of code segment descriptor in the LDT
            ldt_data_idx: 1,            // Index of data segment descriptor in the LDT
            limit: ADDR_SPACE as u32,   // Limit for both code and data segments
            kernel_stack: [0;STACK_SIZE]
        }
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