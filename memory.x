ENTRY(_start)

MEMORY
{
   /* NOTE 1 K = 1 KiBi = 1024 bytes */
   FLASH(rx) : ORIGIN = 0x08000000, LENGTH = 64K
   RAM(rwx) : ORIGIN = 0x20000000, LENGTH = 8K
}

SECTIONS {
   .isr_vector :
   {
      KEEP(*(.isr_vector));
      . = ALIGN(4);
   }> FLASH

   .text : 
   {
      *(.text.boot)
      *(.text)
      . = ALIGN(4);
   }> FLASH

   .rodata :
   {
      *(.rodata)
      *(.rodata.*)
      . = ALIGN(4);
   }> FLASH

   .data : 
   {
      *(.data)
      . = ALIGN(4);
   }> RAM AT> FLASH

   .bss :
   {
      PROVIDE(_bss_start = .);
      *(.bss)
      *(.bss.*)
      . = ALIGN(4);
      PROVIDE(_bss_end = .);
   }> RAM

   PROVIDE(_memory_end = ORIGIN(RAM) + LENGTH(RAM));

   /* Make space for kernel stack. 0x800 = 2K */
   PROVIDE(_stack_start = _bss_end + 0x4);
   PROVIDE(_stack_end = _stack_start + 0x800);

   PROVIDE(_heap_start = _stack_end);
   PROVIDE(_heap_size = _memory_end - _heap_start);
}