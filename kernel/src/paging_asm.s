%include "const.inc"

extern kernel_start
extern kernel_end
extern page_directory
extern kernel_pt

global load_directory
global get_kernel_start
global get_kernel_end
global get_kernel_page_directory
global get_kernel_page_table

section .text:          ; start of the text (code) section

load_directory:
    push ebp
    mov ebp, esp
    
    mov eax,[esp+8]     ; Get the pointer to the page directory, passed as a parameter.
    mov cr3, eax
    
    mov ebx, cr0        ; read current cr0
    or  ebx, 1 << 31    ; set PG
    mov cr0, ebx        ; update cr0
    
    leave
    ret
    
get_kernel_start:
    push ebp
    mov ebp, esp
    
    mov eax, kernel_start
    
    leave
    ret
    
get_kernel_end:
    push ebp
    mov ebp, esp
    
    mov eax, kernel_end
    
    leave
    ret
    
get_kernel_page_directory:
    push ebp
    mov ebp, esp
    
    mov eax, page_directory
    add eax, KERNEL_BASE
    
    leave
    ret
    
get_kernel_page_table:
    push ebp
    mov ebp, esp
    
    mov eax, kernel_pt
    add eax, KERNEL_BASE
    
    leave
    ret