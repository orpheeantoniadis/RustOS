extern rust_entry
global longmode_entry

section .text
bits 64
longmode_entry:
  ; load 0 into all data segment registers
  mov ax, 0
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  ; TODO : Pass the multiboot info to the kernel
  ; push 	ebx
  
  ; Call the kernel entry point (Rust code)
  call rust_entry
  
  ; infinite loop (should never get here)
  .forever:
    hlt
    jmp .forever
    