#![allow(dead_code)]

use core::mem::size_of;
use x86::*;
use vga::*;

static mut GDT_TABLE: GdtTable = [GdtEntry::null(), GdtEntry::null(), GdtEntry::null()];
static mut GDT_PTR: GdtPtr = GdtPtr::null();

type GdtTable = [GdtEntry; 3];

extern "C" {
    fn gdt_load(gdt_ptr: *const GdtPtr);
}

// Initialize the GDT
pub fn gdt_init() {
    unsafe {
        // initialize 3 segment descriptors: NULL, code segment, data segment.
        // Code and data segments must have a privilege level of 0.
        GDT_TABLE = [
            GdtEntry::null(), 
            GdtEntry::make_code_segment(0, 0xfffff, 0), 
            GdtEntry::make_data_segment(0, 0xfffff, 0)
        ];
        // setup gdt_ptr so it points to the GDT and ensure it has the right limit.
        GDT_PTR = GdtPtr::new((size_of::<GdtTable>() - 1) as u16, &GDT_TABLE);
        // Load the GDT
        // gdt_load(&GDT_PTR);
    }
    println!("GDT Table : {:?}",GDT_TABLE );
    println!("GDT Pointer : {:?}", GDT_PTR);
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct GdtEntry {
    lim15_0: u16,
    base15_0: u16,
    base23_16: u8,

    gdt_type: u8,
    s: u8,
    dpl: u8,
    present: u8,

    lim19_16: u8,
    avl: u8,
    l: u8,
    db: u8,
    granularity: u8,

    base31_24: u8
}

impl GdtEntry {
    const fn null() -> GdtEntry {
        GdtEntry { 
            lim15_0:        0,
            base15_0:       0,
            base23_16:      0,
            gdt_type:       0,
            s:              0,
            dpl:            0,
            present:        0,
            lim19_16:       0,
            avl:            0,
            l:              0,
            db:             0,
            granularity:    0,
            base31_24:      0
        }
    }
    
    fn new() -> GdtEntry {
        GdtEntry { 
            lim15_0:        0,
            base15_0:       0,
            base23_16:      0,
            gdt_type:       4,  // See TYPE_xxx flags
            s:              1,  // 1 for segments; 0 for system (TSS, LDT, gates)
            dpl:            2,  // privilege level
            present:        1,  // present in memory
            lim19_16:       4,
            avl:            1,  // available for use
            l:              1,  // should be 0 (64-bit code segment)
            db:             1,  // 1 for 32-bit code and data segments; 0 for system (TSS, LDT, gate)
            granularity:    1,  // granularity of the limit value: 0 = 1 byte; 1 = 4096 bytes
            base31_24:      0
        }
    }
    
    fn build_entry(base: u32, limit: u32, gdt_type: u8, s: u8, db: u8, granularity: u8, dpl: u8) -> GdtEntry {
        GdtEntry { 
            lim15_0:        (limit & 0xffff) as u16,
            base15_0:       (base & 0xffff) as u16,
            base23_16:      ((base >> 16) & 0xff) as u8,
            gdt_type:       gdt_type,
            s:              s,
            dpl:            dpl,
            present:        1,
            lim19_16:       ((limit >> 16) & 0xf) as u8,
            avl:            0,
            l:              0,
            db:             db,
            granularity:    granularity,
            base31_24:      ((base >> 24) & 0xff) as u8
        }
    }
    
    fn make_code_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_CODE_EXECREAD, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
    
    fn make_data_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_DATA_READWRITE, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct GdtPtr {
    limit: u16, // Limit of the table (ie. its size)
    base: *const GdtTable // Address of the first entry
}

impl GdtPtr {
    const fn null() -> GdtPtr {
        GdtPtr {
            limit:  0,
            base:   0 as *const _
        }
    }
    
    fn new(limit: u16, base: *const GdtTable) -> GdtPtr {
        GdtPtr {
            limit:  limit,
            base:   base
        }
    }
}