ARCH = i386
CC = xargo
TARGET = $(ARCH)-rust_os
FLAGS = -v --target $(TARGET)
SOURCE_PATH = src/

.PHONY: all build run crun test clean mrproper
	
all: build

build:
	RUST_TARGET_PATH=$(shell pwd) $(CC) build $(FLAGS)
	$(MAKE) -C $(SOURCE_PATH)
	
run:
	$(MAKE) -C $(SOURCE_PATH) run
	
crun:
	$(MAKE) -C $(SOURCE_PATH) crun

test:
	cargo test
	
doc :
	$(CC) doc --open

clean:
	$(MAKE) -C $(SOURCE_PATH) clean
	$(CC) clean
	
mrproper:
	$(MAKE) -C $(SOURCE_PATH) mrproper
	$(CC) clean