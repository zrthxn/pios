// Definitions --------------

.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add 	\register, \register, #:lo12:\symbol
.endm

.equ _EL2, 0x8
.equ _core_id_mask, 0b11

// ==========================
// Bootloader ---------------

.section ".text._start"

_start:
	// Only proceed if the core executes in EL2. Park it otherwise.
	mrs	x0, CurrentEL
	cmp	x0, _EL2
	b.ne	.park_loop

  // Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, BOOT_CORE_ID
	cmp	x1, x2
	b.ne	.park_loop

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
	// This ensures that any code in EL2 that needs the stack will work.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to Rust code.
	// x0 holds the function argument provided to _start_rust().
	b	_start_rust

// ==========================
// Park ---------------------

// If this is not the right core
// Infinitely wait for events (aka "park the core").
.park_loop:
	wfe
	b	.park_loop

.size	_start, . - _start
.global	_start
.type	_start, function
