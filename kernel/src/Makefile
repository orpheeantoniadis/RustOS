ARCH = i386
BUILD_FOLDER = build

LD = gcc
QEMU = qemu-system-$(ARCH)
GCC_KERNEL = -static -m32 -ffreestanding -nostdlib
LDFLAGS = -T $(LINKER) $(GCC_KERNEL) -Wl,-Map,$(BUILD_FOLDER)/kernel.map

SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
RUST = ../target/$(ARCH)-rust_os/debug/libkernel.a

KERNEL = kernel.elf
LINKER = kernel.ld

GRUB = ../../grub
OS = rust_os
ISO = $(BUILD_FOLDER)/$(OS).iso
IFLAGS = -input-charset utf8 -no-emul-boot -boot-info-table

FS_FOLDER = ../../tools/MicroFS
USER_PATH = ../../user
FS = $(BUILD_FOLDER)/fs.img
SPLASH = ../../doc/splash.txt

.PHONY : all run kernel user clean mrproper

all : $(ISO) $(FS)

run : $(ISO) $(FS)
	$(QEMU) -cdrom $(ISO) -hda $(FS)
	
kernel : $(BUILD_FOLDER)/$(KERNEL)
	
user :
	$(MAKE) -C $(USER_PATH)
	
$(ISO) : | kernel
	mkdir -p $(BUILD_FOLDER)/isofiles/boot/grub
	cp $(BUILD_FOLDER)/$(KERNEL) $(BUILD_FOLDER)/isofiles/boot/$(KERNEL)
	cp -r $(GRUB) $(BUILD_FOLDER)/isofiles/boot
	genisoimage -R -b boot/grub/stage2_eltorito $(IFLAGS) -o $(@) $(BUILD_FOLDER)/isofiles
	rm -r $(BUILD_FOLDER)/isofiles
	
$(BUILD_FOLDER)/$(KERNEL) : $(LINKER) $(OBJS) $(RUST)
	$(LD) $(LDFLAGS) $(OBJS) $(RUST) -o $@

$(BUILD_FOLDER)/%.o : %.s
	mkdir -p $(shell dirname $@)
	nasm -f elf $< -o $@
	
$(RUST) :
	$(MAKE) -C ../
	
$(FS) : $(SPLASH) | user
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ create MicroFS 1 1000000
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ add $(SPLASH)
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ add $(USER_PATH)/build/hello
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ add $(USER_PATH)/build/demo
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ add $(USER_PATH)/build/shell
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $@ add $(USER_PATH)/build/splash
	
clean :
	rm -rf $(BUILD_FOLDER)
	
mrproper : clean
	cargo clean --manifest-path $(FS_FOLDER)/Cargo.toml
	$(MAKE) -C $(USER_PATH) clean