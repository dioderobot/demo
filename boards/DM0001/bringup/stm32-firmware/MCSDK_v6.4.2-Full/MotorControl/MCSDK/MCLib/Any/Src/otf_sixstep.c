/**
  ******************************************************************************
  * @file    otf_sixstep.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides the functions that implement the features
  *          of the OnTheFly Rev-Up Control component for Six-Step drives of the Motor 
  *          Control SDK.
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2026 STMicroelectronics International N.V.
  * All rights reserved.</center></h2>
  *
  * Redistribution and use in source and binary forms, with or without
  * modification, are permitted, provided that the following conditions are met:
  *
  * 1. Redistribution of source code must retain the above copyright notice,
  *    this list of conditions and the following disclaimer.
  * 2. Redistributions in binary form must reproduce the above copyright notice,
  *    this list of conditions and the following disclaimer in the documentation
  *    and/or other materials provided with the distribution.
  * 3. Neither the name of STMicroelectronics nor the names of other
  *    contributors to this software may be used to endorse or promote products
  *    derived from this software without specific written permission.
  * 4. This software, including modifications and/or derivative works of this
  *    software, must execute solely and exclusively on microcontroller or
  *    microprocessor devices manufactured by or for STMicroelectronics.
  * 5. Redistribution and use of this software other than as permitted under
  *    this license is void and will automatically terminate your rights under
  *    this license.
  *
  * THIS SOFTWARE IS PROVIDED BY STMICROELECTRONICS AND CONTRIBUTORS "AS IS"
  * AND ANY EXPRESS, IMPLIED OR STATUTORY WARRANTIES, INCLUDING, BUT NOT
  * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
  * PARTICULAR PURPOSE AND NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY
  * RIGHTS ARE DISCLAIMED TO THE FULLEST EXTENT PERMITTED BY LAW. IN NO EVENT
  * SHALL STMICROELECTRONICS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
  * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
  * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
  * OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
  * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
  * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
  * EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
  *
  ******************************************************************************
  * @ingroup OnTheFly6S
  */

/* Includes ------------------------------------------------------------------*/
#include "otf_sixstep.h"

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup SixStep
  *
  * @{
  */


/** @addtogroup SIXSTEP_MCLLAPI
  * @{
  */


/** @defgroup OnTheFly6S On The Fly RevUp Management
  * @brief OnTheFly Control component used to start motor driven with the Six-Step technique
  *
  * @{
  */


/* Private defines -----------------------------------------------------------*/

/* Private functions ----------------------------------------------------------*/
void OTF_6S_TurnOnLowSides(OTF_6S_Handle_t *pHandle);
void OTF_6S_Brake(OTF_6S_Handle_t *pHandle);

/*
  * @brief  Initialize internal 6-Step OnTheFly controller state.
  * @param  pHandle: Pointer on Handle structure of OnTheFly controller.
  */

__weak void OTF_6S_Init(OTF_6S_Handle_t *pHandle, BusVoltageSensor_Handle_t *BusVHandle)
{
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  { 
    uint16_t latest_busConv = VBS_GetAvBusVoltage_d(BusVHandle);
    uint16_t Threshold_Pwm = (uint16_t)(pHandle->pSensing_ThresholdPerc * latest_busConv/pHandle->pBADC->Pwm_H_L.Bus2ThresholdConvFactor)
                               + pHandle->pBADC->Pwm_H_L.ThresholdCorrectFactor;
    LL_TIM_DisableIT_CC1(ADC_TIMER_TRIGGER);
    pHandle->pBADC->pSensing_Threshold_HSMod = Threshold_Pwm;
    pHandle->pBADC->pSensing_Point = &(pHandle->pBADC->Pwm_H_L.SamplingPointOff);
    pHandle->pBADC->Last_TimerSpeed_Counter =  LL_TIM_GetCounter(ADC_TIMER_TRIGGER);    
    PWMC_SetADCTriggerChannel( pHandle->pPwmc, *(pHandle->pBADC->pSensing_Point));
    if (pHandle->pRevUp->hDirection > 0) 
    {
      pHandle->pPwmc->Step = STEP_2;
    }
    else
    {
      pHandle->pPwmc->Step = STEP_1;
    }      
    OTF_6S_TurnOnLowSides(pHandle);
    LL_TIM_SetTriggerOutput(TIM1, LL_TIM_TRGO_OC4REF);
    BADC_Start(pHandle->pBADC, pHandle->pPwmc->Step, pHandle->pPwmc->LSModArray); 
    pHandle->OTFongoing = true;
    pHandle->OTFabort = false;
    pHandle->BemfErrors = 0;
  }
}

/*
  * @brief  Initialize internal 6-Step RevUp controller state.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  hMotorDirection: Rotor rotation direction.
  *         This parameter must be -1 or +1.
  */

__weak void OTF_6S_Clear(OTF_6S_Handle_t *pHandle)
{
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
    BADC_Stop();
    BADC_Clear(pHandle->pBADC);
    PWMC_SwitchOffPWM(pHandle->pPwmc);
    OTF_6S_Brake(pHandle);
    pHandle->OTFongoing = false;
    pHandle->index = 0U;
  }
}

bool OTF_6S_Task(OTF_6S_Handle_t *pHandle)
{
  bool retVal = false;
  if (MC_NULL == pHandle)
  {
    retVal = false;
  }
  else
  {
    uint32_t TimerSpeed_Counter = LL_TIM_GetCounter(ADC_TIMER_TRIGGER);
    uint32_t CounterAutoreload = LL_TIM_GetAutoReload(ADC_TIMER_TRIGGER) + 1U;
    uint32_t tBemfPeriod;
    if (TimerSpeed_Counter < pHandle->pBADC->Last_TimerSpeed_Counter)
    {
      tBemfPeriod = (int32_t) ((CounterAutoreload - (pHandle->pBADC->Last_TimerSpeed_Counter - TimerSpeed_Counter))/2);
    }
    else
    {
      tBemfPeriod = (int32_t) ((TimerSpeed_Counter - pHandle->pBADC->Last_TimerSpeed_Counter)/2);
    }
    if ((tBemfPeriod * pHandle->speedTimerPsc) < (3 * pHandle->pPwmc->PWMperiod) )
    {
      pHandle->BemfErrors++;
    }
    else
    {
      pHandle->pBADC->StepTime_Last = tBemfPeriod;
      pHandle->pBADC->SpeedBufferDpp[pHandle->pBADC->SpeedFIFOIdx] = (int32_t) (tBemfPeriod * pHandle->pRevUp->hDirection);
      pHandle->pBADC->SpeedFIFOIdx ++;
    }
    pHandle->pBADC->Last_TimerSpeed_Counter =   TimerSpeed_Counter;
    BADC_Stop();
    if ( pHandle->pBADC->SpeedFIFOIdx > pHandle->maxConsecutiveBemfTransitions)
    {
      pHandle->AvrSpeed = ((int16_t)(((int32_t)pHandle->pBADC->_Super.speedConvFactor )/
                                     ((int32_t)pHandle->pBADC->SpeedBufferDpp[pHandle->pBADC->SpeedFIFOIdx-1]
                                      )));
      BADC_SetSpeedTimer(pHandle->pBADC, (uint32_t) (pHandle->pBADC->StepTime_Last/2));
      
      int32_t tSpeedBuffer = pHandle->pBADC->SpeedBufferDpp[pHandle->pBADC->SpeedFIFOIdx-1];
      pHandle->pBADC->StepTime_Up = pHandle->pBADC->StepTime_Last;
      pHandle->pBADC->StepTime_Down = pHandle->pBADC->StepTime_Last;
      for (pHandle->pBADC->SpeedFIFOIdx = 0; pHandle->pBADC->SpeedFIFOIdx < pHandle->pBADC->SpeedBufferSize; pHandle->pBADC->SpeedFIFOIdx++)
      {
        pHandle->pBADC->SpeedBufferDpp[pHandle->pBADC->SpeedFIFOIdx] = tSpeedBuffer;
      }
      pHandle->pBADC->SpeedFIFOIdx = 0;
      pHandle->pBADC->ElPeriodSum = (int32_t) (tSpeedBuffer * pHandle->pBADC->SpeedBufferSize);
      pHandle->pBADC->_Super.hAvrMecSpeedUnit = pHandle->AvrSpeed;
      pHandle->pBADC->BufferFilled = pHandle->pBADC->SpeedBufferSize;
      
      retVal = true;
      LL_TIM_ClearFlag_CC1(ADC_TIMER_TRIGGER);
      LL_TIM_EnableIT_CC1(ADC_TIMER_TRIGGER);
    }
    else
    {
      if (pHandle->BemfErrors < pHandle->maxBemfErrors)
      {
        switch (pHandle->pPwmc->Step)
        {
        case STEP_1:
          pHandle->pPwmc->Step = STEP_5;
          break;
        case STEP_2:
          pHandle->pPwmc->Step = STEP_4;
          break;
        case STEP_3:
          pHandle->pPwmc->Step = STEP_1;
          break;
        case STEP_4:
          pHandle->pPwmc->Step = STEP_6;
          break;
        case STEP_5:
          pHandle->pPwmc->Step = STEP_3;
          break;
        case STEP_6:
        default:
          pHandle->pPwmc->Step = STEP_2;
          break;
        }
        OTF_6S_TurnOnLowSides(pHandle);
        BADC_Start(pHandle->pBADC, pHandle->pPwmc->Step, pHandle->pPwmc->LSModArray); 
      }
      else
      {
        pHandle->OTFabort = true;
      }
    }
  }
  return retVal;
}

void OTF_6S_UpdateDutyConv( OTF_6S_Handle_t *pHandle, uint16_t duty)
{
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {  
    if ((0 != duty) && (0 != pHandle->pBADC->_Super.hAvrMecSpeedUnit))
    {
      uint32_t tspeed, wtemp;
      uint8_t i;
      if (pHandle->pBADC->_Super.hAvrMecSpeedUnit > 0)
      {
        tspeed = (uint32_t) pHandle->pBADC->_Super.hAvrMecSpeedUnit;
      }
      else 
      {
        tspeed = (uint32_t) (- pHandle->pBADC->_Super.hAvrMecSpeedUnit);
      }      
      /* Store latest value on buffer */
      pHandle->aBuffer[pHandle->index] = (uint32_t) (((uint32_t) (duty * 65536)) / tspeed);
      wtemp = 0u;
      for (i = 0U; i < (uint8_t)pHandle->LowPassFilterBW; i++)
      {
        wtemp += pHandle->aBuffer[i];
      }
      wtemp /= pHandle->LowPassFilterBW;
      /* Averaging done over the buffer stored values */
      pHandle->speedDutyConvFactor = wtemp;
      
      if ((uint16_t)pHandle->index < (pHandle->LowPassFilterBW - 1U))
      {
        pHandle->index++;
      }
      else
      {
        pHandle->index = 0U;
      }
    }
  }
}

void OTF_6S_TurnOnLowSides(OTF_6S_Handle_t *pHandle)
{
  TIM_TypeDef * TIMx = pHandle->pPwmc->pParams_str->TIMx;

  /*Turn on the three low side switches */
  LL_TIM_OC_SetCompareCH1(TIMx, pHandle->LSDetectCnt);
  LL_TIM_OC_SetCompareCH2(TIMx, pHandle->LSDetectCnt);
  LL_TIM_OC_SetCompareCH3(TIMx, pHandle->LSDetectCnt);
  LL_TIM_WriteReg(TIMx, CCMR1, pHandle->pPwmc->TimerCfg->CCMR1_BootCharge);
  LL_TIM_WriteReg(TIMx, CCMR2, pHandle->pPwmc->TimerCfg->CCMR2_BootCharge);
  LL_TIM_WriteReg(TIMx, CCER, pHandle->TimerCfg->CCER_cfg[pHandle->pPwmc->Step]);
  LL_TIM_GenerateEvent_UPDATE( TIMx );  
  LL_TIM_GenerateEvent_COM( TIMx );
  if (ES_GPIO == pHandle->pPwmc->LowSideOutputs)
  {
    switch (pHandle->pPwmc->Step)
    {
    case STEP_2:
    case STEP_5:
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_w_port, pHandle->pPwmc->pParams_str->pwm_en_w_pin );
      LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_u_port, pHandle->pPwmc->pParams_str->pwm_en_u_pin );
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_v_port, pHandle->pPwmc->pParams_str->pwm_en_v_pin );
      break;
    case STEP_1:
    case STEP_4:
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_w_port, pHandle->pPwmc->pParams_str->pwm_en_w_pin );
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_u_port, pHandle->pPwmc->pParams_str->pwm_en_u_pin );
      LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_v_port, pHandle->pPwmc->pParams_str->pwm_en_v_pin );
      break;
    case STEP_3:
    case STEP_6:
    default:
      LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_w_port, pHandle->pPwmc->pParams_str->pwm_en_w_pin );
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_u_port, pHandle->pPwmc->pParams_str->pwm_en_u_pin );
      LL_GPIO_ResetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_v_port, pHandle->pPwmc->pParams_str->pwm_en_v_pin );
      break;
    }
  }

  /* Main PWM Output Enable */
  LL_TIM_EnableAllOutputs(TIMx);
}

void OTF_6S_Brake(OTF_6S_Handle_t *pHandle)
{
  TIM_TypeDef * TIMx = pHandle->pPwmc->pParams_str->TIMx;

  /*Turn on the three low side switches */
  LL_TIM_OC_SetCompareCH1(TIMx, pHandle->LSBrakeCnt);
  LL_TIM_OC_SetCompareCH2(TIMx, pHandle->LSBrakeCnt);
  LL_TIM_OC_SetCompareCH3(TIMx, pHandle->LSBrakeCnt);
  LL_TIM_WriteReg(TIMx, CCMR1, pHandle->pPwmc->TimerCfg->CCMR1_BootCharge);
  LL_TIM_WriteReg(TIMx, CCMR2, pHandle->pPwmc->TimerCfg->CCMR2_BootCharge);
  LL_TIM_WriteReg(TIMx, CCER, pHandle->TimerCfg->CCER_Brake_cfg);

  LL_TIM_GenerateEvent_UPDATE( TIMx );  
  LL_TIM_GenerateEvent_COM( TIMx );

  if (ES_GPIO == pHandle->pPwmc->LowSideOutputs)
  {
    LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_w_port, pHandle->pPwmc->pParams_str->pwm_en_w_pin );
    LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_u_port, pHandle->pPwmc->pParams_str->pwm_en_u_pin );
    LL_GPIO_SetOutputPin( pHandle->pPwmc->pParams_str->pwm_en_v_port, pHandle->pPwmc->pParams_str->pwm_en_v_pin );
  }

  /* Main PWM Output Enable */
  LL_TIM_EnableAllOutputs(TIMx);
  LL_TIM_SetTriggerOutput(pHandle->pPwmc->pParams_str->TIMx, LL_TIM_TRGO_OC4REF);
}

void OTF_6S_SwitchOver(OTF_6S_Handle_t *pHandle)
{
  TIM_TypeDef * TIMx = pHandle->pPwmc->pParams_str->TIMx;
  
  pHandle->OTFongoing = false;
  /* Select the Capture Compare preload feature */
  LL_TIM_CC_EnablePreload( TIMx );
  
  LL_TIM_GenerateEvent_COM( TIMx );
  /* Main PWM Output Enable */
  TIMx->BDTR |= LL_TIM_OSSI_ENABLE;
  LL_TIM_EnableAllOutputs(TIMx);
  
  LL_TIM_SetTriggerOutput(TIMx, LL_TIM_TRGO_OC4REF);
}

uint16_t OTF_6S_CalcSpeedReference(OTF_6S_Handle_t *pHandle)
{
  uint16_t retval;
  uint16_t tSpeed;
  if (pHandle->AvrSpeed >=0)  
  {
    tSpeed = pHandle->AvrSpeed;
  }
  else 
  {
    tSpeed = - pHandle->AvrSpeed;
  }
  retval = (uint16_t) (pHandle->speedDutyConvFactor * tSpeed / 65536);
  return retval;
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
