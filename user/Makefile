ARCH = i386
TARGET = $(ARCH)-app
BUILD_FOLDER = build

LINKER = app.ld
FLAGS = -T $(LINKER) -m32 -MMD -g -ffreestanding -nostdlib -Wall -Wextra -fno-pie

APPS = hello demo shell

SRCS = $(wildcard *.s)
OBJS = $(patsubst %.s, $(BUILD_FOLDER)/%.o, $(SRCS))
ROBJS = $(foreach A,$(APPS),$A/target/$(TARGET)/release/lib$A.a)
EXECS = $(patsubst %, $(BUILD_FOLDER)/%, $(APPS))

all : build

build : $(EXECS)
	
$(EXECS) : $(ROBJS)
	
%.a : $(OBJS)
	$(MAKE) -C $(shell echo "$@" | cut -d "/" -f1)
	gcc $(FLAGS) $^ $@ -o $(BUILD_FOLDER)/$(shell echo "$@" | cut -d "/" -f1)
	
$(BUILD_FOLDER)/%.o : %.s
	$(shell mkdir -p $(BUILD_FOLDER))
	nasm -f elf32 $< -o $@
	
clean :
	$(MAKE) -C hello clean
	$(MAKE) -C demo clean
	$(MAKE) -C shell clean
	rm -rf build
	
rebuild : clean build
	