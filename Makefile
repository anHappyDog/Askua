include include.mk


TARGET_DIR	:= target/$(ARCH)/$(VERSION)
TARGET 		:= $(TARGET_DIR)/askua

LIBS		:= $(TARGET_DIR)/libaskua.a lib/target/libclib.a
LD_SCRIPT_DIR 	:= ./lds


ifeq ($(ARCH),rv64)
LD_SCRIPT	:= $(LD_SCRIPT_DIR)/rv64.ld
else ifeq ($(ARCH),aarch64)
LD_SCRIPT	:= $(LD_SCRIPT_DIR)/aarch64.ld
endif

.PHONY: clean

$(TARGET): all

all: 
	export RUSTC_WRAPPER=sscache
	$(MAKE) -C lib
	$(MAKE) -C fs $(FSTYPE)
	$(CARGO) build $(CARGO_FLAGS)

run: $(TARGET)
	$(QEMU) $(QEMU_FLAGS) -kernel $(TARGET)

gdb:
	$(GDB) -ex "target remote localhost:1234" -ex "file $(TARGET)"

gdb-run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(TARGET) -s -S

objdump:all
	$(OBJDUMP) -DS $(TARGET) > $(TARGET).asm

clean:
	cargo clean
	$(MAKE) -C lib clean