ARCH = i386
CC = xargo
TARGET = $(ARCH)-rust_os
FLAGS = -v --no-default-features --target $(TARGET)
KERNEL_PATH = kernel/

.PHONY: all build run clean
	
all: build

build:
	$(CC) build $(FLAGS)
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run

clean:
	$(MAKE) -C $(KERNEL_PATH) clean
	$(CC) clean