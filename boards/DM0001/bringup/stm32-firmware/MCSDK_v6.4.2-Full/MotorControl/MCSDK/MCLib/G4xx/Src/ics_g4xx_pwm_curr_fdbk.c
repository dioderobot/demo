/**
  ******************************************************************************
  * @file    ics_g4xx_pwm_curr_fdbk.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides firmware functions that implement current sensor
  *          class to be stantiated when the three shunts current sensing
  *          topology is used.
  * 
  *          It is specifically designed for STM32G4X microcontrollers and
  *          implements the successive sampling of motor current using two ADCs.
  *           + MCU peripheral and handle initialization function
  *           + three shunt current sensing
  *           + space vector modulation function
  *           + ADC sampling function
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2026 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under Ultimate Liberty license
  * SLA0044, the "License"; You may not use this file except in compliance with
  * the License. You may obtain a copy of the License at:
  *                             www.st.com/SLA0044
  *
  ******************************************************************************
  * @ingroup ICS_G4XX_pwm_curr_fdbk
  */

/* Includes ------------------------------------------------------------------*/
#include "ics_g4xx_pwm_curr_fdbk.h"
#include "pwm_common.h"
#include "mc_type.h"

/** @addtogroup MCSDK
  * @{
  */


/** @addtogroup ConvFOC
  * @{
  */

/** @addtogroup MCLLAPI
  * @{
  */
/** @addtogroup pwm_curr_fdbk
  * @{
  */

/** @addtogroup ICS_pwm_curr_fdbk
  * @{
  */

/* Private defines -----------------------------------------------------------*/
#define TIMxCCER_MASK_CH123 ((uint16_t)(LL_TIM_CHANNEL_CH1|LL_TIM_CHANNEL_CH1N|\
                                        LL_TIM_CHANNEL_CH2|LL_TIM_CHANNEL_CH2N|\
                                        LL_TIM_CHANNEL_CH3|LL_TIM_CHANNEL_CH3N))

/* Private typedef -----------------------------------------------------------*/

/* Private function prototypes -----------------------------------------------*/
static void ICS_TIMxInit(TIM_TypeDef *TIMx, PWMC_Handle_t *pHdl);
static void ICS_ADCxInit(ADC_TypeDef *ADCx);
static void ICS_HFCurrentsPolarization(PWMC_Handle_t *pHdl, ab_t *Iab);
static void ICS_RLTurnOnLowSides(PWMC_Handle_t *pHdl, uint32_t ticks);
static void ICS_RLGetPhaseCurrents(PWMC_Handle_t *pHdl, ab_t *pStator_Currents);
static void ICS_RLSwitchOnPWM(PWMC_Handle_t *pHdl);

/*
  * @brief  Initializes TIMx, ADC, GPIO and NVIC for current reading
  *         in ICS configuration using STM32G4XX.
  *
  * @param  pHandle: Handler of the current instance of the PWM component.
  */
__weak void ICS_Init(PWMC_ICS_Handle_t *pHandle)
{
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  ADC_TypeDef *ADCx_1 = pHandle->pParams_str->ADCx_1;
  ADC_TypeDef *ADCx_2 = pHandle->pParams_str->ADCx_2;

  /*Check that _Super is the first member of the structure PWMC_ICS_Handle_t */
  if ((uint32_t)pHandle == (uint32_t)&pHandle->_Super)
  {
    /* disable IT and flags in case of LL driver usage
     * workaround for unwanted interrupt enabling done by LL driver */
    LL_ADC_DisableIT_EOC(ADCx_1);
    LL_ADC_ClearFlag_EOC(ADCx_1);
    LL_ADC_DisableIT_JEOC(ADCx_1);
    LL_ADC_ClearFlag_JEOC(ADCx_1);
    LL_ADC_DisableIT_EOC(ADCx_2);
    LL_ADC_ClearFlag_EOC(ADCx_2);
    LL_ADC_DisableIT_JEOC(ADCx_2);
    LL_ADC_ClearFlag_JEOC(ADCx_2);

    if (TIMx == TIM1)
    {
      /* TIM1 Counter Clock stopped when the core is halted */
      LL_DBGMCU_APB2_GRP1_FreezePeriph(LL_DBGMCU_APB2_GRP1_TIM1_STOP);
    }
#if defined(TIM8)
    else
    {
      /* TIM8 Counter Clock stopped when the core is halted */
      LL_DBGMCU_APB2_GRP1_FreezePeriph(LL_DBGMCU_APB2_GRP1_TIM8_STOP);
    }
#endif

    if (0U == LL_ADC_IsEnabled(ADCx_1))
    {
      ICS_ADCxInit(ADCx_1);
      /* Only the Interrupt of the first ADC is enabled.
       * As Both ADCs are fired by HW at the same moment
       * It is safe to consider that both conversion are ready at the same time*/
    }
    else
    {
      /* Nothing to do ADCx_1 already configured */
    }
    if (0U == LL_ADC_IsEnabled(ADCx_2))
    {
      ICS_ADCxInit(ADCx_2);
    }
    else
    {
      /* Nothing to do ADCx_2 already configured */
    }
    ICS_TIMxInit(TIMx, &pHandle->_Super);
  }
}

/**
  * @brief Initializes @p ADCx peripheral for current sensing.
  * 
  * Specific to G4XX.
  * 
  */
static void ICS_ADCxInit(ADC_TypeDef *ADCx)
{
  /* - Exit from deep-power-down mode */
  LL_ADC_DisableDeepPowerDown(ADCx);

  if (LL_ADC_IsInternalRegulatorEnabled(ADCx) == 0u)
  {
    /* Enable ADC internal voltage regulator */
    LL_ADC_EnableInternalRegulator(ADCx);

    /* Wait for Regulator Startup time, once for both */
    /* Note: Variable divided by 2 to compensate partially              */
    /*       CPU processing cycles, scaling in us split to not          */
    /*       exceed 32 bits register capacity and handle low frequency. */
    volatile uint32_t wait_loop_index = ((LL_ADC_DELAY_INTERNAL_REGUL_STAB_US / 10UL) * (SystemCoreClock / (100000UL * 2UL)));
    while (wait_loop_index != 0UL)
    {
      wait_loop_index--;
    }
  }
  else
  {
    /* Nothing to do */
  }

  LL_ADC_StartCalibration(ADCx, LL_ADC_SINGLE_ENDED);
  while (LL_ADC_IsCalibrationOnGoing(ADCx) == 1u)
  {}
  /* ADC Enable (must be done after calibration) */
  /* ADC5-140924: Enabling the ADC by setting ADEN bit soon after polling ADCAL=0
  * following a calibration phase, could have no effect on ADC
  * within certain AHB/ADC clock ratio.
  */
  while (LL_ADC_IsActiveFlag_ADRDY(ADCx) == 0u)
  {
    LL_ADC_Enable(ADCx);
  }

  /* start injected conversion */
  LL_ADC_INJ_StartConversion(ADCx);

  /* TODO: check if not already done by MX */
  LL_ADC_INJ_SetQueueMode(ADCx, LL_ADC_INJ_QUEUE_2CONTEXTS_END_EMPTY);

  /* Dummy conversion (ES0431 doc chap. 2.5.8 ADC channel 0 converted instead of the required ADC channel) 
     Note: Sequence length forced to zero in order to prevent ADC OverRun occurrence */
  LL_ADC_REG_SetSequencerLength(ADCx, 0U);
  LL_ADC_REG_StartConversion(ADCx);
}

/**
  * @brief  Initializes @p TIMx peripheral with @p pHdl handler for PWM generation.
  * 
  * Specific to G4XX.
  * 
  */
static void ICS_TIMxInit(TIM_TypeDef *TIMx, PWMC_Handle_t *pHdl)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
  uint32_t Brk2Timeout = 1000;

  /* disable main TIM counter to ensure
   * a synchronous start by TIM2 trigger */
  LL_TIM_DisableCounter(TIMx);

  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_UPDATE);

  /* Enables the TIMx Preload on CC1 Register */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  /* Enables the TIMx Preload on CC2 Register */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  /* Enables the TIMx Preload on CC3 Register */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH3);
  /* Enables the TIMx Preload on CC4 Register */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH4);
  /* Prepare timer for synchronization */
  LL_TIM_GenerateEvent_UPDATE(TIMx);
  if (pHandle->pParams_str->FreqRatio == 2u)
  {
    if (pHandle->pParams_str->IsHigherFreqTim == HIGHER_FREQ)
    {
      if (pHandle->pParams_str->RepetitionCounter == 3u)
      {
        /* Set TIMx repetition counter to 1 */
        LL_TIM_SetRepetitionCounter(TIMx, 1);
        LL_TIM_GenerateEvent_UPDATE(TIMx);
        /* Repetition counter will be set to 3 at next Update */
        LL_TIM_SetRepetitionCounter(TIMx, 3);
      }
      else
      {
        /* Nothing to do */
      }
    }
    else
    {
      /* Nothing to do */
    }
    LL_TIM_SetCounter(TIMx, (uint32_t)(pHandle->Half_PWMPeriod) - 1u);
  }
  else /* FreqRatio equal to 1 or 3 */
  {
    if (pHandle->_Super.Motor == M1)
    {
      if (pHandle->pParams_str->RepetitionCounter == 1u)
      {
        LL_TIM_SetCounter(TIMx, (uint32_t)(pHandle->Half_PWMPeriod) - 1u);
      }
      else if (pHandle->pParams_str->RepetitionCounter == 3u)
      {
        /* Set TIMx repetition counter to 1 */
        LL_TIM_SetRepetitionCounter(TIMx, 1);
        LL_TIM_GenerateEvent_UPDATE(TIMx);
        /* Repetition counter will be set to 3 at next Update */
        LL_TIM_SetRepetitionCounter(TIMx, 3);
      }
      else
      {
        /* Nothing to do */
      }
    }
    else
    {
      /* Nothing to do */
    }
  }
  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_OC4REF);
  LL_TIM_ClearFlag_BRK(TIMx);
  while ((LL_TIM_IsActiveFlag_BRK2(TIMx) == 1u) && (Brk2Timeout != 0u))
  {
    LL_TIM_ClearFlag_BRK2(TIMx);
    Brk2Timeout--;
  }
  
  LL_TIM_EnableIT_BRK(TIMx);

  /* Enable PWM channel */
  LL_TIM_CC_EnableChannel(TIMx, TIMxCCER_MASK_CH123);

  LL_TIM_ClearFlag_UPDATE(TIMx);
  /* Enable Update IRQ */
  LL_TIM_EnableIT_UPDATE(TIMx);
}

/*
  * @brief  Stores in @p pHdl handler the calibrated @p offsets.
  * 
  */
__weak void ICS_SetOffsetCalib(PWMC_Handle_t *pHdl, PolarizationOffsets_t *offsets)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3

  pHandle->PhaseAOffset = offsets->phaseAOffset;
  pHandle->PhaseBOffset = offsets->phaseBOffset;
  pHdl->offsetCalibStatus = true;
}

/*
  * @brief Reads the calibrated @p offsets stored in @p pHdl.
  * 
  */
__weak void ICS_GetOffsetCalib(PWMC_Handle_t *pHdl, PolarizationOffsets_t *offsets)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3

  offsets->phaseAOffset = pHandle->PhaseAOffset;
  offsets->phaseBOffset = pHandle->PhaseBOffset;
}

/*
  * @brief  Stores into the @p pHdl handler the voltage present on Ia and
  *         Ib current feedback analog channels when no current is flowing into the
  *         motor.
  * 
  */
__weak void ICS_CurrentReadingPolarization(PWMC_Handle_t *pHdl)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  
  /* reset offset and counter */
  pHandle->PhaseAOffset = 0u;
  pHandle->PhaseBOffset = 0u;
  pHandle->PolarizationCounter = 0u;

  /* It forces inactive level on TIMx CHy and CHyN */
  LL_TIM_CC_DisableChannel(TIMx, TIMxCCER_MASK_CH123);

  /* Offset calibration for all phases */
  /* Change function to be executed in ADCx_ISR */
  __disable_irq();
  pHandle->_Super.pFctGetPhaseCurrents = &ICS_HFCurrentsPolarization;
  __enable_irq();

  ICS_SwitchOnPWM(&pHandle->_Super);

  /* Wait for NB_CONVERSIONS to be executed */
  waitForPolarizationEnd(TIMx,
                        &pHandle->_Super.SWerror,
                        pHandle->pParams_str->RepetitionCounter,
                        &pHandle->PolarizationCounter);

  ICS_SwitchOffPWM( &pHandle->_Super );

  pHandle->PhaseAOffset /= NB_CONVERSIONS;
  pHandle->PhaseBOffset /= NB_CONVERSIONS;
  if (0U == pHandle->_Super.SWerror)
  {
    pHandle->_Super.offsetCalibStatus = true;
  }
  else
  {
    /* nothing to do */
  }
  
  /* Change back function to be executed in ADCx_ISR */
  __disable_irq();
  pHandle->_Super.pFctGetPhaseCurrents = &ICS_GetPhaseCurrents;
  pHandle->_Super.pFctSetADCSampPointSectX = &ICS_WriteTIMRegisters;
  __enable_irq();

  /* Disable TIMx preload */
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH3);
  /* It over write TIMx CCRy wrongly written by FOC during calibration so as to
     force 50% duty cycle on the three inverer legs */
  LL_TIM_OC_SetCompareCH1 (TIMx, pHandle->Half_PWMPeriod >> 1u);
  LL_TIM_OC_SetCompareCH2 (TIMx, pHandle->Half_PWMPeriod >> 1u);
  LL_TIM_OC_SetCompareCH3 (TIMx, pHandle->Half_PWMPeriod >> 1u);
  /* Apply new CC values */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH3);

  /* It re-enable drive of TIMx CHy and CHyN by TIMx CHyRef*/
  LL_TIM_CC_EnableChannel(TIMx, TIMxCCER_MASK_CH123);

  pHandle->_Super.BrakeActionLock = false;

}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section(".ccmram")))
#endif
#endif

/*
  * @brief  Computes and stores in @p pHdl handler the latest converted motor phase currents in @p Iab ab_t format.
  *
  */
__weak void ICS_GetPhaseCurrents(PWMC_Handle_t *pHdl, ab_t *Iab)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  ADC_TypeDef *ADCx_1 = pHandle->pParams_str->ADCx_1;
  ADC_TypeDef *ADCx_2 = pHandle->pParams_str->ADCx_2;
  int32_t aux;
  uint16_t reg;

  /* disable ADC trigger source */
  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_RESET);

  /* Ia = (hPhaseAOffset)-(PHASE_A_ADC_CHANNEL value)  */
  reg = (uint16_t)(ADCx_1->JDR1);
  aux = (int32_t)(reg) - (int32_t)(pHandle->PhaseAOffset);

  /* Saturation of Ia */
  if (aux < -INT16_MAX)
  {
    Iab->a = -INT16_MAX;
  }
  else  if (aux > INT16_MAX)
  {
    Iab->a = INT16_MAX;
  }
  else
  {
    Iab->a = (int16_t)aux;
  }

  /* Ib = (hPhaseBOffset)-(PHASE_B_ADC_CHANNEL value) */
  reg = (uint16_t)(ADCx_2->JDR1);
  aux = (int32_t)(reg) - (int32_t)(pHandle->PhaseBOffset);

  /* Saturation of Ib */
  if (aux < -INT16_MAX)
  {
    Iab->b = -INT16_MAX;
  }
  else  if (aux > INT16_MAX)
  {
    Iab->b = INT16_MAX;
  }
  else
  {
    Iab->b = (int16_t)aux;
  }

  pHandle->_Super.Ia = Iab->a;
  pHandle->_Super.Ib = Iab->b;
  pHandle->_Super.Ic = -Iab->a - Iab->b;
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section(".ccmram")))
#endif
#endif

/*
  * @brief  Writes into peripheral registers the new duty cycle and sampling point.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  SamplingPoint: Point at which the ADC will be triggered, written in timer clock counts.
  * @retval Returns #MC_NO_ERROR if no error occurred or #MC_DURATION if the duty cycles were
  *         set too late for being taken into account in the next PWM cycle.
  */
__weak uint16_t ICS_WriteTIMRegisters(PWMC_Handle_t *pHdl)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  uint16_t Aux;


  LL_TIM_OC_SetCompareCH1(TIMx, (uint32_t) pHandle->_Super.CntPhA);
  LL_TIM_OC_SetCompareCH2(TIMx, (uint32_t) pHandle->_Super.CntPhB);
  LL_TIM_OC_SetCompareCH3(TIMx, (uint32_t) pHandle->_Super.CntPhC);

  /* Limit for update event */
  if (((TIMx->CR2) & TIM_CR2_MMS_Msk) != LL_TIM_TRGO_RESET)
  {
    Aux = MC_DURATION;
  }
  else
  {
    Aux = MC_NO_ERROR;
  }
  return (Aux);
}
/*
  * @brief  Implementation of PWMC_GetPhaseCurrents to be performed during polarization.
  * 
  * It sums up injected conversion data into PhaseAOffset and
  * PhaseBOffset to compute the offset introduced in the current feedback
  * network. It is required to properly configure ADC inputs before in order to enable
  * the offset computation.
  * 
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  Iab: Pointer to the structure that will receive motor current
  *         of phase A and B in ab_t format.
  */
static void ICS_HFCurrentsPolarization(PWMC_Handle_t *pHdl, ab_t *Iab)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  ADC_TypeDef *ADCx_1 = pHandle->pParams_str->ADCx_1;
  ADC_TypeDef *ADCx_2 = pHandle->pParams_str->ADCx_2;

  /* disable ADC trigger source */
  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_RESET);

  if (pHandle->PolarizationCounter < NB_CONVERSIONS)
  {
    pHandle-> PhaseAOffset += ADCx_1->JDR1;
    pHandle-> PhaseBOffset += ADCx_2->JDR1;
    pHandle->PolarizationCounter++;
  }
  else
  {
    /* Nothing to do */
  }

  /* during offset calibration no current is flowing in the phases */
  Iab->a = 0;
  Iab->b = 0;
}

/*
  * @brief  Turns on low sides switches.
  * 
  * This function is intended to be used for charging boot capacitors of driving section. It has to be
  * called on each motor start-up when using high voltage drivers.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  ticks: Timer ticks value to be applied
  *                Min value: 0 (low sides ON)
  *                Max value: PWM_PERIOD_CYCLES/2 (low sides OFF)
  */
__weak void ICS_TurnOnLowSides(PWMC_Handle_t *pHdl, uint32_t ticks)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

  pHandle->_Super.TurnOnLowSidesAction = true;

  /* Disable TIMx preload */
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH3);
  /*Turn on the three low side switches */
  LL_TIM_OC_SetCompareCH1(TIMx, ticks);
  LL_TIM_OC_SetCompareCH2(TIMx, ticks);
  LL_TIM_OC_SetCompareCH3(TIMx, ticks);
  /* Apply new CC values */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH3);

  /* Main PWM Output Enable */
  LL_TIM_EnableAllOutputs(TIMx);

  if ((pHandle->_Super.LowSideOutputs) == ES_GPIO)
  {
    LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
    LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
    LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
  }
  else
  {
    /* Nothing to do */
  }
}

/*
  * @brief  Enables PWM generation on the proper Timer peripheral acting on MOE bit.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
__weak void ICS_SwitchOnPWM(PWMC_Handle_t *pHdl)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

  pHandle->_Super.TurnOnLowSidesAction = false;

  /* Disable TIMx preload */
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH3);
  LL_TIM_OC_DisablePreload(TIMx, LL_TIM_CHANNEL_CH4);
  /* Set all duty to 50% */
  LL_TIM_OC_SetCompareCH1(TIMx, ((uint32_t) pHandle->Half_PWMPeriod / (uint32_t) 2));
  LL_TIM_OC_SetCompareCH2(TIMx, ((uint32_t) pHandle->Half_PWMPeriod / (uint32_t) 2));
  LL_TIM_OC_SetCompareCH3(TIMx, ((uint32_t) pHandle->Half_PWMPeriod / (uint32_t) 2));
  LL_TIM_OC_SetCompareCH4(TIMx, ((uint32_t) pHandle->Half_PWMPeriod - (uint32_t) 5));
  /* Apply new CC values */
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH1);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH2);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH3);
  LL_TIM_OC_EnablePreload(TIMx, LL_TIM_CHANNEL_CH4);

  /* Main PWM Output Enable */
  TIMx->BDTR |= LL_TIM_OSSI_ENABLE;
  LL_TIM_EnableAllOutputs(TIMx);

  if ((pHandle->_Super.LowSideOutputs) == ES_GPIO)
  {
    if ((TIMx->CCER & TIMxCCER_MASK_CH123) != 0u)
    {
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
    }
    else
    {
      /* It is executed during calibration phase the EN signal shall stay off */
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
    }
  }
  else
  {
    /* Nothing to do */
  }
  pHandle->_Super.PWMState = true;

}

/*
  * @brief  Disables PWM generation on the proper Timer peripheral acting on MOE bit.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
__weak void ICS_SwitchOffPWM(PWMC_Handle_t *pHdl)
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  pHandle->_Super.PWMState = false;
  pHandle->_Super.TurnOnLowSidesAction = false;

  /* Main PWM Output Disable */
  if (pHandle->_Super.BrakeActionLock == true)
  {
    /* Nothing to do */
  }
  else
  {
    TIMx->BDTR &= ~((uint32_t)(LL_TIM_OSSI_ENABLE));
    LL_TIM_DisableAllOutputs(TIMx);

    if ((pHandle->_Super.LowSideOutputs) == ES_GPIO)
    {
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
      LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
    }
    else
    {
      /* Nothing to do */
    }
  }
}




#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section(".ccmram")))
#endif
#endif
/*
  * @brief  Contains the TIMx Update event interrupt.
  *
  * @param  pHandle: Handler of the current instance of the PWM component.
  */
__weak void *ICS_TIMx_UP_IRQHandler(PWMC_ICS_Handle_t *pHandle)
{
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
  ADC_TypeDef *ADCx_1 = pHandle->pParams_str->ADCx_1;
  ADC_TypeDef *ADCx_2 = pHandle->pParams_str->ADCx_2;

    /* In dual drive use case with shared ADC: wait for both ADCs end of sequence conversion before writing a new one */
  if ((0x0u != ADCx_1->JSQR) || (0x0u != ADCx_2->JSQR))
  {  
    while (0x0u != ADCx_1->JSQR)
    {
      /* Nothing to do */
    }
  }
  ADCx_1->JSQR = (uint32_t) pHandle->pParams_str->ADCConfig1;
  ADCx_2->JSQR = (uint32_t) pHandle->pParams_str->ADCConfig2;

  /* enable ADC trigger source */
  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_OC4REF);

  return (&(pHandle->_Super.Motor));
}

/*
  * @brief  Sets the PWM mode for R/L detection.
  *
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
void ICS_RLDetectionModeEnable( PWMC_Handle_t * pHdl )
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

  if (false == pHandle->_Super.RLDetectionMode)
  {
    /* Channel1 configuration */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH1, LL_TIM_OCMODE_PWM1);
    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH1);
    LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH1N);
    LL_TIM_OC_SetCompareCH1(TIMx, 0U);

    /* Channel2 configuration */
    if (LS_PWM_TIMER == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH2, LL_TIM_OCMODE_ACTIVE);
      LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH2);
      LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH2N);
    }
    else if (ES_GPIO ==  pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH2, LL_TIM_OCMODE_INACTIVE);
      LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH2);
      LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH2N);
    }
    else
    {
      /* Nothing to do */
    }

    /* Channel3 configuration */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH3, LL_TIM_OCMODE_PWM2);
    LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH3);
    LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH3N);

    pHandle->PhaseAOffset = pHandle->PhaseBOffset; /* Use only the offset of phB */
  }
  else
  {
    /* Nothing to do */
  }

  __disable_irq();
  pHandle->_Super.pFctGetPhaseCurrents = &ICS_RLGetPhaseCurrents;
  pHandle->_Super.pFctTurnOnLowSides = &ICS_RLTurnOnLowSides;
  pHandle->_Super.pFctSwitchOnPwm = &ICS_RLSwitchOnPWM;
  pHandle->_Super.pFctSwitchOffPwm = &ICS_SwitchOffPWM;
  __enable_irq();

  pHandle->_Super.RLDetectionMode = true;
}

/*
  * @brief  Disables the PWM mode for R/L detection.
  *
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
void ICS_RLDetectionModeDisable( PWMC_Handle_t * pHdl )
{
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

  if (true ==  pHandle->_Super.RLDetectionMode)
  {
    /* Channel1 configuration */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH1, LL_TIM_OCMODE_PWM1);
    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH1);

    if (LS_PWM_TIMER == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH1N);
    }
    else if (ES_GPIO == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH1N);
    }
    else
    {
      /* Nothing to do */
    }

    LL_TIM_OC_SetCompareCH1(TIMx, ((uint32_t)pHandle->Half_PWMPeriod) >> 1);

    /* Channel2 configuration */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH2, LL_TIM_OCMODE_PWM1);
    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH2);

    if (LS_PWM_TIMER == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH2N);
    }
    else if (ES_GPIO == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH2N);
    }
    else
    {
      /* Nothing to do */
    }

    LL_TIM_OC_SetCompareCH2(TIMx, ((uint32_t)pHandle->Half_PWMPeriod) >> 1);

    /* Channel3 configuration */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH3, LL_TIM_OCMODE_PWM1);
    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH3);

    if (LS_PWM_TIMER == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH3N);
    }
    else if (ES_GPIO == pHandle->_Super.LowSideOutputs)
    {
      LL_TIM_CC_DisableChannel(TIMx, LL_TIM_CHANNEL_CH3N);
    }
    else
    {
      /* Nothing to do */
    }

    LL_TIM_OC_SetCompareCH3(TIMx, ((uint32_t)pHandle->Half_PWMPeriod) >> 1);

    __disable_irq();
    pHandle->_Super.pFctGetPhaseCurrents = &ICS_GetPhaseCurrents;
    pHandle->_Super.pFctTurnOnLowSides = &ICS_TurnOnLowSides;
    pHandle->_Super.pFctSwitchOnPwm = &ICS_SwitchOnPWM;
    pHandle->_Super.pFctSwitchOffPwm = &ICS_SwitchOffPWM;
    __enable_irq();

    pHandle->_Super.RLDetectionMode = false;
  }
  else
  {
    /* Nothing to do */
  }
}

/*
  * @brief  Sets the PWM dutycycle for R/L detection.
  *
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  hDuty: Duty cycle to apply, written in uint16_t.
  * @retval Returns #MC_NO_ERROR if no error occurred or #MC_DURATION if the duty cycles were
  *         set too late for being taken into account in the next PWM cycle.
  */
uint16_t ICS_RLDetectionModeSetDuty( PWMC_Handle_t * pHdl, uint16_t hDuty )
{
  uint16_t tempValue;
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  if (MC_NULL == pHdl)
  {
    tempValue = 0U;
  }
  else
  {
#endif
    PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
    TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
    uint32_t val;
    uint16_t hAux;

    pHandle->ADCRegularLocked = true;

    val = (((uint32_t)pHandle->Half_PWMPeriod) * ((uint32_t)hDuty)) >> 16;
    pHandle->_Super.CntPhA = (uint16_t)val;

    /* Set CC4 as PWM mode 2 (default) */
    LL_TIM_OC_SetMode(TIMx, LL_TIM_CHANNEL_CH4, LL_TIM_OCMODE_PWM2);

    LL_TIM_OC_SetCompareCH4(TIMx, (((uint32_t)pHandle->Half_PWMPeriod) - ((uint32_t)pHandle->_Super.Ton)));
    LL_TIM_OC_SetCompareCH3(TIMx, (uint32_t)pHandle->_Super.Toff);
    LL_TIM_OC_SetCompareCH1(TIMx, (uint32_t)pHandle->_Super.CntPhA);

    /* Enabling next Trigger */
    /* LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH4) */
    LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_OC4REF);
    /* Set the sector that correspond to Phase A and B sampling */
    pHdl->Sector = SECTOR_4;

    /* Limit for update event */
    /* Check the status flag. If an update event has occurred before to set new
    values of regs the FOC rate is too high */
    if (((TIMx->CR2) & TIM_CR2_MMS_Msk) != LL_TIM_TRGO_RESET)
    {
      hAux = MC_DURATION;
    }
    else
    {
      hAux = MC_NO_ERROR;
    }
    if (1U ==  pHandle->_Super.SWerror)
    {
      hAux = MC_DURATION;
      pHandle->_Super.SWerror = 0U;
    }
    else
    {
      /* Nothing to do */
    }
    tempValue = hAux;
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  }
#endif
  return (tempValue);
}

/*
  * @brief  Computes and stores into @p pHdl latest converted motor phase currents
  *         during RL detection phase.
  *
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  pStator_Currents: Pointer to the structure that will receive motor current
  *         of phase A and B in ab_t format.
  */
static void ICS_RLGetPhaseCurrents( PWMC_Handle_t * pHdl, ab_t * pStator_Currents )
{
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  if (MC_NULL == pStator_Currents)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
    TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
    ADC_TypeDef *ADCx_2 = pHandle->pParams_str->ADCx_2;
    int32_t wAux;

    /* Disable ADC trigger source */
    LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_RESET);

    wAux = ( (int32_t)( ADCx_2->JDR1 ) ) - ( (int32_t)( pHandle->PhaseBOffset ) );

    /* Check saturation */
    if (wAux > -INT16_MAX)
    {
      if (wAux < INT16_MAX)
      {
        /* Nothing to do */
      }
      else
      {
        wAux = INT16_MAX;
      }
    }
    else
    {
      wAux = -INT16_MAX;
    }

    pStator_Currents->a = (int16_t)wAux;
    pStator_Currents->b = (int16_t)wAux;
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  }
#endif
}

/*
  * @brief  Turns on low sides switches.
  * 
  * This function is intended to be used for charging boot capacitors
  * of driving section. It has to be called at each motor start-up when
  * using high voltage drivers.
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  * @param  ticks: Timer ticks value to be applied
  *                Min value: 0 (low sides ON)
  *                Max value: PWM_PERIOD_CYCLES/2 (low sides OFF)
  */
static void ICS_RLTurnOnLowSides( PWMC_Handle_t * pHdl, uint32_t ticks )
{
  (void)ticks; /* parameter not used */
  
  PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl;    //cstat !MISRAC2012-Rule-11.3
  TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

  /* We forbid ADC usage for regular conversion on Systick cut 2.2 patch*/
  pHandle->ADCRegularLocked = true;

  /* Turn on the phase A low side switch */
  LL_TIM_OC_SetCompareCH1(TIMx, 0u);
  
  /* Clear Update Flag */
  LL_TIM_ClearFlag_UPDATE(TIMx);
  
  /* Wait until next update */
  while (0U == LL_TIM_IsActiveFlag_UPDATE(TIMx))
  {
    /* Nothing to do */
  }

  /* Main PWM Output Enable */
  LL_TIM_EnableAllOutputs(TIMx);

  if ((pHandle->_Super.LowSideOutputs) == ES_GPIO)
  {
    LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
    LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
    LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
  }
  else
  {
    /* Nothing to do */
  }
}



/*
  * @brief  Enables PWM generation on the proper Timer peripheral.
  * 
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
static void ICS_RLSwitchOnPWM( PWMC_Handle_t * pHdl )
{
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  if (MC_NULL == pHdl)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
    TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;
 
    /* We forbid ADC usage for regular conversion on Systick cut 2.2 patch*/
    pHandle->ADCRegularLocked=true;
    
    /* Wait for a new PWM period */
    LL_TIM_ClearFlag_UPDATE(TIMx);
    while (0U == LL_TIM_IsActiveFlag_UPDATE(TIMx))
    {
      /* Nothing to do */
    }
    /* Clear Update Flag */
    LL_TIM_ClearFlag_UPDATE(TIMx);

    LL_TIM_OC_SetCompareCH1(TIMx, 1U);
    LL_TIM_OC_SetCompareCH4(TIMx, ((uint32_t )pHandle->Half_PWMPeriod) - 5U);

    while (0U == LL_TIM_IsActiveFlag_UPDATE(TIMx))
    {
      /* Nothing to do */
    }

    /* Enable TIMx update interrupt */
    LL_TIM_EnableIT_UPDATE(TIMx);

    /* Main PWM Output Enable */
    TIMx->BDTR |= LL_TIM_OSSI_ENABLE ;
    LL_TIM_EnableAllOutputs(TIMx);

    if (ES_GPIO ==  pHandle->_Super.LowSideOutputs)
    {
      if ((TIMx->CCER & TIMxCCER_MASK_CH123 ) != 0U)
      {
        LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
        LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
        LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
      }
      else
      {
        /* It is executed during calibration phase the EN signal shall stay off */
        LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
        LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
        LL_GPIO_ResetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
      }
    }
    else
    {
      /* Nothing to do */
    }

    /* Set the sector that correspond to Phase B and C sampling
     * B will be sampled by ADCx_1 */
    pHdl->Sector = SECTOR_4;

    LL_ADC_INJ_StartConversion(pHandle->pParams_str->ADCx_1);
    LL_ADC_INJ_StartConversion(pHandle->pParams_str->ADCx_2);
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  }
#endif
}


/*
  * @brief  Turns on low sides switches and start ADC triggering.
  * 
  * Specific for Motor Profiling using Isolated Current Sensors.
  *
  * @param  pHdl: Handler of the current instance of the PWM component.
  */
void ICS_RLTurnOnLowSidesAndStart( PWMC_Handle_t * pHdl )
{
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  if (MC_NULL == pHdl)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    PWMC_ICS_Handle_t *pHandle = (PWMC_ICS_Handle_t *)pHdl; //cstat !MISRAC2012-Rule-11.3
    TIM_TypeDef *TIMx = pHandle->pParams_str->TIMx;

    /* We forbid ADC usage for regular conversion on Systick cut 2.2 patch*/
    pHandle->ADCRegularLocked=true;

    LL_TIM_ClearFlag_UPDATE(TIMx);
    while (0U == LL_TIM_IsActiveFlag_UPDATE(TIMx))
    {
      /* Nothing to do */
    }
    /* Clear Update Flag */
    LL_TIM_ClearFlag_UPDATE(TIMx);

    LL_TIM_OC_SetCompareCH1(TIMx, 0x0U);
    LL_TIM_OC_SetCompareCH2(TIMx, 0x0U);
    LL_TIM_OC_SetCompareCH3(TIMx, 0x0U);
    LL_TIM_OC_SetCompareCH4(TIMx, ((uint32_t )pHandle->Half_PWMPeriod) - 5U);

    while (0U == LL_TIM_IsActiveFlag_UPDATE(TIMx))
    {
      /* Nothing to do */
    }

    /* Main PWM Output Enable */
    TIMx->BDTR |= LL_TIM_OSSI_ENABLE ;
    LL_TIM_EnableAllOutputs (TIMx);

    if (ES_GPIO == pHandle->_Super.LowSideOutputs )
    {
      /* It is executed during calibration phase the EN signal shall stay off */
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_u_port, pHandle->_Super.pwm_en_u_pin);
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_v_port, pHandle->_Super.pwm_en_v_pin);
      LL_GPIO_SetOutputPin(pHandle->_Super.pwm_en_w_port, pHandle->_Super.pwm_en_w_pin);
    }
    else
    {
      /* Nothing to do */
    }

    pHdl->Sector = SECTOR_4;

    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH4);

    LL_TIM_EnableIT_UPDATE(TIMx);
#ifdef NULL_PTR_CHECK_ICS_PWM_CURR_FDB
  }
#endif
}

/**
  * @}
  */

/**
  * @}
  */

/**
  * @}
  */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
