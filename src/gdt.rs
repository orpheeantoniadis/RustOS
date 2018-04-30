use core::mem::uninitialized;
use core::mem::size_of;
use x86::*;

extern "C" {
    fn gdt_load(gdt_ptr: *const GdtPtr);
}

#[repr(C, packed)]
pub struct Gdt {
    gdt_table: [GdtEntry; 3],
    gdt_ptr: GdtPtr
}

impl Gdt {
    pub fn new() -> Gdt {
        unsafe { uninitialized() }
    }

    // Initialize the GDT
    pub fn gdt_init(&mut self) {
    	// initialize 3 segment descriptors: NULL, code segment, data segment.
    	// Code and data segments must have a privilege level of 0.
        self.gdt_table[0] = GdtEntry::new();
        self.gdt_table[1] = GdtEntry::make_code_segment(0, 0xfffff, 0);
        self.gdt_table[2] = GdtEntry::make_data_segment(0, 0xfffff, 0);

    	// TODO: setup gdt_ptr so it points to the GDT and ensure it has the right limit.
        self.gdt_ptr.base = &self.gdt_table as *const _ as u32;
        self .gdt_ptr.limit = size_of::<GdtEntry>() as u16;

        // Load the GDT
        unsafe { gdt_load(&self.gdt_ptr); }
    }
}

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
    fn new() -> GdtEntry {
        GdtEntry { 
            lim15_0: 0,
            base15_0: 0,
            base23_16: 0,
            gdt_type: 4,
            s: 1,
            dpl: 2,
            present: 1,
            lim19_16: 4,
            avl: 1,
            l: 1,
            db: 1,
            granularity: 1,
            base31_24: 0
        }
    }
    
    fn build_entry(base: u32, limit: u32, gdt_type: u8, s: u8, db: u8, granularity: u8, dpl: u8) -> GdtEntry {
        GdtEntry { 
            lim15_0: (limit & 0xffff) as u16,
            base15_0: (base & 0xffff) as u16,
            base23_16: ((base >> 16) & 0xff) as u8,
            gdt_type: gdt_type,
            s: s,
            dpl: dpl,
            present: 1,
            lim19_16: ((limit >> 16) & 0xf) as u8,
            avl: 0,
            l: 0,
            db: db,
            granularity: granularity,
            base31_24: ((base >> 24) & 0xff) as u8
        }
    }
    
    fn make_code_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_CODE_EXECREAD, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
    
    fn make_data_segment(base: u32, limit: u32, dpl: u8) -> GdtEntry {
        GdtEntry::build_entry(base, limit, TYPE_DATA_READWRITE, S_CODE_OR_DATA, DB_SEG, 1, dpl)
    }
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16, // Limit of the table (ie. its size)
    base: u32   // Address of the first entry
}