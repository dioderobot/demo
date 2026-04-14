#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="${1:-/tmp/eirbot-B-G431B-ESC1-guide/project/NEWMCSDK}"
BUILD_DIR="${BUILD_DIR:-$PROJECT_DIR/STM32CubeIDE/DM0001Build}"
DEBUG_DIR="$PROJECT_DIR/STM32CubeIDE/Debug"
LINKER_SCRIPT="$PROJECT_DIR/STM32CubeIDE/STM32G431CBUX_FLASH.ld"

ARM_GCC="${ARM_GCC:-arm-none-eabi-gcc}"
ARM_SIZE="${ARM_SIZE:-arm-none-eabi-size}"
ARM_OBJDUMP="${ARM_OBJDUMP:-arm-none-eabi-objdump}"

MCLIB_DIR="$PROJECT_DIR/MCSDK_v6.2.0-Full/MotorControl/MCSDK/MCLib"
LIBMP_DIR="$PROJECT_DIR/MCSDK_v6.2.0-Full/MotorControl/libMP"

if [[ ! -d "$PROJECT_DIR" ]]; then
  echo "project not found: $PROJECT_DIR" >&2
  exit 1
fi

mkdir -p "$BUILD_DIR"

COMMON_FLAGS=(
  -mcpu=cortex-m4
  -mthumb
  -mfpu=fpv4-sp-d16
  -mfloat-abi=hard
  -Ofast
  -g0
  -ffunction-sections
  -fdata-sections
  -fmessage-length=0
  -Wall
)

DEFS=(
  -DDEBUG
  -DARM_MATH_CM4
  -DUSE_HAL_DRIVER
  -DSTM32G431xx
)

INCLUDES=(
  -I"$PROJECT_DIR/Inc"
  -I"$PROJECT_DIR/Drivers/STM32G4xx_HAL_Driver/Inc"
  -I"$PROJECT_DIR/Drivers/STM32G4xx_HAL_Driver/Inc/Legacy"
  -I"$PROJECT_DIR/Drivers/CMSIS/Device/ST/STM32G4xx/Include"
  -I"$PROJECT_DIR/Drivers/CMSIS/Include"
  -I"$PROJECT_DIR/Drivers/CMSIS/DSP/Include"
  -I"$MCLIB_DIR/Any/Inc"
  -I"$MCLIB_DIR/G4xx/Inc"
)

compile_c() {
  local src="$1"
  local obj="$2"
  mkdir -p "$(dirname "$obj")"
  "$ARM_GCC" \
    "${COMMON_FLAGS[@]}" \
    "${DEFS[@]}" \
    "${INCLUDES[@]}" \
    --specs=nano.specs \
    -std=gnu11 \
    -c "$src" \
    -o "$obj"
}

find_mclib_source() {
  local base="$1"
  find "$MCLIB_DIR" -path "*/Src/$base.c" | head -n 1
}

USER_SRCS=(
  "$PROJECT_DIR/Src/aspep.c"
  "$PROJECT_DIR/Src/main.c"
  "$PROJECT_DIR/Src/mc_api.c"
  "$PROJECT_DIR/Src/mc_app_hooks.c"
  "$PROJECT_DIR/Src/mc_config.c"
  "$PROJECT_DIR/Src/mc_configuration_registers.c"
  "$PROJECT_DIR/Src/mc_interface.c"
  "$PROJECT_DIR/Src/mc_math.c"
  "$PROJECT_DIR/Src/mc_parameters.c"
  "$PROJECT_DIR/Src/mc_perf.c"
  "$PROJECT_DIR/Src/mc_tasks.c"
  "$PROJECT_DIR/Src/mcp.c"
  "$PROJECT_DIR/Src/mcp_config.c"
  "$PROJECT_DIR/Src/motorcontrol.c"
  "$PROJECT_DIR/Src/pwm_curr_fdbk.c"
  "$PROJECT_DIR/Src/register_interface.c"
  "$PROJECT_DIR/Src/regular_conversion_manager.c"
  "$PROJECT_DIR/Src/stm32g4xx_hal_msp.c"
  "$PROJECT_DIR/Src/stm32g4xx_it.c"
  "$PROJECT_DIR/Src/stm32g4xx_mc_it.c"
  "$PROJECT_DIR/Src/usart_aspep_driver.c"
)

MCLIB_BASENAMES=(
  bus_voltage_sensor
  circle_limitation
  digital_output
  mcpa
  ntc_temperature_sensor
  open_loop
  pid_regulator
  pqd_motor_power_measurement
  pwm_common
  r3_2_g4xx_pwm_curr_fdbk
  r_divider_bus_voltage_sensor
  ramp_ext_mngr
  revup_ctrl
  speed_pos_fdbk
  speed_torq_ctrl
  sto_pll_speed_pos_fdbk
  virtual_speed_sensor
)

USER_OBJS=()
for src in "${USER_SRCS[@]}"; do
  obj="$BUILD_DIR/Application/User/$(basename "${src%.c}").o"
  compile_c "$src" "$obj"
  USER_OBJS+=("$obj")
done

MCLIB_OBJS=()
for base in "${MCLIB_BASENAMES[@]}"; do
  src="$(find_mclib_source "$base")"
  if [[ -z "$src" ]]; then
    echo "missing MCLib source for $base" >&2
    exit 1
  fi
  obj="$BUILD_DIR/Middlewares/MotorControl/$base.o"
  compile_c "$src" "$obj"
  MCLIB_OBJS+=("$obj")
done

DRIVER_OBJS=()
while IFS= read -r obj; do
  DRIVER_OBJS+=("$obj")
done < <(find "$DEBUG_DIR/Drivers" -name '*.o' | sort)

SUPPORT_OBJS=()
while IFS= read -r obj; do
  SUPPORT_OBJS+=("$obj")
done < <(
  {
    find "$DEBUG_DIR/Application/Startup" -name '*.o'
    find "$DEBUG_DIR/Application/User" \( -name 'syscalls.o' -o -name 'sysmem.o' \)
  } | sort
)

ELF="$BUILD_DIR/NEWMCSDK-dm0001.elf"
MAP="$BUILD_DIR/NEWMCSDK-dm0001.map"
LIST="$BUILD_DIR/NEWMCSDK-dm0001.list"

"$ARM_GCC" \
  -o "$ELF" \
  "${USER_OBJS[@]}" \
  "${MCLIB_OBJS[@]}" \
  "${DRIVER_OBJS[@]}" \
  "${SUPPORT_OBJS[@]}" \
  -mcpu=cortex-m4 \
  -mthumb \
  -mfpu=fpv4-sp-d16 \
  -mfloat-abi=hard \
  -T"$LINKER_SCRIPT" \
  --specs=nosys.specs \
  -Wl,-Map="$MAP" \
  -Wl,--gc-sections \
  -static \
  -L"$LIBMP_DIR" \
  --specs=nano.specs \
  -Wl,--start-group \
  -l:libmp-IAR_ARMv7-M.a \
  -lc \
  -lm \
  -Wl,--end-group

"$ARM_SIZE" "$ELF"
"$ARM_OBJDUMP" -h -S "$ELF" > "$LIST"

echo "$ELF"
