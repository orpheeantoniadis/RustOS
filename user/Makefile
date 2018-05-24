ARCH = i386
TARGET = $(ARCH)-app

CC = gcc
LINKER = app.ld
CFLAGS = -T $(LINKER) -m32 -ffreestanding -nostdlib -fno-pie
RUSTC = rustc
RFLAGS = --target=$(TARGET) -Z no-landing-pads -C relocation-model=static

BUILD_FOLDER = build
SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s,$(BUILD_FOLDER)/%.o,$(SRCS))
RSRCS = $(wildcard *.rs)
EXECS = $(patsubst %.rs,$(BUILD_FOLDER)/%,$(RSRCS))

COMMON_PATH = ../common
LIBCORE = $(BUILD_FOLDER)/libcore.rlib
LIBCOMMON = $(BUILD_FOLDER)/libcommon.rlib
ULIBC = ulibc

$(shell mkdir -p $(BUILD_FOLDER))

all : $(EXECS)

$(BUILD_FOLDER)/% : $(OBJS) $(BUILD_FOLDER)/%.o
	$(CC) $(CFLAGS) $^ -o $@

$(BUILD_FOLDER)/%.o : %.s
	nasm -f elf32 $< -o $@

$(BUILD_FOLDER)/%.o : %.rs $(LIBCORE) $(LIBCOMMON)
	$(RUSTC) -L $(BUILD_FOLDER) $(RFLAGS) --out-dir $(BUILD_FOLDER) --emit obj $<

$(LIBCORE) : libcore/lib.rs
	$(RUSTC) --crate-name core --crate-type=rlib $(RFLAGS) --out-dir $(BUILD_FOLDER) $<

$(LIBCOMMON) : $(COMMON_PATH)/src/lib.rs
	$(RUSTC) -L $(BUILD_FOLDER) --crate-name common --crate-type=rlib $(RFLAGS) --out-dir $(BUILD_FOLDER) $<

$(BUILD_FOLDER)/$(ULIBC) : $(OBJS) $(BUILD_FOLDER)/$(ULIBC).o
	@# Exception to pattern rule

$(BUILD_FOLDER)/$(ULIBC).o : $(ULIBC).rs
	@# Exception to pattern rule

clean :
	rm -rf $(BUILD_FOLDER)/*.o $(EXECS)

mrproper :
	rm -rf $(BUILD_FOLDER)

rebuild : clean all