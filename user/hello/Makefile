ARCH = i386
CC = xargo
TARGET = $(ARCH)-app
FLAGS = -v --target $(TARGET)
SOURCE_PATH = src/

.PHONY: all build test clean
	
all: build

build:
	RUST_TARGET_PATH=$(shell pwd) $(CC) build $(FLAGS)
	$(MAKE) -C $(SOURCE_PATH)

test:
	cargo test

clean:
	$(MAKE) -C $(SOURCE_PATH) clean
	$(CC) clean