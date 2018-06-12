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
ISO = $(OS).iso
IFLAGS = -input-charset utf8 -no-emul-boot -boot-info-table

FS_FOLDER = ../../tools/MicroFS
USER_PATH = ../../user
FS = fs.img
SPLASH = ../../doc/splash.txt

.PHONY : all run iso elf fs user clean mrproper

all : iso fs

run : iso fs
	$(QEMU) -cdrom $(BUILD_FOLDER)/$(ISO) -hda $(BUILD_FOLDER)/$(FS)
	
iso : $(BUILD_FOLDER)/$(ISO)
	
elf : $(BUILD_FOLDER)/$(KERNEL)
	
fs : $(BUILD_FOLDER)/$(FS)
	
user :
	$(MAKE) -C $(USER_PATH)
	
$(BUILD_FOLDER)/$(ISO) : elf
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
	
$(BUILD_FOLDER)/$(FS) : $(SPLASH) user
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) create MicroFS 1 1000000
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(SPLASH)
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(USER_PATH)/build/hello
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(USER_PATH)/build/demo
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(USER_PATH)/build/shell
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(USER_PATH)/build/splash
	
clean :
	rm -rf $(BUILD_FOLDER)
	
mrproper : clean
	cargo clean --manifest-path $(FS_FOLDER)/Cargo.toml
	$(MAKE) -C $(USER_PATH) clean