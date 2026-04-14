#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEFAULT_PROJECT_DIR="$(cd "$SCRIPT_DIR/../st_mcsdk/vendor/NEWMCSDK" && pwd)"

PROJECT_DIR="${1:-$DEFAULT_PROJECT_DIR}"
BUILD_DIR="${BUILD_DIR:-$PROJECT_DIR/STM32CubeIDE/DM0001Build}"
LINKER_SCRIPT="$PROJECT_DIR/STM32CubeIDE/STM32G431CBUX_FLASH.ld"

XPACK_BIN_DEFAULT="${HOME}/.local/xpack-arm-none-eabi-gcc-11.3.1-1.1/bin"
if [[ -x "$XPACK_BIN_DEFAULT/arm-none-eabi-gcc" ]]; then
  ARM_GCC_DEFAULT="$XPACK_BIN_DEFAULT/arm-none-eabi-gcc"
  ARM_SIZE_DEFAULT="$XPACK_BIN_DEFAULT/arm-none-eabi-size"
  ARM_OBJDUMP_DEFAULT="$XPACK_BIN_DEFAULT/arm-none-eabi-objdump"
else
  ARM_GCC_DEFAULT="arm-none-eabi-gcc"
  ARM_SIZE_DEFAULT="arm-none-eabi-size"
  ARM_OBJDUMP_DEFAULT="arm-none-eabi-objdump"
fi

ARM_GCC="${ARM_GCC:-$ARM_GCC_DEFAULT}"
ARM_SIZE="${ARM_SIZE:-$ARM_SIZE_DEFAULT}"
ARM_OBJDUMP="${ARM_OBJDUMP:-$ARM_OBJDUMP_DEFAULT}"

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

compile_s() {
  local src="$1"
  local obj="$2"
  mkdir -p "$(dirname "$obj")"
  "$ARM_GCC" \
    "${COMMON_FLAGS[@]}" \
    "${DEFS[@]}" \
    "${INCLUDES[@]}" \
    -x assembler-with-cpp \
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
  "$PROJECT_DIR/Src/system_stm32g4xx.c"
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

HAL_BASENAMES=(
  stm32g4xx_hal
  stm32g4xx_hal_adc
  stm32g4xx_hal_adc_ex
  stm32g4xx_hal_comp
  stm32g4xx_hal_cordic
  stm32g4xx_hal_cortex
  stm32g4xx_hal_dac
  stm32g4xx_hal_dac_ex
  stm32g4xx_hal_dma
  stm32g4xx_hal_dma_ex
  stm32g4xx_hal_exti
  stm32g4xx_hal_flash
  stm32g4xx_hal_flash_ex
  stm32g4xx_hal_flash_ramfunc
  stm32g4xx_hal_gpio
  stm32g4xx_hal_opamp
  stm32g4xx_hal_opamp_ex
  stm32g4xx_hal_pwr
  stm32g4xx_hal_pwr_ex
  stm32g4xx_hal_rcc
  stm32g4xx_hal_rcc_ex
  stm32g4xx_hal_tim
  stm32g4xx_hal_tim_ex
  stm32g4xx_hal_uart
  stm32g4xx_hal_uart_ex
  stm32g4xx_ll_adc
)

SUPPORT_C_SRCS=(
  "$PROJECT_DIR/STM32CubeIDE/Application/User/syscalls.c"
  "$PROJECT_DIR/STM32CubeIDE/Application/User/sysmem.c"
)

SUPPORT_ASM_SRCS=(
  "$PROJECT_DIR/STM32CubeIDE/Application/Startup/startup_stm32g431cbux.s"
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

SUPPORT_OBJS=()
for base in "${HAL_BASENAMES[@]}"; do
  src="$PROJECT_DIR/Drivers/STM32G4xx_HAL_Driver/Src/$base.c"
  if [[ ! -f "$src" ]]; then
    echo "missing HAL source for $base" >&2
    exit 1
  fi
  obj="$BUILD_DIR/Drivers/$(basename "${src%.c}").o"
  compile_c "$src" "$obj"
  SUPPORT_OBJS+=("$obj")
done

for src in "${SUPPORT_C_SRCS[@]}"; do
  obj="$BUILD_DIR/Application/User/$(basename "${src%.c}").o"
  compile_c "$src" "$obj"
  SUPPORT_OBJS+=("$obj")
done

for src in "${SUPPORT_ASM_SRCS[@]}"; do
  obj="$BUILD_DIR/Application/Startup/$(basename "${src%.s}").o"
  compile_s "$src" "$obj"
  SUPPORT_OBJS+=("$obj")
done

ELF="$BUILD_DIR/NEWMCSDK-dm0001.elf"
MAP="$BUILD_DIR/NEWMCSDK-dm0001.map"
LIST="$BUILD_DIR/NEWMCSDK-dm0001.list"

"$ARM_GCC" \
  -o "$ELF" \
  "${USER_OBJS[@]}" \
  "${MCLIB_OBJS[@]}" \
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
