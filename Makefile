ARCH = i386
CC = xargo
TARGET = $(ARCH)-rust_os
FLAGS = -v --target $(TARGET)
KERNEL_PATH = kernel/

.PHONY: all build run clean
	
all: build

build:
	RUST_TARGET_PATH=$(shell pwd) $(CC) build $(FLAGS)
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run

test:
	cargo test

clean:
	$(MAKE) -C $(KERNEL_PATH) clean
	$(CC) clean