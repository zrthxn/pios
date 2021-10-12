define colorecho
	@tput setaf 6 2> /dev/null || true
	@echo $1
	@tput sgr0 2> /dev/null || true
endef


# =====================================
# Variables----------------------------

LINKER_FILE = linker/rpi3.ld
export LINKER_FILE

TARGET_ARCH = aarch64
TARGET_CPU = cortex-a53
TARGET_RUSTC = aarch64-unknown-none-softfloat

BOOT_SRC = boot/$(TARGET_ARCH).S
BOOT_OBJ = boot/$(TARGET_ARCH).o
KERNEL_ELF = target/$(TARGET_RUSTC)/release/pios
KERNEL_BIN = target/$(TARGET_RUSTC)/release/kernel8.img

BSP ?= rpi3
DEV_SERIAL ?= /dev/ttyUSB0

# =====================================
# Cmdlets -----------------------------

# Bootloader assembler 
ASSEMBLER_CMD = aarch64-none-elf-gcc $(ASSEMBLER_ARGS)
ASSEMBLER_ARGS = \
	-mcpu=$(TARGET_CPU)     \
	-mtune=$(TARGET_CPU)    \
	-mlittle-endian         \
	-mfix-cortex-a53-835769 \
	-fpic                   \


# rustc compiler
RUSTC_CMD = cargo rustc $(RUSTC_ARGS)
RUSTC_ARGS = \
	--target=$(TARGET_RUSTC)  \
	--features bsp_rpi4       \
	--release                 \

RUSTC_TCPU = -C target-cpu=$(TARGET_CPU)
RUSTC_LINK = -C link-arg=-T$(LINKER_FILE) -C link-arg=$(BOOT_OBJ)
# RUSTC_NICE = -D warnings -D missing_docs
RUSTFLAGS = $(RUSTC_TCPU) $(RUSTC_LINK)# $(RUSTC_NICE)

# QEMU run
OBJCOPY = aarch64-none-elf-objcopy --strip-all -O binary
QEMU_CMD = qemu-system-aarch64 $(QEMU_ARGS)
QEMU_ARGS = \
	-M raspi3     \
	-display none \
	-serial stdio \


# =====================================
# Targets -----------------------------

.PHONY: all boot build kernel clean qemu

all: boot build kernel clean

boot:
	$(call colorecho, "Assembling Bootloader")
	$(ASSEMBLER_CMD) -c $(BOOT_SRC) -o $(BOOT_OBJ)

build: boot
	$(call colorecho, "Compiling Kernel")
	RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

kernel: boot build
	$(call colorecho, "Building Kernel image")
	$(OBJCOPY) $(KERNEL_ELF) $(KERNEL_BIN)

qemu: boot build kernel
	$(call colorecho, "Run QEMU")
	$(QEMU_CMD) -kernel $(KERNEL_BIN)

clean:
	$(call colorecho, "Cleanup")
	rm $(BOOT_OBJ) 
