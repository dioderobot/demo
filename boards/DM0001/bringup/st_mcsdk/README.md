This directory now contains the DM0001 ST MCSDK source of truth in-repo at `boards/DM0001/bringup/st_mcsdk/vendor/NEWMCSDK`.

The old patch file is kept as a compact record of the original DM0001-specific delta against the upstream ST `B-G431B-ESC1` MCSDK project.

Files:
- `vendor/NEWMCSDK`: vendored ST MCSDK project, trimmed to source-only contents and buildable from repo.
- `dm0001_mcsdk_working.patch`: historical DM0001 delta against the upstream ST project.
- `WORKING_BASELINE.md`: exact recovered baseline, compiler line, and ELF checksum for the currently working image.
- `../tools/sample_mcsdk_speed.py`: SWD sampler for live MCSDK state, observer speed, and virtual startup speed.

Rebuild:
```bash
bash boards/DM0001/bringup/tools/build_st_mcsdk.sh
```

Toolchain note:
- the recovered working baseline was rebuilt with `xPack GNU Arm Embedded GCC 11.3.1`
- `build_st_mcsdk.sh` now prefers `~/.local/xpack-arm-none-eabi-gcc-11.3.1-1.1/bin` automatically if it exists
- earlier GCC 15 rebuilds were not behaviorally equivalent on hardware

Sample live RPM estimate:
```bash
python3 boards/DM0001/bringup/tools/sample_mcsdk_speed.py \
  --elf boards/DM0001/bringup/st_mcsdk/vendor/NEWMCSDK/STM32CubeIDE/DM0001Build/NEWMCSDK-dm0001.elf \
  --duration 8 --period 0.1 --attach-timeout 4
```

Current debug mode:
- `vendor/NEWMCSDK/Src/mc_app_hooks.c` is intentionally in single-shot autorun mode.
- It performs one delayed startup attempt, keeps refreshing the speed command only while in `RUN`, and does not auto-retry after a drop or fault.
- This is specifically to debug the current symptom: `spin up -> fall back down`, without the hook masking it by immediately restarting.

Latest bench result:
- `12.0 V / 5.0 A`
- earlier best observed run: MCSDK stayed `RUN`, `fault_now = 0`, `fault_past = 0`, observer peak about `4.58k RPM`
- current vendored single-shot image is for debugging repeatable drop-out, not for peak-speed characterization

Clock note:
- The checked-in DM0001 Zener source references the STM32 module at `reference/STM32G431C8T6/STM32G431C8T6.zen`.
- That module instantiates Murata `CSTNE8M00GH5C000R0` and defaults `crystal_frequency` to `8MHz`.
- The current ST build therefore keeps the HSE-based `8 MHz -> 170 MHz PLL` clocking path.
