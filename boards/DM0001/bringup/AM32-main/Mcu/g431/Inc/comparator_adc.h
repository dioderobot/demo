/*
 * comparator_adc.h
 *
 * ADC-based BEMF zero-crossing detection for DM0001
 * Replaces hardware comparator with software comparison of ADC samples
 *
 * Created for DM0001 which uses ADC pins for BEMF sensing:
 *   Phase A: PA4  -> ADC2_IN17
 *   Phase B: PB12 -> ADC1_IN11
 *   Phase C: PB11 -> ADC1_IN14 / ADC2_IN14
 */

#ifndef COMPARATOR_ADC_H_
#define COMPARATOR_ADC_H_

#include "main.h"
#include <stdint.h>

// BEMF ADC channels for DM0001
#define BEMF_A_ADC         ADC2
#define BEMF_A_CHANNEL     LL_ADC_CHANNEL_17   // PA4 -> ADC2_IN17
#define BEMF_B_ADC         ADC1
#define BEMF_B_CHANNEL     LL_ADC_CHANNEL_11   // PB12 -> ADC1_IN11
#define BEMF_C_ADC         ADC1
#define BEMF_C_CHANNEL     LL_ADC_CHANNEL_14   // PB11 -> ADC1_IN14

// Virtual neutral point calculation method
// The neutral point is estimated as (VA + VB + VC) / 3
// Zero crossing occurs when floating phase voltage crosses neutral

// ADC sample buffer for BEMF
typedef struct {
    uint16_t phase_a;
    uint16_t phase_b;
    uint16_t phase_c;
    uint16_t neutral;      // Calculated virtual neutral point
    uint16_t floating;     // Current floating phase value
} bemf_adc_t;

extern bemf_adc_t bemf_adc;

// Initialize ADC channels for BEMF sensing
void BEMF_ADC_Init(void);

// Sample all three BEMF phases
void BEMF_ADC_Sample(void);

// Get the current floating phase ADC value
uint16_t BEMF_ADC_GetFloatingPhase(void);

// Get the virtual neutral point
uint16_t BEMF_ADC_GetNeutral(void);

// Check if zero crossing detected (returns 1 if crossed)
// This replaces getCompOutputLevel() for ADC mode
uint8_t BEMF_ADC_GetCrossingState(void);

// Configure which phase to monitor based on commutation step
// This replaces changeCompInput() for ADC mode
void BEMF_ADC_SelectPhase(void);

// Compatibility functions that match the comparator.h interface
void maskPhaseInterrupts_ADC(void);
void enableCompInterrupts_ADC(void);
void changeCompInput_ADC(void);
uint8_t getCompOutputLevel_ADC(void);

// External variables from main.c
extern char rising;
extern char step;

#endif /* COMPARATOR_ADC_H_ */
