// To keep this in the first portion of the binary.
.section .text.boot

// Make _start global
.global _start

// Entry point for the kernel.
// r15 -> should begin executation at 0x80000.
// r0  -> 0x00000000
// r1  -> 0x00000C42
// r2  -> 0x00000100 - start of ATAGS
// preserve these registers as argument for kernel_main
_start:
  // read cpu affinity into x1
  mrs x1, mpidr_el1
  // Start core 0 and halt rest. 7:0 in the mpidr_el1 indicates the core number
  and x1, x1, #3
  // & with #3 will ensure that only 0x0 will pass the next statement
  cbz x1, 2f

1:
  // Wait for event
  wfe
  b 1b

2:
  // set the stack to start before our boot code
  ldr x1, =_start
  mov sp, x1

  // Load the start address and number of bytes in BSS section
  ldr x1, =__bss_start
  ldr x2, =__bss_length

3:
  // zero out the BSS section, 64 bits at a time
  cbz  x2,  4f
  // xzr is the zero register in 64 byte
  str  xzr, [x1], #8
  sub  x2,  x2, #8
  cbnz x2,  3b

4:
  // jump to kain, which shouldn't return. halt if it does
  bl kmain
  b  1b
