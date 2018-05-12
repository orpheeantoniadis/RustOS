ARCH = i386
BUILD_FOLDER = build
SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
KERNEL = kernel.elf
LINKER = kernel.ld
GRUB = ../../grub
OS = rust_os
ISO = $(BUILD_FOLDER)/$(OS).iso
RUST = ../target/$(ARCH)-rust_os/debug/librust_os.a
FS = ../../tools/MicroFS/fs.img
QEMU = qemu-system-$(ARCH)

.PHONY: all clean run iso

all: $(BUILD_FOLDER)/$(KERNEL) iso

run: $(ISO) $(FS)
	$(QEMU) -monitor stdio -hda $(FS) -cdrom $(ISO)

iso: $(ISO)

$(ISO): $(BUILD_FOLDER)/$(KERNEL)
	mkdir -p $(BUILD_FOLDER)/isofiles/boot/grub
	cp $(BUILD_FOLDER)/$(KERNEL) $(BUILD_FOLDER)/isofiles/boot/$(KERNEL)
	cp -r $(GRUB) $(BUILD_FOLDER)/isofiles/boot
	genisoimage -R -b boot/grub/stage2_eltorito -input-charset utf8 -no-emul-boot -boot-info-table -o $(@) $(BUILD_FOLDER)/isofiles
	rm -r $(BUILD_FOLDER)/isofiles

$(BUILD_FOLDER)/$(KERNEL): $(OBJS) $(LINKER) $(RUST)
	gcc $(OBJS) -T $(LINKER) -static -m32 -ffreestanding -nostdlib -o $@ $(RUST)

$(BUILD_FOLDER)/%.o: %.s
	mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@
	
$(RUST):
	$(MAKE) -C ../
	
clean:
	rm -rf $(BUILD_FOLDER)