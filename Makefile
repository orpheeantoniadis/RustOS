KERNEL_PATH = kernel/

.PHONY: all build run clean
	
all: build

build:
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run

test:
	$(MAKE) -C $(KERNEL_PATH) test

clean:
	$(MAKE) -C $(KERNEL_PATH) clean