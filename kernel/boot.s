extern paging64
extern longmode_entry
global entrypoint

STACK_SIZE 	equ 0x100000

;-------------------------------------------------------------------------------
section .text
bits 32
entrypoint:
  ; code starts executing here
  cli  ; disable hardware interrupts

  ; Initialize the stack pointer and EBP (both to the same value)
  mov 	esp, stack + STACK_SIZE
  mov 	ebp, stack + STACK_SIZE
  
  call paging64
  
  ; load the 64-bit GDT
  lgdt [gdt64.pointer]
  jmp gdt64.code:longmode_entry

;-------------------------------------------------------------------------------
; .stack section 1MB long
section .stack nobits
stack:
resb STACK_SIZE 	; reserve 1MB for the stack

;-------------------------------------------------------------------------------
section .rodata
gdt64:
  dq 0 ; zero entry
.code: equ $ - gdt64 ; new
  dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
  dw $ - gdt64 - 1
  dq gdt64
