# Option Bytes for DM0001

These binary files contain option byte values for the STM32G431C8T6 FLASH_OPTR register.

## Files

### `force_flash_boot.bin` (0xFBEFF8AA)
Forces the MCU to always boot from flash memory, ignoring the BOOT0 pin state.

**Use case:** Workaround for v0.0.1 boards where PB8/BOOT0 is connected to Hall encoder and may be pulled high at reset.

```bash
# Write using probe-rs
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFBEFF8AA

# Or flash the binary file directly
probe-rs download --chip STM32G431C8 --base-address 0x1FFF7800 force_flash_boot.bin
```

### `default_boot.bin` (0xFFEFF8AA)
Default boot configuration that reads the BOOT0 pin at reset.

**Use case:** Restore normal boot behavior if you need BOOT0 functionality.

```bash
# Write using probe-rs
probe-rs write --chip STM32G431C8 b32 0x1FFF7800 0xFFEFF8AA

# Or flash the binary file directly
probe-rs download --chip STM32G431C8 --base-address 0x1FFF7800 default_boot.bin
```

## Option Bits Explanation

The FLASH_OPTR register controls boot behavior:

- **nBOOT_SEL** (bit 24): 
  - 0 = Boot according to BOOT0 pin
  - 1 = Boot according to nBOOT0 bit (ignore pin)
  
- **nBOOT0** (bit 27):
  - 0 = Boot from system memory (bootloader)
  - 1 = Boot from main flash

### Values:
- `0xFBEFF8AA`: nBOOT_SEL=1, nBOOT0=1 → Always boot from flash
- `0xFFEFF8AA`: nBOOT_SEL=0, nBOOT0=1 → Read BOOT0 pin, default to flash

## Reading Current Option Bytes

```bash
# Using probe-rs
probe-rs read --chip STM32G431C8 b32 0x1FFF7800 1
```

## Warning

>[!CAUTION]
>Always flash your application firmware BEFORE setting option bytes to force flash boot. Otherwise you may brick the device and need special recovery procedures.

