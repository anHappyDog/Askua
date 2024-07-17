VERSION     ?= debug
ARCH 		?= rv64
TARGET_DIR  := ./target
DT_DIR      := ./dt
INC_DIR     := include
MEMORY      := 256
NCORE       ?= 2

ROOT_FSIMG  := ext4-fs.img
FSTYPE 		:= ext4

CROSS_COMPILE   := riscv64-unknown-elf-
QEMU_FLAGS      := -smp $(NCORE) -m $(MEMORY) -nographic \
                    -device virtio-blk-device,drive=hd0,bus=virtio-mmio-bus.0 \
                    -drive if=none,file=fs/$(ROOT_FSIMG),format=raw,id=hd0 -M virt
LDFLAGS         := -EL -m elf64lriscv -static -n -nostdlib --relax  -z max-page-size=4096
CFLAGS          += --std=gnu99 -mcmodel=medany -ffreestanding -fno-stack-protector -fno-builtin \
                    -ffunction-sections -fno-pic -march=rv64g -mabi=lp64d -fno-omit-frame-pointer
CARGO_FLAGS		:= 
CARGO 			:= cargo
QEMU            := qemu-system-riscv64

ifeq ($(VERSION), debug)
	CFLAGS += -g -O0
	LDFLAGS += -G 0 -g
else
	CARGO_FLAGS += --release
	CFLAGS += -O2 -Wall -DLOG_LEVEL=0
	LDFLAGS += --gc-sections --fatal-warnings -G 2
endif

OBJDUMP     := $(CROSS_COMPILE)objdump
CC          := $(CROSS_COMPILE)gcc
LD          := $(CROSS_COMPILE)ld
AR 			:= $(CROSS_COMPILE)ar
GDB         := gdb-multiarch
HOST_CC     := gcc
HOST_CFLAGS += --std=gnu99 -O2 -Wall
HOST_ENDIAN := $(shell lscpu | grep -iq 'little endian' && echo EL || echo EB)


CLIB_DIR 	:= ./target