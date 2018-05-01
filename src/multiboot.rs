#![allow(dead_code)]

#[repr(C)]
pub struct MultibootAoutSymbolTable {
    pub tabsize: u32,
    pub strsize: u32,
    pub addr: u32,
    pub reserved: u32
}

#[repr(C)]
pub struct MultibootElfSectionHeaderTable {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32
}

#[repr(C)]
pub struct MultibootInfo {
    /* Multiboot info version number */
    pub flags: u32,

    /* Available memory from BIOS (in KB) */
    pub mem_lower: u32,
    pub mem_upper: u32,

    /* "root" partition */
    pub boot_device: u32,

    /* Kernel command line */
    pub cmdline: u32,

    /* Boot-Module list */
    pub mods_count: u32,
    pub mods_addr: u32,

    pub aout_sym: MultibootAoutSymbolTable,
    pub elf_sec: MultibootElfSectionHeaderTable,

    /* Memory Mapping buffer */
    pub mmap_length: u32,
    pub mmap_addr: u32,

    /* Drive Info buffer */
    pub drives_length: u32,
    pub drives_addr: u32,

    /* ROM configuration table */
    pub config_table: u32,

    /* Boot Loader Name */
    pub boot_loader_name: u32,

    /* APM table */
    pub apm_table: u32,

    /* Video */
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16
}

#[repr(C)]
pub struct MultibootModList {
    /* the memory used goes from bytes 'mod_start' to 'mod_end-1' inclusive */
    pub mod_start: u32,
    pub mod_end: u32,

    /* Module command line */
    pub cmdline: u32,

    /* padding to take it to 16 bytes (must be zero) */
    pub pad: u32
}
