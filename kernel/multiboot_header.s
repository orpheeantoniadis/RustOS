; Values for the multiboot header
MULTIBOOT_MAGIC         equ 0xE85250D6
MULTIBOOT_ARCH          equ 0
MULTIBOOT_HEADER_LENGTH equ (header_end - header_start)
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_MAGIC + MULTIBOOT_ARCH + MULTIBOOT_HEADER_LENGTH)

section .multiboot_header
header_start:
    dd MULTIBOOT_MAGIC
    dd MULTIBOOT_ARCH
    dd MULTIBOOT_HEADER_LENGTH
    dd MULTIBOOT_CHECKSUM

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
