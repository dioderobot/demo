Recovered working baseline for the DM0001 ST controller.

Source:
- `boards/DM0001/bringup/st_mcsdk/vendor/NEWMCSDK`

Controller baseline:
- matches the `44ed9f7` ST tune direction
- `AUTORUN_TARGET_SPEED_RPM = 3800`
- `AUTORUN_RAMP_MS = 3400`
- `MAX_APPLICATION_SPEED_RPM = 9000`
- `MOTOR_MAX_SPEED_RPM = 9000`
- TIM1 break inputs remain disabled in `Src/main.c`

Toolchain:
- `xPack GNU Arm Embedded GCC arm64 11.3.1 20220712`
- local path used on this machine:
  `$HOME/.local/xpack-arm-none-eabi-gcc-11.3.1-1.1/bin`

Known-good build artifact:
- ELF:
  `boards/DM0001/bringup/st_mcsdk/vendor/NEWMCSDK/STM32CubeIDE/DM0001Build/NEWMCSDK-dm0001.elf`
- SHA-256:
  `5b629986d22af8ab13794010f53ff0578c5a5c92fcf9ee797e8c00556917b219`

Build:
```bash
bash boards/DM0001/bringup/tools/build_st_mcsdk.sh
```

Flash:
```bash
probe-rs download --chip STM32G431C8 --protocol swd --verify \
  boards/DM0001/bringup/st_mcsdk/vendor/NEWMCSDK/STM32CubeIDE/DM0001Build/NEWMCSDK-dm0001.elf
```
