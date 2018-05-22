ARCH = i386
BUILD_FOLDER = build

LINKER = app.ld
FLAGS = -T $(LINKER) -m32 -MMD -g -ffreestanding -nostdlib -Wall -Wextra -fno-pie

SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
EXEC = hello
RUST = ../target/$(ARCH)-app/debug/libhello.a

.PHONY : all bin clean

all : bin

bin : $(BUILD_FOLDER)/$(EXEC)

$(BUILD_FOLDER)/$(EXEC) : $(OBJS) $(RUST)
	gcc $(FLAGS) $^ -o $@

$(BUILD_FOLDER)/%.o : %.s
	mkdir -p $(shell dirname $@)
	nasm -f elf32 $< -o $@
	
$(RUST) :
	$(MAKE) -C ../
	
clean :
	rm -rf $(BUILD_FOLDER)