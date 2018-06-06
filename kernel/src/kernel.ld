/* the entry point */
ENTRY(entrypoint)

SECTIONS {
    /* Low memory Kernel */
    . = 0x00100000;
    low_kernel_start = .;
    .boot ALIGN(4) :
    {
        *(.multiboot)
    }
    .low_text ALIGN (4K) :
    {
        *(.low_text)
    }
    .low_data ALIGN (4K) :
    {
        *(.low_data)
    }
    .low_bss ALIGN (4K) :
    {
        low_bss_start = .;
        *(.low_bss)
        low_bss_end = .;
    }
    low_kernel_end = .;

    /* Higher-half Kernel */
    . += 0xC0000000;
    kernel_start = .;
    /* kernel stack */
    .stack ALIGN(4) : AT(ADDR(.stack) - 0xC0000000)
    {
        *(.stack)
    }
    /* code */
    .text ALIGN(4K) : AT(ADDR(.text) - 0xC0000000)
    {
        *(.text*)
    }
    /* read-only data */
    .rodata ALIGN(4K) : AT(ADDR(.rodata) - 0xC0000000)
    {
        *(.rodata*)
    }
    /* initialized data */
    .data ALIGN(4K) : AT(ADDR(.data) - 0xC0000000)
    {
        *(.data*)
    }
    /* unitialized data */
    .bss ALIGN(4K) : AT(ADDR(.bss) - 0xC0000000)
    {
        bss_start = .;
        *(COMMON)
        *(.bss*)
        bss_end = .;
    }
    kernel_end = .;
    
    /* Utils */
    low_kernel_size = low_kernel_end - low_kernel_start;
    low_bss_size = low_bss_end - low_bss_start;
    kernel_size = kernel_end - kernel_start;
    bss_size = bss_end - bss_start;
}

ASSERT(kernel_size < 0x300000, "Kernel exceeds the 3 MB limit!");