CC = xargo
KERNEL_PATH = kernel/
TARGET = x86_64-rust_os

.PHONY: all build run clean
	
all: build

build:
	RUST_TARGET_PATH=$(shell pwd) $(CC) build --target $(TARGET)
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run

clean:
	$(MAKE) -C $(KERNEL_PATH) clean
	$(CC) clean