ARCH 				?= riscv64
VERSION				?= debug
MODULES				:= boot mm lib kernel dev
TARGET_DIR 			:= target
DT_DIR 				:= dt
LD_DIR 				:= ./tool/lds
INC_DIR 			:= include


MEMORY 				:= 256
CORE				?= 4

ifeq ($(ARCH), riscv64)
CROSS_COMPILE		:= riscv64-linux-gnu-
QEMU_FLAGS			:= -smp $(CORE) -m $(MEMORY) -nographic -M virt
LDFLAGS        		:= -EL -m elf64lriscv -G 0 -g -static -n -nostdlib --no-relax \
						-N -T $(LD_DIR)/rv64.ld
CFLAGS         		+= --std=gnu99  -g -mcmodel=medany  -ffreestanding -fno-stack-protector \
						-fno-builtin -ffunction-sections  -fpic  -march=rv64gc -mabi=lp64d \
						 -fno-omit-frame-pointer
QEMU				:= qemu-system-riscv64

endif

ifeq ($(VERSION), release)

CFLAGS				+= -O2  -Wall -Werror
LDFLAGS				+= -O2 --fatal-warnings
endif


OBJDUMP				:= $(CROSS_COMPILE)objdump
CC					:= $(CROSS_COMPILE)gcc
LD					:= $(CROSS_COMPILE)ld
OBJCOPY				:= $(CROSS_COMPILE)objcopy
GDB					:= gdb-multiarch