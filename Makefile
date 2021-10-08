LINKER_FILE = bsp/rpi3/linker.ld
export LINKER_FILE

define colorecho
	@tput setaf 6 2> /dev/null || true
	@echo $1
	@tput sgr0 2> /dev/null || true
endef


# =====================================
# Variables----------------------------

BOOT_SRC = boot/boot.S
BOOT_OBJ = boot/boot.o
TARGET_CPU = cortex-a53
TARGET_ARCH = armv8-a
TARGET_RUSTC = aarch64-unknown-none-softfloat
KERNEL_ELF = target/$(TARGET_RUSTC)/release/pios

# =====================================
# Cmdlets -----------------------------

# Bootloader assembler 
ASSEMBLER_CMD = aarch64-none-elf-gcc $(ASSEMBLER_ARGS)
ASSEMBLER_ARGS = \
	-mcpu=$(TARGET_CPU)							\
	-mtune=$(TARGET_CPU)						\
	-march=$(TARGET_ARCH)						\
	-mlittle-endian 								\
	-fpic														\


# rustc compiler
RUSTC_CMD = cargo rustc $(RUSTC_ARGS)
RUSTC_ARGS = \
	--target=$(TARGET_RUSTC)	\
	--features bsp_rpi3 			\
	--release 								\

RUSTC_TCPU = -C target-cpu=$(TARGET_CPU)
RUSTC_LINK = -C link-arg=-T$(LINKER_FILE) -C link-arg=$(BOOT_OBJ)
# RUSTC_NICE = -D warnings -D missing_docs
RUSTFLAGS = $(RUSTC_TCPU) $(RUSTC_LINK) #$(RUSTC_NICE)


# =====================================
# Targets -----------------------------

.PHONY: all boot clean qemu

all: boot build clean qemu

qemu:
	qemu-system-aarch64 -M raspi3 -d in_asm -display none -kernel $(KERNEL_ELF)

boot:
	$(call colorecho, "Assembling Bootloader")
	$(ASSEMBLER_CMD) -c $(BOOT_SRC) -o $(BOOT_OBJ)

build:
	$(call colorecho, "Compiling Kernel")
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

clean:
	$(call colorecho, "Cleanup")
	rm $(BOOT_OBJ) 

