ARCH = i386
CC = xargo
TARGET = $(ARCH)-rust_os
FLAGS = -v --target $(TARGET)
SOURCE_PATH = src/

.PHONY: all build run clean
	
all: build

build:
	RUST_TARGET_PATH=$(shell pwd) $(CC) build $(FLAGS)
	$(MAKE) -C $(SOURCE_PATH)
	
run:
	$(MAKE) -C $(SOURCE_PATH) run

test:
	cargo test

clean:
	$(MAKE) -C $(SOURCE_PATH) clean
	$(CC) clean