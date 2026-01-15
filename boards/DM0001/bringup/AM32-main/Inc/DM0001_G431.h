/*
 * DM0001_G431.h - ZenDrive Motor Controller Target Definition
 * 
 * Target: DM0001 (Diode Computers ZenDrive)
 * MCU: STM32G431C8T6
 * 
 * Pin Mapping:
 *   Phase A High: PA8  (TIM1_CH1)
 *   Phase A Low:  PC13 (TIM1_CH1N)
 *   Phase B High: PA9  (TIM1_CH2)
 *   Phase B Low:  PA12 (TIM1_CH2N)
 *   Phase C High: PA10 (TIM1_CH3)
 *   Phase C Low:  PB15 (TIM1_CH3N)
 *   
 *   BEMF Phase A: PA4
 *   BEMF Phase B: PB12
 *   BEMF Phase C: PB11
 *   
 *   VBUS Sense:   PA0
 *   Temp Sense:   PB14
 *   PWM Input:    PA15
 *   
 *   CAN RX:       PA11
 *   CAN TX:       PB9
 */

#ifndef DM0001_G431_H_
#define DM0001_G431_H_

#ifdef DM0001_G431

#define FIRMWARE_NAME "DM0001 G431"
#define FILE_NAME "DM0001_G431"
#define DEAD_TIME 80
#define HARDWARE_GROUP_G4_DM0001
#define TARGET_STALL_PROTECTION_INTERVAL 20000
#define USE_SERIAL_TELEMETRY

// Voltage divider: 169k/18k = ~10.39:1
// At 3.3V ADC ref, max measurable = 3.3 * 10.39 = 34.3V
// For 60V max, need different calculation
// TARGET_VOLTAGE_DIVIDER = (R_top + R_bottom) / R_bottom * 10 = (169 + 18) / 18 * 10 = 103.9
#define TARGET_VOLTAGE_DIVIDER 104

// Current sensing via shunt (3mΩ) + op-amp
// This needs calibration based on actual gain
#define MILLIVOLT_PER_AMP 3  // 3mΩ shunt

#endif // DM0001_G431

#ifdef HARDWARE_GROUP_G4_DM0001

#define MCU_G431

// Input capture for DSHOT/PWM on PA15 (TIM2_CH1)
#define USE_TIMER_2_CHANNEL_1
#define INPUT_PIN LL_GPIO_PIN_15
#define INPUT_PIN_PORT GPIOA
#define IC_TIMER_CHANNEL LL_TIM_CHANNEL_CH1
#define IC_TIMER_REGISTER TIM2
#define IC_TIMER_POINTER htim2

#define INPUT_DMA_CHANNEL LL_DMA_CHANNEL_1
#define DMA_HANDLE_TYPE_DEF hdma_tim2_ch1
#define IC_DMA_IRQ_NAME DMA1_Channel1_IRQn

// Phase A: High=PA8 (TIM1_CH1), Low=PC13 (TIM1_CH1N)
#define PHASE_A_GPIO_HIGH LL_GPIO_PIN_8
#define PHASE_A_GPIO_PORT_HIGH GPIOA
#define PHASE_A_GPIO_LOW LL_GPIO_PIN_13
#define PHASE_A_GPIO_PORT_LOW GPIOC

// Phase B: High=PA9 (TIM1_CH2), Low=PA12 (TIM1_CH2N)
#define PHASE_B_GPIO_HIGH LL_GPIO_PIN_9
#define PHASE_B_GPIO_PORT_HIGH GPIOA
#define PHASE_B_GPIO_LOW LL_GPIO_PIN_12
#define PHASE_B_GPIO_PORT_LOW GPIOA

// Phase C: High=PA10 (TIM1_CH3), Low=PB15 (TIM1_CH3N)
#define PHASE_C_GPIO_HIGH LL_GPIO_PIN_10
#define PHASE_C_GPIO_PORT_HIGH GPIOA
#define PHASE_C_GPIO_LOW LL_GPIO_PIN_15
#define PHASE_C_GPIO_PORT_LOW GPIOB

// ============================================================================
// BEMF Sensing Configuration
// ============================================================================
// The DM0001 uses ADC-based BEMF sensing instead of hardware comparators.
// BEMF signals are routed through voltage dividers to ADC-capable pins:
//   Phase A BEMF: PA4  -> ADC2_IN17
//   Phase B BEMF: PB12 -> ADC1_IN11
//   Phase C BEMF: PB11 -> ADC1_IN14
//
// Zero-crossing detection is performed in software by comparing the floating
// phase voltage to a calculated virtual neutral point.
// ============================================================================

// Enable ADC-based BEMF detection (disables hardware comparator mode)
#define USE_ADC_BEMF

// BEMF ADC pin definitions
#define BEMF_A_PIN LL_GPIO_PIN_4
#define BEMF_A_PORT GPIOA
#define BEMF_A_ADC_INSTANCE ADC2
#define BEMF_A_ADC_CHANNEL LL_ADC_CHANNEL_17

#define BEMF_B_PIN LL_GPIO_PIN_12
#define BEMF_B_PORT GPIOB
#define BEMF_B_ADC_INSTANCE ADC1
#define BEMF_B_ADC_CHANNEL LL_ADC_CHANNEL_11

#define BEMF_C_PIN LL_GPIO_PIN_11
#define BEMF_C_PORT GPIOB
#define BEMF_C_ADC_INSTANCE ADC1
#define BEMF_C_ADC_CHANNEL LL_ADC_CHANNEL_14

// Dummy comparator defines for compatibility with AM32 code
// These are not actually used when USE_ADC_BEMF is defined
#define PHASE_A_COMP LL_COMP_INPUT_MINUS_IO1
#define PHASE_B_COMP LL_COMP_INPUT_MINUS_IO1
#define PHASE_C_COMP LL_COMP_INPUT_MINUS_IO2

#define PHASE_A_INPUT_PLUS LL_COMP_INPUT_PLUS_IO1
#define PHASE_B_INPUT_PLUS LL_COMP_INPUT_PLUS_IO1
#define PHASE_C_INPUT_PLUS LL_COMP_INPUT_PLUS_IO1

#define PHASE_A_EXTI_LINE LL_EXTI_LINE_21
#define PHASE_A_COMP_NUMBER COMP1

#define PHASE_B_EXTI_LINE LL_EXTI_LINE_21
#define PHASE_B_COMP_NUMBER COMP1

#define PHASE_C_EXTI_LINE LL_EXTI_LINE_22
#define PHASE_C_COMP_NUMBER COMP2

// Voltage ADC: PA0 (ADC1_IN1)
#define VOLTAGE_ADC_PIN LL_GPIO_PIN_0
#define VOLTAGE_ADC_CHANNEL LL_ADC_CHANNEL_1

// Temperature ADC: PB14 (ADC1_IN5 on G431)
// Note: Need to verify this channel mapping
#define TEMPERATURE_ADC_PIN LL_GPIO_PIN_14
#define TEMPERATURE_ADC_CHANNEL LL_ADC_CHANNEL_5

#endif // HARDWARE_GROUP_G4_DM0001

#endif // DM0001_G431_H_
