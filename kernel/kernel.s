global kernel_entry

VRAM      equ 0xb8000
VRAM_SIZE	equ 4000
POSITION	equ 1986
CRTC_CMD  equ 0x3d4
CRTC_DATA equ 0x3d5

txt 	db "Hello World !", 0

clear:
	push ebp										; set up stack frame
	mov ebp,esp

	mov eax, VRAM								; set eax to start of VRAM
	.while:
		mov dword [eax], 0x0000		; set character to 0 (black)
		add eax, 2								; move to next address
		cmp eax, (VRAM+VRAM_SIZE)	; check if we are at the end of VRAM
		jne .while

	mov eax, 0
	leave												; free stack frame
	ret
	
print:
	push ebp									; set up stack frame
	mov ebp,esp
	
	mov eax, (VRAM+POSITION)	; set eax to position to write
	mov ecx, txt							; ecx contains the text to display
	mov edx, 0								; clear edx
	
	.while:
		mov dh, 0x0f						; MSB is color (white on black)
		mov dl, [ecx]						; LSB is character
		mov dword [eax], edx		; write in VRAM
		add eax, 2							; move to next addr
		inc ecx									; move to next char
		cmp byte [ecx], 0				; break the loop it is the end of the string
		jnz .while
	
	mov eax, 0
	leave											; free stack frame
	ret

move_cursor:
	push ebp
	mov ebp,esp
	
	; move cursor to POSITION
	mov ecx, POSITION
	shr ecx, 1

	; set MSB of cursor position
	mov dx, CRTC_CMD
	mov al, 0xe
	out dx, al
	mov dx, CRTC_DATA
	mov al, ch
	out dx, al
	
	; set LSB of cursor position
	mov dx, CRTC_CMD
	mov al, 0xf
	out dx, al
	mov dx, CRTC_DATA
	mov al, cl
	out dx, al
	
	mov eax, 0
	leave
	ret

kernel_entry:
	push ebp
	mov ebp,esp

	call clear
	call print
	call move_cursor
	
	mov eax, 0
	leave
	ret