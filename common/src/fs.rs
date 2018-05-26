pub const MAX_FILENAME_LENGTH: usize = 26;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Stat {
    pub name: [u8;MAX_FILENAME_LENGTH],
    pub size: usize,
    pub entry_offset: u16,
    pub start: usize
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FileIterator {
    pub sector: u32,
    pub offset: usize
}

impl Stat {
    pub fn null() -> Stat {
        Stat {
            name: [0;MAX_FILENAME_LENGTH],
            size: 0,
            entry_offset: 0,
            start: 0
        }
    }
    
    pub fn as_ptr(&mut self) -> *const Stat {
        self as *const Stat
    }
}

impl FileIterator {
    pub fn null() -> FileIterator {
        FileIterator {
            sector: 0,
            offset: 0
        }
    }
    
    pub fn as_ptr(&mut self) -> *const FileIterator {
        self as *const FileIterator
    }
}