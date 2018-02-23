// To keep this in the first portion of the binary.
.section .text.boot

// Make _start global
.global _start

// Entry point for the kernel.
_start:
  // read cpu affinity into x1
  mrs x3, mpidr_el1
  // Start core 0 and halt rest. 7:0 in the mpidr_el1 indicates the core number
  and x3, x3, #3
  // & with #3 will ensure that only 0x0 will pass the next statement
  cbz x3, 2f

1:
  // Wait for event
  wfe
  b 1b

2:
  // set the stack to start before our boot code
  ldr x4, =_start
  mov sp, x4

  // Load the start address and number of bytes in BSS section
  ldr x4, =__bss_start
  ldr x5, =__bss_length

3:
  // zero out the BSS section, 64 bits at a time
  cbz  x5,  4f
  // xzr is the zero register in 64 byte
  str  xzr, [x4], #8
  sub  x5,  x5, #8
  cbnz x5,  3b

4:
  // jump to kmain, which shouldn't return. halt if it does
  bl kmain
  b  1b
