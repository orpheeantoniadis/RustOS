ARCH = i386
BUILD_FOLDER = build

LD = gcc
QEMU = qemu-system-$(ARCH)
GCC_KERNEL=-nostdlib \
	-nostdinc \
	-fno-builtin \
	-nostartfiles \
	-nodefaultlibs

LDFLAGS=-Wl,-m,elf_i386 \
	-Wl,-Map,kernel.map \
	-ggdb \
	-T $(LINKER) \
	-L lib \
	$(GCC_KERNEL)
	
CLDFLAGS=-Wl,-m,elf_i386 \
	-Wl,-Map,ckernel.map \
	-ggdb \
	-T $(LINKER) \
	-L lib \
	$(GCC_KERNEL)
	
CFLAGS=-m32 \
	-I$(CURDIR)/include/ \
	-ggdb \
	-Wall \
	-Wextra \
	$(GCC_KERNEL)

SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
RUST = ../target/$(ARCH)-rust_os/release/libkernel.a

KERNEL = kernel.elf
CKERNEL = ckernel.elf
LINKER = kernel.ld

.PHONY: all run crun elf celf clean mrproper

all: elf celf

run: elf
	$(QEMU) -name RustOS -m 32 -no-reboot -kernel $(BUILD_FOLDER)/$(KERNEL)

crun: celf
	$(QEMU) -name RustOS -m 32 -no-reboot -kernel $(BUILD_FOLDER)/$(CKERNEL)

elf: $(BUILD_FOLDER)/$(KERNEL)
	
celf: $(BUILD_FOLDER)/$(CKERNEL)
	
$(BUILD_FOLDER)/$(KERNEL): $(OBJS) $(LINKER) $(RUST)
	$(LD) $(LDFLAGS) $(OBJS) $(RUST) -o $@ -lgcc
	
$(BUILD_FOLDER)/$(CKERNEL): $(OBJS) $(BUILD_FOLDER)/kernel.o
	$(LD) $(CLDFLAGS) $^ -o $@ -lgcc

$(BUILD_FOLDER)/%.o: %.s
	mkdir -p $(shell dirname $@)
	nasm -f elf $< -o $@
	
$(BUILD_FOLDER)/kernel.o: kernel.c
	gcc $(CFLAGS) -c $< -o $@
	
$(RUST):
	$(MAKE) -C ../
	
clean:
	rm -rf $(BUILD_FOLDER)
	rm -f kernel.map
	rm -f ckernel.map
	
mrproper: clean
	