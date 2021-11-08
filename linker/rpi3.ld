/* LinkerScript for RaspberryPi 3B+ and related models */

__load_addr = 0x80000;

ENTRY(__load_addr)

PHDRS
{
  segment_rx PT_LOAD FLAGS(5); /* 5 == RX */
  segment_rw PT_LOAD FLAGS(6); /* 6 == RW */
}

SECTIONS
{
  . =  __load_addr;
  __boot_core_stack_end_exclusive = .;

  .text :
  {
    KEEP(*(.text._start))
    *(.text._start_arguments) /* Constants (or statics in Rust speak) read by _start(). */
    *(.text._start_rust)      /* The Rust entry point */
    *(.text*)                 /* Everything else */
  } :segment_rx

  .rodata : ALIGN(8) { *(.rodata*) } :segment_rx
  .got    : ALIGN(8) { *(.got)     } :segment_rx

  .data : { *(.data*) } :segment_rw

  /* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
  .bss : ALIGN(16)
  {
    __bss_start = .;
    *(.bss*);
    . = ALIGN(16);
    __bss_end_exclusive = .;
  } :NONE
}
