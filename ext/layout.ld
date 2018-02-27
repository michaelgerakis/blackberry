SECTIONS {
  . = 0x80000; /* Raspberry Pi Aarch64 (kernel8.img) load address */

  /* start of the binary */
  _start  = .;

  .text : {
    KEEP(*(.text.boot)) /* from boot.S */
    *(.text .text.* .gnu.linkonce.t*)
  }

  .rodata : {
    *(.rodata .rodata.* .gnu.linkonce.r*)
  }

  .data : {
    *(.data .data.* .gnu.linkonce.d*)
  }

  .bss : {
    __bss_start = .;
    *(.bss .bss.*)
    *(COMMON)
  }

  /* end of the binary */
  _end = ALIGN(8);

  /* number of bytes in BSS section and complete binary */
  __bss_length = (__bss_end - __bss_start);
  __binary_length = (_end - _start);

  /DISCARD/ : { *(.comment) *(.eh_frame*) *(.gnu*) *(.note*) }
}
