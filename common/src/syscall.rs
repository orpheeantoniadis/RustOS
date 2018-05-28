#[repr(u8)]
pub enum Syscall {
    Puts            = 0x0,
    Putc            = 0x1,
    Exec            = 0x2,
    Keypressed      = 0x3,
    Getc            = 0x4,
    FileStat        = 0x5,
    FileOpen        = 0x6,
    FileClose       = 0x7,
    FileRead        = 0x8,
    FileSeek        = 0x9,
    FileIterator    = 0xa,
    FileNext        = 0xb,
    GetTicks        = 0xc,
    Sleep           = 0xd
}