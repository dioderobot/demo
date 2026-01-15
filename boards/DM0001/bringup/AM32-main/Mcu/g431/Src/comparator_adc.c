/*
 * comparator_adc.c
 *
 * ADC-based BEMF zero-crossing detection for DM0001
 * 
 * This implementation replaces hardware comparator-based BEMF detection
 * with software-based ADC sampling and zero-crossing detection.
 *
 * The DM0001 board routes BEMF signals through voltage dividers to ADC pins:
 *   Phase A BEMF: PA4  -> ADC2_IN17
 *   Phase B BEMF: PB12 -> ADC1_IN11  
 *   Phase C BEMF: PB11 -> ADC1_IN14
 *
 * Zero-crossing detection algorithm:
 * 1. Sample all three phase voltages via ADC
 * 2. Calculate virtual neutral point: Vn = (Va + Vb + Vc) / 3
 * 3. Compare floating phase voltage to neutral point
 * 4. Detect crossing when floating phase crosses neutral
 *
 * Created for AM32 firmware port to DM0001
 */

#include "comparator_adc.h"
#include "targets.h"

#ifdef USE_ADC_BEMF

// BEMF ADC sample storage
bemf_adc_t bemf_adc = {0};

// Track which phase is currently floating (0=A, 1=B, 2=C)
static uint8_t floating_phase = 0;

// Previous crossing state for edge detection
static uint8_t prev_crossing_state = 0;

// Hysteresis threshold to prevent noise-induced false crossings
// Value in ADC counts (12-bit ADC, so 0-4095 range)
#define BEMF_HYSTERESIS 20

/**
 * @brief Initialize ADC channels for BEMF sensing
 * 
 * Configures PA4, PB12, PB11 as analog inputs for BEMF measurement.
 * Uses injected ADC channels for fast sampling during PWM off-time.
 */
void BEMF_ADC_Init(void)
{
    LL_GPIO_InitTypeDef GPIO_InitStruct = {0};
    
    // Enable GPIO clocks
    LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOA);
    LL_AHB2_GRP1_EnableClock(LL_AHB2_GRP1_PERIPH_GPIOB);
    
    // Configure PA4 as analog (BEMF Phase A)
    GPIO_InitStruct.Pin = LL_GPIO_PIN_4;
    GPIO_InitStruct.Mode = LL_GPIO_MODE_ANALOG;
    GPIO_InitStruct.Pull = LL_GPIO_PULL_NO;
    LL_GPIO_Init(GPIOA, &GPIO_InitStruct);
    
    // Configure PB12 as analog (BEMF Phase B)
    GPIO_InitStruct.Pin = LL_GPIO_PIN_12;
    LL_GPIO_Init(GPIOB, &GPIO_InitStruct);
    
    // Configure PB11 as analog (BEMF Phase C)
    GPIO_InitStruct.Pin = LL_GPIO_PIN_11;
    LL_GPIO_Init(GPIOB, &GPIO_InitStruct);
    
    // Note: ADC peripheral initialization is handled in ADC.c
    // We just need to configure the injected channels for BEMF sampling
}

/**
 * @brief Sample a single ADC channel (blocking)
 * @param ADCx ADC peripheral (ADC1 or ADC2)
 * @param channel ADC channel to sample
 * @return 12-bit ADC value
 */
static uint16_t BEMF_ADC_ReadChannel(ADC_TypeDef* ADCx, uint32_t channel)
{
    // Configure the channel for single conversion
    LL_ADC_REG_SetSequencerRanks(ADCx, LL_ADC_REG_RANK_1, channel);
    LL_ADC_SetChannelSamplingTime(ADCx, channel, LL_ADC_SAMPLINGTIME_2CYCLES_5);
    
    // Start conversion
    LL_ADC_REG_StartConversion(ADCx);
    
    // Wait for conversion complete
    while (!LL_ADC_IsActiveFlag_EOC(ADCx)) {
        // Timeout protection could be added here
    }
    
    // Read and return result
    return LL_ADC_REG_ReadConversionData12(ADCx);
}

/**
 * @brief Sample all three BEMF phases
 * 
 * Reads ADC values for all three phases and calculates the virtual neutral point.
 * Should be called during PWM off-time for accurate BEMF measurement.
 */
void BEMF_ADC_Sample(void)
{
    // Sample all three phases
    // Note: For better performance, these could be done with DMA
    bemf_adc.phase_a = BEMF_ADC_ReadChannel(BEMF_A_ADC, BEMF_A_CHANNEL);
    bemf_adc.phase_b = BEMF_ADC_ReadChannel(BEMF_B_ADC, BEMF_B_CHANNEL);
    bemf_adc.phase_c = BEMF_ADC_ReadChannel(BEMF_C_ADC, BEMF_C_CHANNEL);
    
    // Calculate virtual neutral point: Vn = (Va + Vb + Vc) / 3
    bemf_adc.neutral = (bemf_adc.phase_a + bemf_adc.phase_b + bemf_adc.phase_c) / 3;
    
    // Update floating phase value based on current commutation step
    switch (floating_phase) {
        case 0: // Phase A floating
            bemf_adc.floating = bemf_adc.phase_a;
            break;
        case 1: // Phase B floating
            bemf_adc.floating = bemf_adc.phase_b;
            break;
        case 2: // Phase C floating
            bemf_adc.floating = bemf_adc.phase_c;
            break;
    }
}

/**
 * @brief Get the current floating phase ADC value
 * @return ADC value of the floating phase
 */
uint16_t BEMF_ADC_GetFloatingPhase(void)
{
    return bemf_adc.floating;
}

/**
 * @brief Get the virtual neutral point
 * @return Calculated neutral point ADC value
 */
uint16_t BEMF_ADC_GetNeutral(void)
{
    return bemf_adc.neutral;
}

/**
 * @brief Check if zero crossing detected
 * 
 * Compares the floating phase voltage to the virtual neutral point.
 * Returns the crossing state based on whether we're looking for rising or falling BEMF.
 *
 * @return 1 if floating phase is above neutral (for rising BEMF detection)
 *         0 if floating phase is below neutral
 */
uint8_t BEMF_ADC_GetCrossingState(void)
{
    // Sample the BEMF voltages
    BEMF_ADC_Sample();
    
    // Compare floating phase to neutral with hysteresis
    if (rising) {
        // Looking for falling BEMF (floating phase going below neutral)
        // Return 1 when floating > neutral (not yet crossed)
        // Return 0 when floating < neutral (crossed)
        if (bemf_adc.floating > (bemf_adc.neutral + BEMF_HYSTERESIS)) {
            return 1;
        } else if (bemf_adc.floating < (bemf_adc.neutral - BEMF_HYSTERESIS)) {
            return 0;
        }
    } else {
        // Looking for rising BEMF (floating phase going above neutral)
        // Return 0 when floating < neutral (not yet crossed)
        // Return 1 when floating > neutral (crossed)
        if (bemf_adc.floating < (bemf_adc.neutral - BEMF_HYSTERESIS)) {
            return 0;
        } else if (bemf_adc.floating > (bemf_adc.neutral + BEMF_HYSTERESIS)) {
            return 1;
        }
    }
    
    // Within hysteresis band, return previous state
    return prev_crossing_state;
}

/**
 * @brief Configure which phase to monitor based on commutation step
 * 
 * In 6-step commutation, one phase is always floating (not driven).
 * This function selects which phase to monitor for BEMF zero-crossing.
 *
 * Step 1,4: Phase C floating
 * Step 2,5: Phase A floating  
 * Step 3,6: Phase B floating
 */
void BEMF_ADC_SelectPhase(void)
{
    if (step == 1 || step == 4) {
        // Phase C floating
        floating_phase = 2;
    } else if (step == 2 || step == 5) {
        // Phase A floating
        floating_phase = 0;
    } else if (step == 3 || step == 6) {
        // Phase B floating
        floating_phase = 1;
    }
    
    // Reset crossing state for new phase
    prev_crossing_state = 0;
}

/*
 * Compatibility functions that match the comparator.h interface
 * These allow the main AM32 code to work with minimal changes
 */

void maskPhaseInterrupts_ADC(void)
{
    // No hardware interrupts to mask in ADC mode
    // This is a no-op for polling-based ADC BEMF
}

void enableCompInterrupts_ADC(void)
{
    // No hardware interrupts in ADC mode
    // Zero-crossing is detected by polling in the main loop
}

void changeCompInput_ADC(void)
{
    // Select the floating phase based on commutation step
    BEMF_ADC_SelectPhase();
}

uint8_t getCompOutputLevel_ADC(void)
{
    // Return the zero-crossing state from ADC comparison
    uint8_t state = BEMF_ADC_GetCrossingState();
    prev_crossing_state = state;
    return state;
}

#endif /* USE_ADC_BEMF */
