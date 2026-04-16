# Makefile for DM0001 ZenDrive Motor Controller - STM32G431C8T6
# Based on ST Motor Control SDK v6.4.2 FOC firmware

######################################
# Target
######################################
TARGET = DM0001_FOC

######################################
# Building variables
######################################
DEBUG = 1
OPT = -Ofast

######################################
# Paths
######################################
BUILD_DIR = build

# Toolchain - Arm GNU Toolchain with newlib
ARM_TOOLCHAIN ?= $(HOME)/.local/arm-gnu-toolchain-15.2.rel1-darwin-arm64-arm-none-eabi
PREFIX = $(ARM_TOOLCHAIN)/bin/arm-none-eabi-
CC = $(PREFIX)gcc
AS = $(PREFIX)gcc -x assembler-with-cpp
CP = $(PREFIX)objcopy
SZ = $(PREFIX)size
HEX = $(CP) -O ihex
BIN = $(CP) -O binary -S

######################################
# Source files
######################################

# ASM source
ASM_SOURCES = \
STM32CubeIDE/Application/Startup/startup_stm32g431cbux.s

# Application sources
C_SOURCES = \
Src/main.c \
Src/stm32g4xx_it.c \
Src/stm32g4xx_hal_msp.c \
Src/system_stm32g4xx.c \
Src/motorcontrol.c \
Src/mc_tasks.c \
Src/mc_tasks_foc.c \
Src/mc_config.c \
Src/mc_config_common.c \
Src/mc_configuration_registers.c \
Src/mc_parameters.c \
Src/mc_interface.c \
Src/mc_api.c \
Src/mc_math.c \
Src/mc_app_hooks.c \
Src/mcp_config.c \
Src/mcp.c \
Src/aspep.c \
Src/usart_aspep_driver.c \
Src/regular_conversion_manager.c \
Src/pwm_curr_fdbk.c \
Src/pwm_common.c \
Src/speed_torq_ctrl.c \
Src/stm32_mc_common_it.c \
Src/stm32g4xx_mc_it.c \
Src/hf_registers.c \
Src/sync_registers.c

# STM32CubeIDE support files
C_SOURCES += \
STM32CubeIDE/Application/User/syscalls.c \
STM32CubeIDE/Application/User/sysmem.c

# MCSDK MCLib - Any (platform-independent)
MCSDK_BASE = MCSDK_v6.4.2-Full/MotorControl/MCSDK/MCLib
C_SOURCES += \
$(MCSDK_BASE)/Any/Src/bus_voltage_sensor.c \
$(MCSDK_BASE)/Any/Src/circle_limitation.c \
$(MCSDK_BASE)/Any/Src/digital_output.c \
$(MCSDK_BASE)/Any/Src/esc.c \
$(MCSDK_BASE)/Any/Src/mcpa.c \
$(MCSDK_BASE)/Any/Src/mcptl.c \
$(MCSDK_BASE)/Any/Src/ntc_temperature_sensor.c \
$(MCSDK_BASE)/Any/Src/pid_regulator.c \
$(MCSDK_BASE)/Any/Src/pqd_motor_power_measurement.c \
$(MCSDK_BASE)/Any/Src/r_divider_bus_voltage_sensor.c \
$(MCSDK_BASE)/Any/Src/ramp_ext_mngr.c \
$(MCSDK_BASE)/Any/Src/revup_ctrl.c \
$(MCSDK_BASE)/Any/Src/speed_pos_fdbk.c \
$(MCSDK_BASE)/Any/Src/sto_pll_speed_pos_fdbk.c \
$(MCSDK_BASE)/Any/Src/virtual_bus_voltage_sensor.c \
$(MCSDK_BASE)/Any/Src/virtual_speed_sensor.c

# MCSDK MCLib - G4xx (STM32G4-specific)
C_SOURCES += \
$(MCSDK_BASE)/G4xx/Src/r3_2_g4xx_pwm_curr_fdbk.c

# HAL drivers (only enabled modules)
HAL_DRV = Drivers/STM32G4xx_HAL_Driver/Src
C_SOURCES += \
$(HAL_DRV)/stm32g4xx_hal.c \
$(HAL_DRV)/stm32g4xx_hal_adc.c \
$(HAL_DRV)/stm32g4xx_hal_adc_ex.c \
$(HAL_DRV)/stm32g4xx_hal_comp.c \
$(HAL_DRV)/stm32g4xx_hal_cordic.c \
$(HAL_DRV)/stm32g4xx_hal_cortex.c \
$(HAL_DRV)/stm32g4xx_hal_dac.c \
$(HAL_DRV)/stm32g4xx_hal_dac_ex.c \
$(HAL_DRV)/stm32g4xx_hal_dma.c \
$(HAL_DRV)/stm32g4xx_hal_dma_ex.c \
$(HAL_DRV)/stm32g4xx_hal_exti.c \
$(HAL_DRV)/stm32g4xx_hal_flash.c \
$(HAL_DRV)/stm32g4xx_hal_flash_ex.c \
$(HAL_DRV)/stm32g4xx_hal_flash_ramfunc.c \
$(HAL_DRV)/stm32g4xx_hal_gpio.c \
$(HAL_DRV)/stm32g4xx_hal_opamp.c \
$(HAL_DRV)/stm32g4xx_hal_opamp_ex.c \
$(HAL_DRV)/stm32g4xx_hal_pwr.c \
$(HAL_DRV)/stm32g4xx_hal_pwr_ex.c \
$(HAL_DRV)/stm32g4xx_hal_rcc.c \
$(HAL_DRV)/stm32g4xx_hal_rcc_ex.c \
$(HAL_DRV)/stm32g4xx_hal_tim.c \
$(HAL_DRV)/stm32g4xx_hal_tim_ex.c \
$(HAL_DRV)/stm32g4xx_hal_uart.c \
$(HAL_DRV)/stm32g4xx_hal_uart_ex.c

# LL drivers used by MCSDK
C_SOURCES += \
$(HAL_DRV)/stm32g4xx_ll_adc.c \
$(HAL_DRV)/stm32g4xx_ll_comp.c \
$(HAL_DRV)/stm32g4xx_ll_dma.c \
$(HAL_DRV)/stm32g4xx_ll_exti.c \
$(HAL_DRV)/stm32g4xx_ll_gpio.c \
$(HAL_DRV)/stm32g4xx_ll_opamp.c \
$(HAL_DRV)/stm32g4xx_ll_rcc.c \
$(HAL_DRV)/stm32g4xx_ll_tim.c \
$(HAL_DRV)/stm32g4xx_ll_usart.c \
$(HAL_DRV)/stm32g4xx_ll_dac.c \
$(HAL_DRV)/stm32g4xx_ll_utils.c

######################################
# Include paths
######################################
C_INCLUDES = \
-IInc \
-IDrivers/STM32G4xx_HAL_Driver/Inc \
-IDrivers/STM32G4xx_HAL_Driver/Inc/Legacy \
-I$(MCSDK_BASE)/Any/Inc \
-I$(MCSDK_BASE)/G4xx/Inc \
-IDrivers/CMSIS/Device/ST/STM32G4xx/Include \
-IDrivers/CMSIS/Include \
-IDrivers/CMSIS/DSP/Include

######################################
# Defines
######################################
C_DEFS = \
-DUSE_HAL_DRIVER \
-DSTM32G431xx \
-DARM_MATH_CM4

######################################
# MCU flags
######################################
CPU = -mcpu=cortex-m4
FPU = -mfpu=fpv4-sp-d16
FLOAT-ABI = -mfloat-abi=hard
MCU = $(CPU) -mthumb $(FPU) $(FLOAT-ABI)

######################################
# Compiler flags
######################################
ASFLAGS = $(MCU) $(C_DEFS) -Wall -fdata-sections -ffunction-sections

CFLAGS = $(MCU) $(C_DEFS) $(C_INCLUDES) $(OPT) -Wall -fdata-sections -ffunction-sections -fsingle-precision-constant -include stdint.h

ifeq ($(DEBUG), 1)
CFLAGS += -g3 -gdwarf-2
endif

# Generate dependency files
CFLAGS += -MMD -MP -MF"$(@:%.o=%.d)"

######################################
# Linker flags
######################################
LDSCRIPT = STM32CubeIDE/STM32G431CBUX_FLASH.ld

LIBS = -lc -lm -lnosys
LDFLAGS = $(MCU) -specs=nano.specs -T$(LDSCRIPT) $(LIBS) -Wl,-Map=$(BUILD_DIR)/$(TARGET).map,--cref -Wl,--gc-sections -Wl,--print-memory-usage

######################################
# Build rules
######################################
all: $(BUILD_DIR)/$(TARGET).elf $(BUILD_DIR)/$(TARGET).hex $(BUILD_DIR)/$(TARGET).bin

# List of C objects
OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))

# List of ASM objects
OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
vpath %.s $(sort $(dir $(ASM_SOURCES)))

$(BUILD_DIR)/%.o: %.c Makefile | $(BUILD_DIR)
	$(CC) -c $(CFLAGS) -Wa,-a,-ad,-alms=$(BUILD_DIR)/$(notdir $(<:.c=.lst)) $< -o $@

$(BUILD_DIR)/%.o: %.s Makefile | $(BUILD_DIR)
	$(AS) -c $(ASFLAGS) $< -o $@

$(BUILD_DIR)/$(TARGET).elf: $(OBJECTS) Makefile
	$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	$(SZ) $@

$(BUILD_DIR)/%.hex: $(BUILD_DIR)/%.elf | $(BUILD_DIR)
	$(HEX) $< $@

$(BUILD_DIR)/%.bin: $(BUILD_DIR)/%.elf | $(BUILD_DIR)
	$(BIN) $< $@

$(BUILD_DIR):
	mkdir -p $@

######################################
# Clean
######################################
clean:
	rm -rf $(BUILD_DIR)

######################################
# Flash (using ST-Link via OpenOCD)
######################################
flash: $(BUILD_DIR)/$(TARGET).bin
	openocd -f interface/stlink.cfg -f target/stm32g4x.cfg -c "program $(BUILD_DIR)/$(TARGET).elf verify reset exit"

######################################
# Dependencies
######################################
-include $(wildcard $(BUILD_DIR)/*.d)

.PHONY: all clean flash
