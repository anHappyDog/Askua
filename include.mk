ARCH 				:= riscv64
MODULES				:= boot
TARGET_DIR 			:= target
DT_DIR 				:= dt
LD_DIR 				:= ./tool/lds
INC_DIR 			:= include


MEMORY 				:= 256
CORE				?= 4

ifeq ($(ARCH), riscv64)
CROSS_COMPILE		:= riscv64-linux-gnu-
CFLAGS				:= -march=rv64gc -mabi=lp64d -mcmodel=medany -nostdlib -nostartfiles
QEMU_FLAGS			:= -smp $(CORE) -m $(MEMORY) -nographic -M virt
LDFLAGS        		:= -EL -m elf64lriscv -G 0 -static -n -nostdlib --no-relax --fatal-warnings \
						-N -T $(LD_DIR)/rv64.ld
CFLAGS         		:= --std=gnu99 -Werror  -mcmodel=medany  -ffreestanding -fno-stack-protector \
						-fno-builtin -ffunction-sections -Wall -fpic  -march=rv64g -mabi=lp64d
QEMU				:= qemu-system-riscv64
OBJDUMP				:= $(CROSS_COMPILE)objdump
CC					:= $(CROSS_COMPILE)gcc
LD					:= $(CROSS_COMPILE)ld

endif