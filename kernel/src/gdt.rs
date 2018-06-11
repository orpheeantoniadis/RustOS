//! Module for the memory management of RustOS using a Global Descriptor Table

#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use task::*;

/// The GDT size (not including the tss and ldt entries)
pub const GDT_SIZE: usize = 6;

/// Converts a descriptor index in the GDT into a selector
pub const fn gdt_index_to_selector(idx: u32) -> u32 {idx << 3}
/// Converts a descriptor selector in the GDT into an index
pub const fn selector_to_gdt_index(idx: u32) -> u32 {idx >> 3}

/// The Global Descriptor Table of RustOS
pub static mut GDT: Gdt = [GdtEntry::null();GDT_SIZE+TASKS_NB];
static mut GDT_PTR: GdtPtr = GdtPtr::null();

/// Defines a Global Descriptor Table
pub type Gdt = [GdtEntry; GDT_SIZE+TASKS_NB];

/// Structure of a GDT descriptor. There are 2 types of descriptors: segments and TSS.
/// Section 3.4.5 of Intel 64 & IA32 architectures software developer's manual describes
/// segment descriptors while section 6.2.2 describes TSS descriptors.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GdtEntry {
    lim15_0: u16,
    base15_0: u16,
    base23_16: u8,

    flags7_0: u8,
    flags15_8: u8,

    base31_24: u8
}

// Structure describing a pointer to the GDT descriptor table.
// This format is required by the lgdt instruction.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct GdtPtr {
    limit: u16, // Limit of the table (ie. its size)
    base: *const Gdt // Address of the first entry
}

extern "C" {
    fn gdt_load(gdt_ptr: *const GdtPtr);
}

/// Initialize the Global Descriptor Table
pub fn gdt_init() {
    unsafe {
        // initialize 3 segment descriptors: NULL, code segment, data segment.
        // Code and data segments must have a privilege level of 0.
        GDT[0] = GdtEntry::null();
        GDT[1] = GdtEntry::make_code_segment(0, 0xfffff, DPL_KERNEL);
        GDT[2] = GdtEntry::make_data_segment(0, 0xfffff, DPL_KERNEL);
        GDT[3] = GdtEntry::make_code_segment(0, 0xfffff, DPL_USER);
        GDT[4] = GdtEntry::make_data_segment(0, 0xfffff, DPL_USER);
        // setup gdt_ptr so it points to the GDT and ensure it has the right limit.
        GDT_PTR = GdtPtr::new((size_of::<Gdt>() - 1) as u16, &GDT);
        // Load the GDT
        gdt_load(&GDT_PTR);
        // Init tasks
        tasks_init();
    }
}

impl GdtEntry {
    /// Create a null segment
    pub const fn null() -> GdtEntry {
        GdtEntry { 
            lim15_0:    0,
            base15_0:   0,
            base23_16:  0,
            flags7_0:   0,
            flags15_8:  0,
            base31_24:  0
        }
    }
    
    fn build_entry(base: u32, limit: u32, gdt_type: u8, s: u8, db: u8, granularity: u8, dpl: u8) -> GdtEntry {
        GdtEntry { 
            lim15_0:    (limit & 0xffff) as u16,
            base15_0:   (base & 0xffff) as u16,
            base23_16:  ((base >> 16) & 0xff) as u8,
            flags7_0:   gdt_type | s<<4 | dpl<<5 | 1<<7,
            flags15_8:  ((limit >> 16) & 0xf) as u8 | db<<6 | granularity<<7,
            base31_24:  ((base >> 24) & 0xff) as u8
        }
    }
    
    /// Create a code segment specified by the base, limit and privilege level passed in arguments
    pub fn make_code_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_CODE_EXECREAD, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
    
    /// Create a data segment specified by the base, limit and privilege level passed in arguments
    pub fn make_data_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_DATA_READWRITE, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
    
    /// Create a TSS segment
    pub fn make_tss(base: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, (size_of::<Tss>() - 1) as u32, TYPE_TSS, S_SYSTEM, DB_SYS, 0, dpl)
    }
    
    /// Create an LDT segment
    pub fn make_ldt(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_LDT, S_SYSTEM, DB_SYS, 0, dpl)
    }
    
    /// Converts a GDT entry to its index in the GDT
    pub fn to_index(&mut self) -> u32 {
        unsafe {
            ((self as *mut _ as u32) - (&GDT as *const _ as u32)) >> 3
        }
    }
    
    /// Converts a GDT entry to its selector in the GDT
    pub fn to_selector(&mut self) -> u32 {
        unsafe {
            (self as *mut _ as u32) - (&GDT as *const _ as u32)
        }
    }
}

impl GdtPtr {
    const fn null() -> GdtPtr {
        GdtPtr {
            limit:  0,
            base:   0 as *const _
        }
    }
    
    fn new(limit: u16, base: *const Gdt) -> GdtPtr {
        GdtPtr {
            limit:  limit,
            base:   base
        }
    }
}