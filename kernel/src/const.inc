; Kernel base address
KERNEL_BASE equ 0xC0000000
KERNEL_PAGE_NUMBER equ (KERNEL_BASE >> 22)

; Kernel stack size
STACK_SIZE  equ 0x100000

; Must match the values of the same constants in gdt.h!
GDT_KERNEL_CODE_SELECTOR  equ     0x08
GDT_KERNEL_DATA_SELECTOR  equ     0x10