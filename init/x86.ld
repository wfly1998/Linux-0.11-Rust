ENTRY(startup_32)

SECTIONS
{
  . = 0x00000000;
  .text ALIGN(4K):
  {
	  *(.text)
	  *(.text.*)
  }
  .rodata ALIGN(4K):
  {
	  *(.rodata .rodata.*)
  }
  .data ALIGN(4K):
  {
	  *(.data .data.*)
  }
  .bss ALIGN(4K):
  {
	  *(.bss .bss.*)
  }
}
