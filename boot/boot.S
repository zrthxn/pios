// Bootloader
.section .text.boot

.global _start

_start:
  mov sp, #0x080000