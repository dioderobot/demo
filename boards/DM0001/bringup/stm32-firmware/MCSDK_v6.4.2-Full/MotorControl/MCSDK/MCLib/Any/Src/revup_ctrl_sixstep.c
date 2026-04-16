/**
  ******************************************************************************
  * @file    revup_ctrl_sixstep.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides the functions that implement the features
  *          of the Rev-Up Control component for Six-Step drives of the Motor 
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
  * @ingroup RevUpCtrl6S
  */

/* Includes ------------------------------------------------------------------*/
#include "revup_ctrl_sixstep.h"

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


/** @defgroup RevUpCtrl6S RevUp management
  * @brief Rev-Up Control component used to start motor driven with the Six-Step technique
  *
  * @{
  */


/* Private defines -----------------------------------------------------------*/

/* Private functions ----------------------------------------------------------*/

/*
  * @brief  Initialize internal 6-Step RevUp controller state.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  hMotorDirection: Rotor rotation direction.
  *         This parameter must be -1 or +1.
  */

__weak void RUC_6S_Clear(RevUpCtrl_6S_Handle_t *pHandle, int16_t hMotorDirection)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    RevUpCtrl_6S_PhaseParams_t *pRUCPhaseParams = &pHandle->ParamsData[0];
    uint8_t bPhase = 0U;
    while ((pRUCPhaseParams != MC_NULL) && (bPhase < RUC_MAX_PHASE_NUMBER))
    {
      /* Dump HF data for now HF data are forced to 16 bits. */
      pRUCPhaseParams = (RevUpCtrl_6S_PhaseParams_t *)pRUCPhaseParams->pNext; //cstat !MISRAC2012-Rule-11.5
      bPhase++;
    }

    if (0U == bPhase)
    {
      /* Nothing to do error. */
    }
    else
    {
      pHandle->ParamsData[bPhase - 1u].pNext = MC_NULL;

      pHandle->bPhaseNbr = bPhase;

    }
    
    pHandle->pCurrentPhaseParams = pHandle->ParamsData;
    SpeednDutyCtrl_Handle_t *pSDC = pHandle->pSDC;
        
    pHandle->hDirection = (int8_t)hMotorDirection;

    /* Initializes the rev up stages counter. */
    pHandle->bStageCnt = 0U;

    pHandle->bPhaseNbr = 1U;

    pHandle->ElAccSpeedRefUnit = 0U;
    pHandle->ElSpeedTimerDpp = pHandle->_Super.speedConvFactor ;
    pHandle->_Super.hAvrMecSpeedUnit = 0;
    /* Sets the STC in torque mode. */
    SDC_SetControlMode(pSDC, MCM_DUTY_MODE);
    pHandle->hPhaseRemainingTicks = 0U;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/**
  * @brief  Update rev-up duty cycle relative to actual Vbus value to be applied.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  BusVHandle: pointer to the bus voltage sensor.
  */
__weak void RUC_6S_UpdatePulse(RevUpCtrl_6S_Handle_t *pHandle, BusVoltageSensor_Handle_t *BusVHandle)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    uint16_t tPulseUpdateFactor = 10U * NOMINAL_BUS_VOLTAGE_V
                                  / VBS_GetAvBusVoltage_V(BusVHandle);
    pHandle->PulseUpdateFactor = tPulseUpdateFactor;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/*
  * @brief  6-Step Main revup controller procedure executing overall programmed phases.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Boolean set to true when entire revup phases have been completed.
  */
__weak bool RUC_6S_Exec(RevUpCtrl_6S_Handle_t *pHandle)
{
  bool retVal = true;
  uint32_t FinalMecSpeedRefUnit;

#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    retVal = false;
  }
  else
  {
#endif
    if (pHandle->hPhaseRemainingTicks > 1U)
    {
      
      /* Decrease the hPhaseRemainingTicks. */
      pHandle->hPhaseRemainingTicks--;
      pHandle->CurrentSpeedRefUnit += pHandle->ElAccSpeedRefUnit;
     
    } /* hPhaseRemainingTicks > 0 */
    else if (1U == pHandle->hPhaseRemainingTicks)
    {

      pHandle->hPhaseRemainingTicks--;
      pHandle->CurrentSpeedRefUnit = (uint32_t)(pHandle->pCurrentPhaseParams->hFinalMecSpeedUnit) * 65536U;

    } /* hPhaseRemainingTicks = 1 */
    else
    {
      if (pHandle->bStageCnt != 0U)
      {
        /* Set the next phases parameter pointer. */
        pHandle->pCurrentPhaseParams = pHandle->pCurrentPhaseParams->pNext; //cstat !MISRAC2012-Rule-11.5
      }
    else
      {
        /* Nothing to do */
      }
      
      if (pHandle->pCurrentPhaseParams != MC_NULL)
      {
        uint16_t hPulse = pHandle->pCurrentPhaseParams->hFinalPulse * pHandle->PulseUpdateFactor / 10U;
        (void)SDC_ExecRamp(pHandle->pSDC, (int16_t)hPulse,
                          (uint32_t)(pHandle->pCurrentPhaseParams->hDurationms));
         
         
         FinalMecSpeedRefUnit = (uint32_t)(pHandle->pCurrentPhaseParams->hFinalMecSpeedUnit) * 65536U;
         if (0U == pHandle->pCurrentPhaseParams->hDurationms)
         {
           
           pHandle->CurrentSpeedRefUnit = FinalMecSpeedRefUnit;
           pHandle->hPhaseRemainingTicks = 0U;
           
         }
         else
         {
           /* Compute hPhaseRemainingTicks. */
           pHandle->hPhaseRemainingTicks = (uint16_t)((((uint32_t)pHandle->pCurrentPhaseParams->hDurationms)
                                                       * ((uint32_t)pHandle->hRUCFrequencyHz)) / 1000U);
           pHandle->hPhaseRemainingTicks++;
           
           if (0 == pHandle->pCurrentPhaseParams->hFinalMecSpeedUnit)
           {
             pHandle->CurrentSpeedRefUnit = 0;
             pHandle->ElAccSpeedRefUnit = 0;
           }
           else
           {
             pHandle->ElAccSpeedRefUnit = FinalMecSpeedRefUnit - pHandle->CurrentSpeedRefUnit;
             pHandle->ElAccSpeedRefUnit /= ((uint32_t)pHandle->hPhaseRemainingTicks);
           }           
         }
         /* Increases the rev up stages counter. */
         pHandle->bStageCnt++;
      }  /* hPhaseRemainingTicks = 0 */
      else
      {
        retVal = false;
      }
    }  /* hPhaseRemainingTicks = 0 */
    
    if (0U == pHandle->CurrentSpeedRefUnit) 
    {
      pHandle->ElSpeedTimerDpp = pHandle->_Super.speedConvFactor ;
    }
    else 
    {
      if (pHandle->CurrentSpeedRefUnit <= 0xFFFFU)
      {
        pHandle->ElSpeedTimerDpp = pHandle->_Super.speedConvFactor ;
        pHandle->_Super.hAvrMecSpeedUnit = (int16_t)((int32_t)pHandle->_Super.speedConvFactor /
                                          ((int32_t)pHandle->ElSpeedTimerDpp * pHandle->hDirection));
      }
      else
      {
        pHandle->ElSpeedTimerDpp = (uint32_t)(pHandle->_Super.speedConvFactor /
                                  ((uint32_t)(pHandle->CurrentSpeedRefUnit >> 16)));
        pHandle->_Super.hAvrMecSpeedUnit = (int16_t)((int32_t)pHandle->_Super.speedConvFactor /
                                          ((int32_t)pHandle->ElSpeedTimerDpp * pHandle->hDirection));
      }
    }
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
  return (retVal);
}

/*
  * @brief  It is used to check if this stage is used for align motor.
  * @param  this related object of class CRUC.
  * @retval Returns 1 if the alignment is correct otherwise it returns 0.
  */
bool RUC_6S_IsAlignStageNow(const RevUpCtrl_6S_Handle_t *pHandle)
{
  bool align_flag = false;

#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    if (0 == pHandle->pCurrentPhaseParams->hFinalMecSpeedUnit)
    {
      align_flag = true;
    }
	else
    {
      /* Nothing to do */
    }
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
  return (align_flag);
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__( ( section ( ".ccmram" ) ) )
#endif
#endif

/*
  * @brief  Check that minimum rotor speed has been reached.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Boolean set to true when speed has been reached.
  */
__weak bool RUC_6S_ObserverSpeedReached(const RevUpCtrl_6S_Handle_t * pHandle)
{
  bool retVal = false;
  
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    uint32_t hStartUpElSpeed = (uint32_t)(pHandle->CurrentSpeedRefUnit >> 16);
    if (hStartUpElSpeed >= pHandle->hMinStartUpValidSpeed)
    {
      retVal = true;
    }
    else
    {
      /* Nothing to do. */
    }
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
  return (retVal);
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
