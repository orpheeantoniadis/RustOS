ARCH = i386
TARGET = $(ARCH)-app

CC = gcc
LINKER = app.ld
CFLAGS = -T $(LINKER) -m32 -ffreestanding -nostdlib -fno-pie
RFLAGS = --target=$(TARGET) -Z no-landing-pads -C relocation-model=static

BUILD_FOLDER = build
SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s,$(BUILD_FOLDER)/%.o,$(SRCS))
RSRCS = $(wildcard *.rs)
ROBJS = $(patsubst %.rs,$(BUILD_FOLDER)/%.o,$(RSRCS))
EXECS = $(patsubst %.rs,$(BUILD_FOLDER)/%,$(RSRCS))

LIBCORE = $(BUILD_FOLDER)/libcore.rlib

$(shell mkdir -p $(BUILD_FOLDER))

all : $(EXECS)

$(BUILD_FOLDER)/% : $(OBJS) $(ROBJS)
	$(CC) $(CFLAGS) $^ -o $@

$(BUILD_FOLDER)/%.o : %.s
	nasm -f elf32 $< -o $@

$(BUILD_FOLDER)/%.o : %.rs $(LIBCORE)
	rustc -L $(BUILD_FOLDER) $(RFLAGS) --out-dir $(BUILD_FOLDER) --emit obj $<
	
$(LIBCORE) :
	cargo build --manifest-path libcore/Cargo.toml --target $(TARGET)
	cp libcore/target/$(TARGET)/debug/libcore.rlib $(BUILD_FOLDER)

clean :
	rm -rf $(BUILD_FOLDER)/*.o $(EXECS)
	
mrproper :
	cargo clean --manifest-path libcore/Cargo.toml
	rm -rf $(BUILD_FOLDER)

rebuild : clean all