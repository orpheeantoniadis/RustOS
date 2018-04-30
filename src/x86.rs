#![allow(dead_code)]

// Privilege levels
pub const  DPL_USER: u8 = 0x3;
pub const  DPL_KERNEL: u8 = 0x0;

// Selectors
pub const  LDT_SELECTOR: u8 = 0x4;

// Descriptor types for code and data segments
pub const  TYPE_DATA_READONLY: u8 = 1;
pub const  TYPE_DATA_READWRITE: u8 = 3;
pub const  TYPE_CODE_EXECONLY: u8 = 9;
pub const  TYPE_CODE_EXECREAD: u8 = 11;

// Descriptor types for system segments and gates
pub const  TYPE_LDT: u8 = 2;
pub const  TYPE_TASK_GATE: u8 = 5;
pub const  TYPE_TSS: u8 = 9;
pub const  TYPE_CALL_GATE: u8 = 12;
pub const  TYPE_TRAP_GATE: u8 = 15;
pub const  TYPE_INTERRUPT_GATE: u8 = 14;

// Descriptor system bit (S)
// For code or data segments
pub const  S_CODE_OR_DATA: u8 = 1;
// For TSS segment, LDT, call gate, interrupt gate, trap gate, task gate
pub const  S_SYSTEM: u8 = 0;

// D/B bit
pub const  DB_SEG: u8 = 1;
pub const  DB_SYS: u8 = 0;

// kernel code and data selectors in the GDT
pub const  GDT_KERNEL_CODE_SELECTOR: u8 = 0x08;
pub const  GDT_KERNEL_DATA_SELECTOR: u8 = 0x10;

// Disable hardware interrupts.
fn cli() {
    unsafe { asm!("cli"); }
}

// Enable hardware interrupts.
fn sti() {
    unsafe { asm!("sti"); }
}

// Halt the processor.
// External interrupts wake up the CPU, hence the cli instruction.
fn halt() {
    unsafe {
    	loop {
            asm!("cli\nhlt");
        }
    }
}