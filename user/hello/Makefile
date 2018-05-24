ARCH = i386
TARGET = $(ARCH)-app

CC = xargo
RFLAGS = "-C relocation-model=static"
XFLAGS = -vv --target $(TARGET)

.PHONY : all build test clean
	
all : build

build :
	RUST_TARGET_PATH=$(shell pwd)/.. RUSTFLAGS=$(RFLAGS) $(CC) rustc $(XFLAGS)

clean :
	$(CC) clean