/* Entry point */
ENTRY(Reset_Handler)

/* Memory Regions */
MEMORY
{
  FLASH  (rx)  : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM    (xrw) : ORIGIN = 0x20000000, LENGTH = 192K
  CCMRAM (rw)  : ORIGIN = 0x10000000, LENGTH = 64K
}

/*
 * The stack is located at the top of RAM.
 * It grows downwards.
 */
_estack = ORIGIN(RAM) + LENGTH(RAM);

/* Heap and stack size definitions */
_Min_Heap_Size  = 0x200;   /* 512 bytes */
_Min_Stack_Size = 0x400;   /* 1 KiB */

