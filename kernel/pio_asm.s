global outb
global outw
global inb
global inw

section .txt

outb:
  push ebp
  mov ebp, esp

	mov word dx, [esp+8]
	mov byte al, [esp+12]
	out dx, al
  
  mov eax, 0
  leave
	ret

outw:
  push ebp
  mov ebp, esp
  
	mov word dx, [esp+8]
	mov word ax, [esp+12]
	out dx, ax
  
  mov eax, 0
  leave
	ret

inb:
  push ebp
  mov ebp, esp
  
	mov word dx, [esp+8]
	in byte al, dx
  
  mov eax, 0
  leave
	ret

inw:
  push ebp
  mov ebp, esp

	mov word dx, [esp+8]
	in word ax, dx
  
  mov eax, 0
  leave
	ret
