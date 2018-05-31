#![allow(dead_code)]

const MAX_ENTRIES : usize = 1024;
pub static mut INITIAL_PAGE_DIRECTORY: PageDirectory = PageDirectory::new();
pub static mut INITIAL_PAGE_TABLE: PageTable = PageTable::new();

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PageEntry(u32);

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageEntry;MAX_ENTRIES]
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
pub struct PageDirectory {
    tables: [PageEntry;MAX_ENTRIES]
}

extern "C" {
    fn enable_paging(page_directory_ptr: *const PageDirectory);
}

pub fn paging_init() {
    unsafe {
        INITIAL_PAGE_TABLE.set();
        INITIAL_PAGE_DIRECTORY.set_table(0, &INITIAL_PAGE_TABLE);
        enable_paging(&INITIAL_PAGE_DIRECTORY);
    }
}

impl PageEntry {
    const fn null() -> PageEntry {
        PageEntry(0x2) // Read/Write
    }
    
    const fn new() -> PageEntry {
        PageEntry(0x3) // Present + Read/Write
    }
}

impl PageTable {
    const fn new() -> PageTable {
        PageTable { entries: [PageEntry::new();MAX_ENTRIES] }
    }
    
    fn set(&mut self) {
        for i in 0..MAX_ENTRIES {
            self.entries[i] = PageEntry((i as u32 * 0x1000) | 0x3);
        }
    }
}

impl PageDirectory {
    const fn new() -> PageDirectory {
        PageDirectory { tables: [PageEntry::null();MAX_ENTRIES] }
    }
    
    fn set_table(&mut self, index: usize, table: *const PageTable) {
        self.tables[index] = PageEntry(table as u32 | 0x3);
    }
}