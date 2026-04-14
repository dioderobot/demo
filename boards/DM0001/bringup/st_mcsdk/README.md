This directory snapshots the working DM0001-specific delta against the cloned ST `B-G431B-ESC1` MCSDK project in `/tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK`.

Files:
- `dm0001_mcsdk_working.patch`: tuned motor parameters, autorun hook, and the temporary TIM1 break-path disable that produced the smooth run on bench.

Rebuild:
```bash
PATH=/Users/nasheed/.local/ArmGNUToolchain/bin:$PATH \
ARM_GCC=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-gcc \
ARM_SIZE=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-size \
ARM_OBJDUMP=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-objdump \
bash boards/DM0001/bringup/tools/build_st_mcsdk.sh \
  /tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK
```

Clock note:
- The checked-in DM0001 Zener source references the STM32 module at `reference/STM32G431C8T6/STM32G431C8T6.zen`.
- That module instantiates Murata `CSTNE8M00GH5C000R0` and defaults `crystal_frequency` to `8MHz`.
- The current ST build therefore keeps the HSE-based `8 MHz -> 170 MHz PLL` clocking path.
