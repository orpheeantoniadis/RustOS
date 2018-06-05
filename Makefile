KERNEL_PATH = kernel/

.PHONY: all build run crun test doc clean mrproper
	
all: build

build:
	$(MAKE) -C $(KERNEL_PATH)
	
run:
	$(MAKE) -C $(KERNEL_PATH) run
	
crun:
	$(MAKE) -C $(KERNEL_PATH) crun

test:
	$(MAKE) -C $(KERNEL_PATH) test
	
doc:
	$(MAKE) -C $(KERNEL_PATH) doc

clean:
	$(MAKE) -C $(KERNEL_PATH) clean
	
mrproper:
	$(MAKE) -C $(KERNEL_PATH) mrproper