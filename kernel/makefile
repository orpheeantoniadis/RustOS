BUILD_FOLDER = build
SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
KERNEL = kernel.elf
LINKER = linker.ld
GRUB = grub.cfg
OS = rustos
ISO = $(BUILD_FOLDER)/$(OS).iso

.PHONY: all clean run iso

all: $(BUILD_FOLDER)/$(KERNEL) iso

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO)

iso: $(ISO)

$(ISO): $(BUILD_FOLDER)/$(KERNEL) $(GRUB)
	mkdir -p $(BUILD_FOLDER)/isofiles/boot/grub
	cp $(BUILD_FOLDER)/$(KERNEL) $(BUILD_FOLDER)/isofiles/boot/$(KERNEL)
	cp $(GRUB) $(BUILD_FOLDER)/isofiles/boot/grub
	grub-mkrescue -o $(ISO) $(BUILD_FOLDER)/isofiles
	rm -r $(BUILD_FOLDER)/isofiles

$(BUILD_FOLDER)/$(KERNEL): $(OBJS) $(LINKER)
	ld -n -T $(LINKER) -o $(BUILD_FOLDER)/$(KERNEL) $(OBJS)

$(BUILD_FOLDER)/%.o: %.s
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
	
clean:
	rm -rf $(BUILD_FOLDER)