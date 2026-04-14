Current best DM0001 motor-control path is the patched ST MCSDK project, not the Rust bringup binaries.

Use this as the starting point:
- ST source clone: `/tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK`
- repo snapshot of the working delta: [st_mcsdk/dm0001_mcsdk_working.patch](/Users/nasheed/GitHub/demo/boards/DM0001/bringup/st_mcsdk/dm0001_mcsdk_working.patch:1)
- rebuild script: [tools/build_st_mcsdk.sh](/Users/nasheed/GitHub/demo/boards/DM0001/bringup/tools/build_st_mcsdk.sh:1)
- live speed sampler: [tools/sample_mcsdk_speed.py](/Users/nasheed/GitHub/demo/boards/DM0001/bringup/tools/sample_mcsdk_speed.py:1)

What is known-good:
- Clocking is HSE-based from the checked-in Zener/module source.
- The current repo source shows Murata `CSTNE8M00GH5C000R0`, so treat DM0001 as `8 MHz HSE -> 170 MHz PLL`.
- TIM1 break inputs from `COMP1/2/4` are temporarily disabled in the ST build. This was required to stop repeated startup fault/retry comb behavior.
- Working bench supply setting is `12.0 V / 5.0 A`.
- Best observed run so far stayed in `RUN` with zero MCSDK faults and the observer reported about `4.58k RPM`.

What not to assume:
- There are no usable Hall sensors in this setup for control.
- The Rust FOC / AM32-style binaries are not the current bringup path.
- The board is not a literal drop-in for ST auxiliary/UI pins, only the motor-control-critical pins line up.

Motor / board assumptions currently used:
- motor: `212 920KV`, `14 poles`, `7 pole pairs`
- target motor voltage range for bringup: `7-12 V`
- ST parameterization uses `MOTOR_VOLTAGE_CONSTANT ~= 1.1 V/krpm`
- current ST tune uses `5 A` controller headroom because the DPS-150 is the practical bench limit

Safe flash / run sequence:
1. DPS off
2. DPS on at `8.0 V / 1.0 A`
3. `probe-rs download --chip STM32G431C8 --protocol swd --verify <elf>`
4. DPS off
5. DPS on at `12.0 V / 5.0 A` to run the motor image
6. DPS off again when done

Useful commands:
```bash
PATH=/Users/nasheed/.local/ArmGNUToolchain/bin:$PATH \
ARM_GCC=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-gcc \
ARM_SIZE=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-size \
ARM_OBJDUMP=/Users/nasheed/.local/ArmGNUToolchain/bin/arm-none-eabi-objdump \
bash boards/DM0001/bringup/tools/build_st_mcsdk.sh \
  /tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK
```

```bash
python3 boards/DM0001/bringup/tools/sample_mcsdk_speed.py \
  --elf /tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK/STM32CubeIDE/DM0001Build/NEWMCSDK-dm0001.elf \
  --duration 8 --period 0.1 --attach-timeout 4
```

Current tuning direction:
- The best gains so far were a modest speed-loop bump, not a large one.
- Big jumps in target speed caused regressions.
- The current best tune is:
  - speed PI about `Kp=3200`, `Ki=6`
  - autorun target `4200 RPM`
  - phase-5 rev-up target `3800 RPM`
  - `5 A` nominal/current headroom
- The next likely experiments are:
  - slower top-end ramp while keeping the same gains
  - lower PWM frequency experiment (`40 kHz -> 20-30 kHz`) if more top-end voltage utilization is needed
  - better runtime telemetry for bus current / bus voltage during the speed hold

Current repo state caveat:
- unrelated dirty files still exist in:
  - `boards/DM0001/bringup/firmware/src/bin/spin_am32_style.rs`
  - `boards/DM0001/bringup/firmware/src/bin/spin_foc_style.rs`
  - `boards/DM0001/layout/fp-lib-table`
  - `boards/DM0001/layout/layout.kicad_pcb`
- do not revert those accidentally when making commits
