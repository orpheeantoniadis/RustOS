#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use vga::*;
use pic::*;

const IDT_SIZE: usize = 256;
const EXCEPTION_MESSAGES: [&str;21] = [
	"Exception 0 - Divide Error",
	"Exception 1 - RESERVED",
	"Exception 2 - NMI Interrupt",
	"Exception 3 - Breakpoint",
	"Exception 4 - Overflow",
	"Exception 5 - BOUND Range Exceeded",
	"Exception 6 - Invalid Opcode (Undefined Opcode)",
	"Exception 7 - Device Not Available (No Math Coprocessor)",
	"Exception 8 - Double Fault",
	"Exception 9 - Coprocessor Segment Overrun (reserved)",
	"Exception 10 - Invalid TSS",
	"Exception 11 - Segment Not Present",
	"Exception 12 - Stack-Segment Fault",
	"Exception 13 - General Protection",
	"Exception 14 - Page Fault",
	"Exception 15 - (Inter reserved. Do not use.)",
	"Exception 16 - x87 FPU Floating-Point Error (Math Fault)",
	"Exception 17 - Alignment Check",
	"Exception 18 - Machine Check",
	"Exception 19 - SIMD Floating-Point Exception",
	"Exception 20 - Virtualization Exception"
];

static mut IDT: Idt = [IdtEntry::null();IDT_SIZE];
static mut IDT_PTR: IdtPtr = IdtPtr::null();

// Initialize the IDT
pub fn idt_init() {
    unsafe {
        // CPU exceptions
        IDT[0] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_0 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[1] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_1 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[2] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_2 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[3] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_3 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[4] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_4 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[5] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_5 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[6] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_6 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[7] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_7 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[8] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_8 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[9] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_9 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[10] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_10 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[11] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_11 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[12] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_12 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[13] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_13 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[14] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_14 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[15] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_15 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[16] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_16 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[17] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_17 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[18] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_18 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[19] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_19 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[20] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_exception_20 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        
        // IRQ
        IDT[32] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_0 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[33] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_1 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[34] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_2 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[35] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_3 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[36] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_4 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[37] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_5 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[38] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_6 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[39] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_7 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[40] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_8 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[41] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_9 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[42] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_10 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[43] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_11 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[44] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_12 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[45] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_13 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[46] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_14 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        IDT[47] = IdtEntry::new(GDT_KERNEL_CODE_SELECTOR as u16, &_irq_15 as *const _ as u32, TYPE_INTERRUPT_GATE, DPL_KERNEL);
        
        // setup idt_ptr so it points to the IDT and ensure it has the right limit.
        IDT_PTR = IdtPtr::new((size_of::<Idt>() - 1) as u16, &IDT as *const _ as u32);
        // Load the IDT
        idt_load(&IDT_PTR);
    }
}

// Exception handler
#[no_mangle]
pub extern fn exception_handler(regs: *mut Regs) {
    unsafe {
        panic!(EXCEPTION_MESSAGES[(*regs).number as usize]);
    }
}

// Irq handler
#[no_mangle]
pub extern fn irq_handler(regs: *mut Regs) {
    let irq = unsafe { (*regs).number };
    match irq {
        _ => println!("irq {} not implemented", irq)
    }
    pic_eoi(irq);
}

type Idt = [IdtEntry; IDT_SIZE];

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
    const fn null() -> IdtEntry {
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
    const fn null() -> IdtPtr {
        IdtPtr {
            limit: 0,
            base: 0
        }
    }
    
    fn new(limit: u16, base: u32) -> IdtPtr {
        IdtPtr {
            limit:  limit,
            base:   base
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

extern "C" {
    fn idt_load(idt_ptr: *const IdtPtr);
    
    // Exception handler
    fn _exception_0();
    fn _exception_1();
    fn _exception_2();
    fn _exception_3();
    fn _exception_4();
    fn _exception_5();
    fn _exception_6();
    fn _exception_7();
    fn _exception_8();
    fn _exception_9();
    fn _exception_10();
    fn _exception_11();
    fn _exception_12();
    fn _exception_13();
    fn _exception_14();
    fn _exception_15();
    fn _exception_16();
    fn _exception_17();
    fn _exception_18();
    fn _exception_19();
    fn _exception_20();
    
    // Interruption handler
    fn _irq_0();
    fn _irq_1();
    fn _irq_2();
    fn _irq_3();
    fn _irq_4();
    fn _irq_5();
    fn _irq_6();
    fn _irq_7();
    fn _irq_8();
    fn _irq_9();
    fn _irq_10();
    fn _irq_11();
    fn _irq_12();
    fn _irq_13();
    fn _irq_14();
    fn _irq_15();
}