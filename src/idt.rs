#![allow(dead_code)]

// Initialize the IDT
pub fn idt_init() {
    unimplemented!()
}

// Exception handler
#[no_mangle]
pub extern fn exception_handler(regs: *mut Regs) {
    unimplemented!()
}

// Structure of an IDT descriptor. There are 3 types of descriptors:
// a task-gate, an interrupt-gate, and a trap-gate.
// See 5.11 of Intel 64 & IA32 architectures software developer's manual for more details.
// For task gates, offset must be 0.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset15_0: u16,   // only used by trap and interrupt gates
	selector: u16,     // segment selector for trap and interrupt gates; TSS segment selector for task gates
	reserved: u8,
	flags: u8,
	offset31_16: u16   // only used by trap and interrupt gates
}
impl IdtEntry {
    fn null() -> IdtEntry {
        IdtEntry {
            offset15_0: 0,
            selector: 0,
            reserved: 0,
            flags: 0,
            offset31_16: 0
        }
    }
    
    // Build and return an IDT entry.
    // selector is the code segment selector to access the ISR
    // offset is the address of the ISR (for task gates, offset must be 0)
    // type indicates the IDT entry type
    // dpl is the privilege level required to call the associated ISR
    fn new(selector: u16, offset: u32, idt_type: u8, dpl: u8) -> IdtEntry {
        IdtEntry {
            offset15_0: (offset & 0xffff) as u16,
            selector: selector,
            reserved: 0,
            flags: idt_type | dpl<<5 | 1<<7,
            offset31_16: ((offset >> 16) & 0xffff) as u16
        }
    }
}

// Structure describing a pointer to the IDT gate table.
// This format is required by the lgdt instruction.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct IdtPtr {
    limit: u16, // Limit of the table (ie. its size)
	base: u32   // Address of the first entry
}
impl IdtPtr {
    fn null() -> IdtPtr {
        IdtPtr {
            limit: 0,
            base: 0
        }
    }
}

// CPU context used when saving/restoring context from an interrupt
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Regs {
    gs: u32, fs: u32, es: u32, ds: u32,
	ebp: u32, edi: u32, esi: u32,
	edx: u32, ecx: u32, ebx: u32, eax: u32,
	number: u32, error_code: u32,
	eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32
}