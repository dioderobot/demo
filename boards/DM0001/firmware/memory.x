/* STM32G431CBU6 Memory Layout */
/* 128KB FLASH, 32KB RAM (22KB RAM + 10KB CCM RAM) */

MEMORY
{
  /* NOTE: You must modify 'memory.x' to match your target chip's memory layout */
  /* STM32G431CB has 128KB Flash starting at 0x08000000 */
  FLASH : ORIGIN = 0x08000000, LENGTH = 128K
  
  /* STM32G431CB has 22KB SRAM1 + 6KB SRAM2 + 10KB CCM SRAM */
  /* Main RAM (SRAM1 + SRAM2) */
  RAM : ORIGIN = 0x20000000, LENGTH = 28K
  
  /* CCM RAM - Can be added if needed for specific use */
  /* CCMRAM : ORIGIN = 0x10000000, LENGTH = 10K */
}
