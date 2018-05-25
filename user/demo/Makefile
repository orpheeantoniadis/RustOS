ARCH = i386
TARGET = $(ARCH)-app

CC = xargo
RFLAGS = "-C relocation-model=static -C opt-level=3"
XFLAGS = --release -vv --target $(TARGET)

.PHONY : all build test clean
	
all : build

build :
	RUST_TARGET_PATH=$(shell pwd)/.. RUSTFLAGS=$(RFLAGS) $(CC) build $(XFLAGS)

clean :
	$(CC) clean