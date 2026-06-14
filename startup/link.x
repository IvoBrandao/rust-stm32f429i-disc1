/* Linker script for Cortex-M4 (STM32F429ZI)
 *
 * Memory regions are provided by memory.x in the application crate.
 * This file is copied to OUT_DIR by startup/build.rs and then passed
 * to the linker via -Tlink.x.
 */

INCLUDE memory.x

ENTRY(Reset)

/* Force the linker to include the vector table even if nothing references it. */
EXTERN(__RESET_VECTOR);
EXTERN(__EXCEPTIONS);

SECTIONS
{
  /* ---- Flash ------------------------------------------------------------ */

  /* Vector table must be at the very start of Flash (0x0800_0000).         */
  .vector_table ORIGIN(FLASH) :
  {
    /* Word 0: initial stack pointer = top of RAM */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Word 1: Reset handler */
    KEEP(*(.vector_table.reset_vector));

    /* Words 2-15: remaining Cortex-M exception vectors */
    KEEP(*(.vector_table.exceptions));
  } > FLASH

  /* Executable code */
  .text :
  {
    *(.text .text.*);
  } > FLASH

  /* Read-only data (constants, string literals) */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  /* ---- RAM -------------------------------------------------------------- */

  /* Initialised globals: stored in Flash, copied to RAM by Reset().
   * AT(...) sets the load address (LMA) in Flash immediately after .rodata. */
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    __sdata = .;
    *(.data .data.*);
    __edata = .;
  } > RAM

  /* Load address of .data in Flash — used by Reset() as copy source. */
  __sidata = LOADADDR(.data);

  /* Uninitialised globals: allocated in RAM, zeroed by Reset(). */
  .bss (NOLOAD) :
  {
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    __ebss = .;
  } > RAM

  /* ---- Discard ---------------------------------------------------------- */

  /DISCARD/ :
  {
    /* ARM exception index / unwind tables (not used in bare-metal Rust) */
    *(.ARM.exidx .ARM.exidx.*);
    *(.ARM.extab .ARM.extab.*);
  }
}
