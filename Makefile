include include.mk

TARGET_ELF 			:= $(TARGET_DIR)/askua.elf




.PHONY: clean run $(MODULES) all

export CC CFLAGS LD LDFLAGS

all: $(TARGET_ELF)


$(TARGET_DIR):
	mkdir -p $@

$(MODULES):
	echo "Building $@"
	$(MAKE) --directory=$@
	


$(TARGET_ELF): $(MODULES) $(TARGET_DIR)
	$(LD) $(LDFLAGS) -o $(TARGET_ELF) $(foreach module, $(MODULES), $(wildcard $(module)/*.o))

.ONESHELL:
clean:
	for d in $(MODULES); do
		$(MAKE) --directory=$$d clean
	done
	rm -rf $(TARGET_DIR) $(DT_DIR)

run: $(TARGET_ELF)
	$(QEMU) -kernel $(TARGET_ELF) $(QEMU_FLAGS)


dts:
	mkdir -p $(DT_DIR)
	$(QEMU) $(QEMU_FLAGS),dumpdtb=$(DT_DIR)/virt.dtb
	dtc -I dtb -O dts -o $(DT_DIR)/virt.dts $(DT_DIR)/virt.dtb

objdump:$(TARGET_ELF)
	$(OBJDUMP) -dS $(TARGET_ELF) > $(TARGET_DIR)/askua.S

dbg-run:
	$(QEMU) -kernel $(TARGET_ELF) $(QEMU_FLAGS) -S -s

dbg:
	$(GDB) -ex "file $(TARGET_ELF)"  -ex "target remote :1234"
