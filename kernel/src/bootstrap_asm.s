extern kernel_start
extern kernel_end

extern low_kernel_start
extern low_kernel_end

extern kmain
global entrypoint

; Values for the multiboot header
MULTIBOOT_MAGIC        equ 0x1BADB002
MULTIBOOT_ALIGN_MODS   equ 1
MULTIBOOT_MEMINFO      equ 2
MULTIBOOT_VIDINFO      equ 4

MULTIBOOT_FLAGS     equ (MULTIBOOT_ALIGN_MODS|MULTIBOOT_MEMINFO)

; Magic + checksum + flags must equal 0!
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

KERNEL_BASE equ 0xC0000000
KERNEL_PAGE_NUMBER equ (KERNEL_BASE >> 22)

STACK_SIZE  equ 0x100000

;-------------------------------------------------------------------------------
; .multiboot section
; This section must be located at the very beginning of the kernel image.

section .multiboot

; Mandatory part of the multiboot header
; see http://git.savannah.gnu.org/cgit/grub.git/tree/doc/multiboot.h?h=multiboot
dd MULTIBOOT_MAGIC
dd MULTIBOOT_FLAGS
dd MULTIBOOT_CHECKSUM

;-------------------------------------------------------------------------------
section .low_text
 
entrypoint:
    ; save multiboot infos
    mov [multiboot_magic], eax
    mov [multiboot_info],  ebx
    
    ; map low kernel pt in pd
    mov eax, low_kernel_pt
    mov [page_directory], eax
    or dword [page_directory], 0x3
    
    mov eax, 0
    .low_kernel_pt_init:
        mov ecx, eax
        shr ecx, 12
        and ecx, 0x3ff
        mov [low_kernel_pt + ecx * 4], eax
        or dword [low_kernel_pt + ecx * 4], 0x3 

        add eax, 0x1000
        cmp eax, low_kernel_end
        jl .low_kernel_pt_init
        
    ; map higher kernel pt in pd
    mov eax, kernel_pt
    mov [page_directory + KERNEL_PAGE_NUMBER * 4], eax
    or dword [page_directory + KERNEL_PAGE_NUMBER * 4], 0x3
    
    mov eax, kernel_start
    .high_kernel_pt_init:
        mov ecx, eax
        shr ecx, 12
        and ecx, 0x3ff

        mov ebx, eax 
        sub ebx, KERNEL_BASE ; convert virt->physical
        mov [kernel_pt + ecx * 4], ebx
        or dword [kernel_pt + ecx * 4], 0x3

        add eax, 0x1000
        cmp eax, kernel_end
        jl .high_kernel_pt_init

    ; init paging
    mov eax, page_directory
    mov cr3, eax
    mov eax, cr0
    or eax, 0x80000000
    mov cr0, eax
    
    lea ecx, [higher_half]
    jmp ecx

;-------------------------------------------------------------------------------
section .low_data

multiboot_magic:
    dd 0
multiboot_info:
    dd 0

;-------------------------------------------------------------------------------
section .low_bss nobits

alignb 4096
page_directory:
    resd 1024
low_kernel_pt:
    resd 1024
kernel_pt:
    resd 1024

;-------------------------------------------------------------------------------
section .text
higher_half:
    ; code starts executing here
    cli  ; disable hardware interrupts

    ; Initialize the stack pointer and EBP (both to the same value)
    mov esp, stack + STACK_SIZE
    mov ebp, stack + STACK_SIZE

    ; pass the multiboot info to the kernel
    push dword [multiboot_info]

    call kmain

    .forever:
        hlt
        jmp .forever

;-------------------------------------------------------------------------------
; .stack section 1MB long
section .stack nobits
stack:
resb STACK_SIZE ; reserve 1MB for the stack
