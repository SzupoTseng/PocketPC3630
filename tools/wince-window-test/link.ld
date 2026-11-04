/* Laid out as the PE image will be: headers at 0x10000, then one page-
   aligned section each for code, read-only data and data. elf2pe.py adds
   the import directory after them. */
ENTRY(entry)
SECTIONS {
  . = 0x11000;
  .text : { *(.text .text.*) }
  . = ALIGN(0x1000);
  .rdata : { *(.rodata .rodata.*) }
  . = ALIGN(0x1000);
  .data : { *(.iat) *(.data .data.*) *(.bss .bss.* COMMON) }
  /DISCARD/ : { *(.ARM.exidx*) *(.ARM.extab*) *(.comment) *(.ARM.attributes) }
}
