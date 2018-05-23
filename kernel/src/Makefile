ARCH = i386
BUILD_FOLDER = build
FS_FOLDER = ../../tools/MicroFS
USER_PATH = ../../user

SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
KERNEL = kernel.elf
LINKER = kernel.ld
GRUB = ../../grub
OS = rust_os
ISO = $(OS).iso
RUST = ../target/$(ARCH)-rust_os/debug/librust_os.a
FS = fs.img
SPLASH = ../../doc/splash.txt

QEMU = qemu-system-$(ARCH)

.PHONY: all run elf iso fs clean mrproper

all: elf iso fs

run: elf iso fs
	$(QEMU) -monitor stdio -hda $(BUILD_FOLDER)/$(FS) -cdrom $(BUILD_FOLDER)/$(ISO)

elf: $(BUILD_FOLDER)/$(KERNEL)

iso: $(BUILD_FOLDER)/$(ISO)
	
fs: $(BUILD_FOLDER)/$(FS) user
	
user:
	$(MAKE) -C $(USER_PATH)

$(BUILD_FOLDER)/$(ISO): elf
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
	
$(BUILD_FOLDER)/$(FS): $(SPLASH)
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) create MicroFS 1 100000
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(SPLASH)
	cargo run --manifest-path $(FS_FOLDER)/Cargo.toml $(BUILD_FOLDER)/$(FS) add $(USER_PATH)/build/hello
	
clean:
	rm -rf $(BUILD_FOLDER)
	
mrproper: clean
	cargo clean --manifest-path $(FS_FOLDER)/Cargo.toml