KERNEL_PATH = kernel/

.PHONY: all build run test clean mrproper
	
all: build

build:
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run

test:
	$(MAKE) -C $(KERNEL_PATH) test

clean:
	$(MAKE) -C $(KERNEL_PATH) clean
	
mrproper:
	$(MAKE) -C $(KERNEL_PATH) mrproper