global enable_paging

section .text:          ; start of the text (code) section

enable_paging:
    push ebp
    mov ebp, esp
    
    mov eax,[esp+8]     ; Get the pointer to the page directory, passed as a parameter.
    mov cr3, eax
    
    mov ebx, cr0        ; read current cr0
    or  ebx, 1 << 31    ; set PG
    mov cr0, ebx        ; update cr0
    
    leave
    ret