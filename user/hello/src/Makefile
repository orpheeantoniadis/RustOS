ARCH = i386
TARGET = $(ARCH)-app
LINKER = app.ld
CC = rustc
CFLAGS = --target=$(TARGET) -Z no-landing-pads -C relocation-model=static -C panic=abort

BUILD_FOLDER = build
SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s,$(BUILD_FOLDER)/%.o,$(SRCS))
RUST = lib
EXEC = hello

LIBCORE = libcore.rlib
CBUILTINS = libcompiler_builtins.rlib

all : $(BUILD_FOLDER)/$(EXEC)

$(BUILD_FOLDER)/$(EXEC) : $(OBJS) $(BUILD_FOLDER)/$(RUST).o
	gcc -T $(LINKER) -m32 -MMD -g -ffreestanding -nostdlib -Wall -Wextra -fno-pie $^ -o $@

$(BUILD_FOLDER)/%.o : %.s
	mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@

$(BUILD_FOLDER)/$(RUST).o : $(RUST).rs $(BUILD_FOLDER)/$(LIBCORE)
	mkdir -p $(shell dirname $@)
	$(CC) -L $(BUILD_FOLDER) $(CFLAGS) --out-dir $(BUILD_FOLDER) --emit obj $<
	
$(BUILD_FOLDER)/$(LIBCORE):
	mkdir -p $(shell dirname $@)
	cargo build --manifest-path libcore/Cargo.toml --target $(TARGET)
	cp libcore/target/$(TARGET)/debug/libcore.rlib $(BUILD_FOLDER)

clean :
	rm -rf $(BUILD_FOLDER)/*.o
	
mrproper :
	cargo clean --manifest-path libcore/Cargo.toml
	rm -rf $(BUILD_FOLDER)

rebuild : clean all