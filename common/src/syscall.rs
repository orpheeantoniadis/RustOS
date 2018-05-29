#[repr(u8)]
pub enum Syscall {
    Puts            = 0x00,
    Putc            = 0x01,
    Exec            = 0x02,
    Keypressed      = 0x03,
    Getc            = 0x04,
    FileStat        = 0x05,
    FileOpen        = 0x06,
    FileClose       = 0x07,
    FileRead        = 0x08,
    FileSeek        = 0x09,
    FileIterator    = 0x0a,
    FileNext        = 0x0b,
    GetTicks        = 0x0c,
    Sleep           = 0x0d,
    SetCursor       = 0x0e,
    GetCursor       = 0x0f,
    CursorDisable   = 0x10,
    CopyScr         = 0x11
}