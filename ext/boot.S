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
  bl jump_to_el1
  ldr x0, =_end
  bl mem_init
  bl kmain
  b  1b

/*
activate_mmu:
  ldr x0, =0x8000
  msr ttbr0_el1, x0
  msr ttbr1_el1, x0
  isb

  mov x1, #0
  // Set t0sz to 16
  orr x1, x1, 1 << 5
  // Set t1sz to 16
  orr x1, x1, 1 << 21
  // 40 bits of intermediate physical address size
  orr x1, x1, 1 << 33
  // 4k page in ttbr1_el1
  orr x1, x1, 1 << 31
  // 4k page in ttbr0_el1
  orr x1, x1, 0 << 15
  msr tcr_el1, x1
  isb

  ldr x5, =kmain
  mrs x0, sctlr_el1
  orr x0, x0, #1
  msr sctlr_el1, x0
  isb
  br x5
*/

jump_to_el1:
  mrs x0, currentel
  cmp x0, #0x4 // If we are in el1 already, ret
  beq 1f

  // set stack for el1
  ldr x0, =_start
  msr sp_el1, x0

  // Disable traps to el2 for FP/SIMD operations, setting res bits appropriately
  mov x0, #0x33ff
  msr cptr_el2, x0
  // disable coprocessor traps to el2, (this only applies for aarch32 thumb ops)
  msr hstr_el2, xzr
  mov x0, #3 << 20
  // enable fp/simd at el1
  msr cpacr_el1, x0

  // set el1 to 64 bit
  mov x0, 1 << 31
  msr hcr_el2, x0

  // set D,A,I,F flags on el1 PSTATE to 0 on eret
  mov x0, #0x5
  msr spsr_el2, x0
  // set eret address to 1f
  adr x0, 1f
  msr elr_el2, x0

  // eret to el1
  eret

1:
  ret
