CC=gcc
CFLAGS=-Wall -Wextra -std=gnu11 -m32
SRCS=$(wildcard *.s)
OBJS=$(SRCS:.s=.o)
KERNEL=kernel.elf
LINKER=kernel.ld
OS=rustos

all : iso

iso : $(KERNEL)
	mkdir -p $(OS)/boot/
	cp -r ../grub $(OS)/boot
	cp kernel.elf $(OS)/boot
	genisoimage -R -b boot/grub/stage2_eltorito -input-charset utf8 -no-emul-boot -boot-info-table -o $(OS).iso $(OS)

$(KERNEL) : $(OBJS)
	$(CC) $(OBJS) -T $(LINKER) -static -m32 -ffreestanding -nostdlib -o $@

%.o : %.s
	nasm -f elf32 $^

run : $(OS).iso
	qemu-system-i386 -cdrom $<

clean:
	rm -rf rustos $(OBJS) $(KERNEL) *.iso