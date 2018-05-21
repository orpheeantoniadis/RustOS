pub enum Syscall {
    Puts            = 0x0,
    Exec            = 0x1,
    Keypressed      = 0x2,
    Getc            = 0x3,
    FileStat        = 0x4,
    FileOpen        = 0x5,
    FileClose       = 0x6,
    FileRead        = 0x7,
    FileSeek        = 0x8,
    FileIterator    = 0x9,
    FileNext        = 0xa,
    GetTicks        = 0xb,
    Sleep           = 0xc
}