// Definitions --------------

.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add 	\register, \register, #:lo12:\symbol
.endm

.equ _core_id_mask, 0b11

// ==========================
// Bootloader ---------------

.section ".text._start"

.global	_start
.type	_start, function

_start:
  // Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID
	cmp	x1, x2
	b.ne	.park

	// If execution reaches here, it is the boot core.
	// Initialize DRAM.
	ADR_REL	x0, __bss_start
	ADR_REL	x1, __bss_end_exclusive

.bss_init_loop:
	cmp	x0, x1
	b.eq	.init_rust
	stp	xzr, xzr, [x0], #16
	b	.bss_init_loop

// Prepare the jump to Rust code.
.init_rust:
	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to Rust code.
	b	_start_rust

// ==========================
// Park ---------------------

// If this is not the right core
// Infinitely wait for events (aka "park the core").
.park:
	wfe
	b	.park

.size	_start, . - _start
