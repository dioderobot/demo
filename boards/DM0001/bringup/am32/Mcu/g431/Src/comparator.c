///*
// * comparator.c
// *
// *  Created on: Sep. 26, 2020
// *      Author: Alka
// */
//
// #include "comparator.h"
// #include "targets.h"
//
//
// void maskPhaseInterrupts(){
//	EXTI->IMR1 &= ~(1 << 18);
//	EXTI->RPR1 = EXTI_LINE;
//	EXTI->FPR1 = EXTI_LINE;
////	LL_EXTI_ClearRisingFlag_0_31(EXTI_LINE);
////	LL_EXTI_ClearFallingFlag_0_31(EXTI_LINE);
//}
//
// void enableCompInterrupts(){
//    EXTI->IMR1 |= (1 << 18);
//}
//

/*
 * comparator.c
 *
 *  Created on: Sep. 26, 2020
 *      Author: Alka
 *
 *  Modified for DM0001: ADC-based BEMF zero-crossing detection
 */

#include "comparator.h"

#include "common.h"
#include "targets.h"

#ifdef USE_ADC_BEMF

/* ADC-based BEMF detection for DM0001 */
static volatile ADC_TypeDef* active_bemf_adc;
static volatile uint32_t active_bemf_channel;
extern uint16_t ADC_raw_volts;

static uint16_t readBemfAdc(ADC_TypeDef* adc, uint32_t channel)
{
    LL_ADC_REG_SetSequencerRanks(adc, LL_ADC_REG_RANK_1, channel);
    LL_ADC_REG_StartConversion(adc);
    while (!LL_ADC_IsActiveFlag_EOC(adc));
    uint16_t val = LL_ADC_REG_ReadConversionData12(adc);
    LL_ADC_ClearFlag_EOC(adc);
    return val;
}

uint8_t getCompOutputLevel()
{
    /* Read the floating phase BEMF voltage via ADC */
    uint16_t bemf = readBemfAdc((ADC_TypeDef*)active_bemf_adc, active_bemf_channel);
    /* Compare against virtual neutral = VBUS/2 */
    uint16_t threshold = ADC_raw_volts / 2;
    return (bemf > threshold) ? 1 : 0;
}

void maskPhaseInterrupts()
{
    /* No hardware comp interrupts to mask in ADC BEMF mode */
}

void enableCompInterrupts()
{
    /* No hardware comp interrupts in ADC BEMF mode */
}

void changeCompInput()
{
    if (step == 1 || step == 4) { // c floating
        active_bemf_adc = BEMF_C_ADC;
        active_bemf_channel = BEMF_C_CHANNEL;
    }
    if (step == 2 || step == 5) { // a floating
        active_bemf_adc = BEMF_A_ADC;
        active_bemf_channel = BEMF_A_CHANNEL;
    }
    if (step == 3 || step == 6) { // b floating
        active_bemf_adc = BEMF_B_ADC;
        active_bemf_channel = BEMF_B_CHANNEL;
    }
}

#else /* Hardware comparator BEMF detection */

COMP_TypeDef* active_COMP = COMP2;
uint32_t current_EXTI_LINE = LL_EXTI_LINE_22;

uint8_t getCompOutputLevel()
{
    return LL_COMP_ReadOutputLevel(active_COMP);
}

void maskPhaseInterrupts()
{
    EXTI->IMR1 &= ~(1 << 21);
    EXTI->IMR1 &= ~(1 << 22);
    EXTI->PR1 = LL_EXTI_LINE_22;
    EXTI->PR1 = LL_EXTI_LINE_21;
}

void enableCompInterrupts() { EXTI->IMR1 |= current_EXTI_LINE; }

void changeCompInput()
{
    if (step == 1 || step == 4) { // c floating

        current_EXTI_LINE = PHASE_C_EXTI_LINE;
        active_COMP = PHASE_C_COMP_NUMBER;

        LL_COMP_ConfigInputs(active_COMP, PHASE_C_COMP, PHASE_C_INPUT_PLUS);
    }

    if (step == 2 || step == 5) { // a floating

        current_EXTI_LINE = PHASE_A_EXTI_LINE;
        active_COMP = PHASE_A_COMP_NUMBER;

        LL_COMP_ConfigInputs(active_COMP, PHASE_A_COMP, PHASE_A_INPUT_PLUS);
    }

    if (step == 3 || step == 6) { // b floating

        current_EXTI_LINE = PHASE_B_EXTI_LINE;
        active_COMP = PHASE_B_COMP_NUMBER;

        LL_COMP_ConfigInputs(active_COMP, PHASE_B_COMP, PHASE_B_INPUT_PLUS);
    }
    if (rising) {
        LL_EXTI_DisableRisingTrig_0_31(LL_EXTI_LINE_22);
        LL_EXTI_DisableRisingTrig_0_31(LL_EXTI_LINE_21);
        LL_EXTI_EnableFallingTrig_0_31(current_EXTI_LINE);
    } else { // falling bemf
        LL_EXTI_EnableRisingTrig_0_31(current_EXTI_LINE);
        LL_EXTI_DisableFallingTrig_0_31(LL_EXTI_LINE_21);
        LL_EXTI_DisableFallingTrig_0_31(LL_EXTI_LINE_22);
    }
}

#endif /* USE_ADC_BEMF */

// void changeCompInput() {
//	if (step == 1 || step == 4) {   // c floating
//		COMP2->CSR = 0x000281;
//	}

//	if (step == 2 || step == 5) {     // a floating
//		COMP2->CSR = 0x000261;
//	}

//	if (step == 3 || step == 6) {      // b floating
//		COMP2->CSR = 0x000271;
//	}
//	if (rising){
//		  EXTI->RTSR1 &= ~(LL_EXTI_LINE_18);
//		  EXTI->FTSR1 |= LL_EXTI_LINE_18;
//	}else{                          // falling bemf
//		  EXTI->RTSR1 |= LL_EXTI_LINE_18;
//		  EXTI->FTSR1 &= ~(LL_EXTI_LINE_18);
//	}
//}
// void changeCompInput() {
//	if (step == 1 || step == 4) {   // c floating
// #ifdef INVERTED_COMP
//		COMP2->CSR = (((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL
//<< (8U))))) | (COMP_COMMON | PHASE_C_COMP)); #else 		COMP2->CSR =
//(((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL << (8U))))) | (PHASE_C_COMP
//|
// LL_COMP_INPUT_PLUS_IO3)); #endif
//	}

//	if (step == 2 || step == 5) {     // a floating
// #ifdef INVERTED_COMP
//		COMP2->CSR = (((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL
//<< (8U))))) | (COMP_COMMON | PHASE_A_COMP)); #else 		COMP2->CSR =
//(((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL << (8U))))) | (PHASE_A_COMP
//|
// LL_COMP_INPUT_PLUS_IO3)); #endif

//	}
//	if (step == 3 || step == 6) {      // b floating
// #ifdef INVERTED_COMP
//		COMP2->CSR = (((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL
//<< (8U))))) | (COMP_COMMON | PHASE_B_COMP)); #else 		COMP2->CSR =
//(((((COMP2->CSR))) & (~((0xFUL << (4U)) | (0x3UL << (8U))))) | (PHASE_B_COMP
//|
// LL_COMP_INPUT_PLUS_IO3)); #endif

//	}

//	if (rising){
//		  EXTI->RTSR1 &= ~(LL_EXTI_LINE_18);
//		  EXTI->FTSR1 |= LL_EXTI_LINE_18;
//	}else{                          // falling bemf
//		  EXTI->RTSR1 |= LL_EXTI_LINE_18;
//		  EXTI->FTSR1 &= ~(LL_EXTI_LINE_18);

//	}
//}
