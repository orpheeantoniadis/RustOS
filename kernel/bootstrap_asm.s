extern kernel_entry

global entrypoint  ; the entry point symbol defined in kernel.ld

; Values for the multiboot header
MULTIBOOT_MAGIC        equ 0x1BADB002
MULTIBOOT_ALIGN_MODS   equ 1
MULTIBOOT_MEMINFO      equ 2
MULTIBOOT_VIDINFO      equ 4

MULTIBOOT_FLAGS     equ (MULTIBOOT_ALIGN_MODS|MULTIBOOT_MEMINFO)

; Magic + checksum + flags must equal 0!
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

;---------------------------------------------------------------------------------------------------
; .multiboot section
; This section must be located at the very beginning of the kernel image.

section .multiboot

; Mandatory part of the multiboot header
; see http://git.savannah.gnu.org/cgit/grub.git/tree/doc/multiboot.h?h=multiboot
dd MULTIBOOT_MAGIC
dd MULTIBOOT_FLAGS
dd MULTIBOOT_CHECKSUM

STACK_SIZE 	equ 0x100000

entrypoint:
	; code starts executing here
	cli  ; disable hardware interrupts

	; Initialize the stack pointer and EBP (both to the same value)
	mov 	esp, stack + STACK_SIZE
	mov 	ebp, stack + STACK_SIZE
	
	; Pass the multiboot info to the kernel
	; push 	ebx
	
	; TODO : Call the kernel entry point (Rust code)
	call 	kernel_entry

	; infinite loop (should never get here)
.forever:
	hlt
	jmp .forever

;---------------------------------------------------------------------------------------------------
; .stack section 1MB long

section .stack nobits

stack:
resb STACK_SIZE 	; reserve 1MB for the stack
