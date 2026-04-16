/**
  ******************************************************************************
  * @file    IPD_sixstep.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides the functions that implement the features
  *          of the Initial Position Detection Control component for Six-Step drives
  *          of the Motor Control SDK.
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
  * @ingroup InitialPositionDetect6S
  */

/* Includes ------------------------------------------------------------------*/
#include "ipd_sixstep.h"

#include "mc_type.h"
#include "main.h"
#include "mc_parameters.h"
#include "pwmc_sixstep.h"
#include "mc_tasks.h"

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


/** @defgroup InitialPositionDetect6S Initial Position Detection
  * @brief Initial Position Detection startup component used to start motor driven with the Six-Step technique
  *
  * @{
  */

/* Private defines -----------------------------------------------------------*/
#define S32_MIN                                                            ((int32_t)2147483648uL) /* 0x80000000 */
#define BEMF_SAMPLING_DELAY_TNOISE_TIME_AFTER_SWITCH_MIN_PWM_OFF_TIME      170U /* 170 = 1uS define delay after commutation for correct BEMF sampling. */

/* Private functions ----------------------------------------------------------*/
static int16_t IPD_6S_ZeroSpeedRotorAngleCheck(IPD_6S_Handle_t *pIPDHandle, PWMC_Handle_t *pHandle);

/**
  * @brief  Clear IPD parameters for next IPD start
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
__weak void IPD_6S_Clear(IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    pHandle->NumZeroSpeedSamples = 0U;
    pHandle->ElecAngle = -1;
    pHandle->IPDRunning = false;
    pHandle->IPDBEMFMeasured = true;
    pHandle->NumZeroSpeedStepValid = 0U;
    pHandle->PreviousStep = 0U;
    for (uint32_t i = 0U; i <= 5U ; i++)
    {
      pHandle->ADC_JDR_Currents[i] = 0;
    }

    for (uint32_t j = 0U; j <= 11U ; j++)
    {
      pHandle->Flux_6_Steps[j] = 0;
    }
#ifdef NULL_PTR_CHECK_IPD_6STEP
  }
#endif
}

/**
  * @brief  Execute the IPD startup process.
  * @param  PwmcHandle: Pointer on Handle structure of PWMc controller.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @return pHandle->IPDRunning: status of the IPD Algorithm
  */
bool IPD_6S_Task(IPD_6S_Handle_t *pHandle, PWMC_Handle_t *PwmcHandle)
{
  bool returnValue;
#ifdef NULL_PTR_CHECK_IPD_6S
  if ((MC_NULL == pHandle) || (MC_NULL == PwmcHandle))
  {
    returnValue = false;
  }
  else
  {
#endif

    pHandle->IPDRunning = true;      /* IPD algorithm is running. */

    if ((STEP_2 == PwmcHandle->Step) || (STEP_4 == PwmcHandle->Step) || (STEP_6 == PwmcHandle->Step))
    {
    /* Check if enough ZeroSpeed sample detected. */
      if (pHandle->NumZeroSpeedSamples >= pHandle->ValideBemfDetectedThreshold)
      {
        pHandle->NumZeroSpeedStepValid++;          /* New Step detected. */
        pHandle->NumZeroSpeedSamples = 0U;         /* Restart a neE IPD sequence for next step. */
        pHandle->PreviousStep = PwmcHandle->Step;  /* Save the current step number. */

        /* Check in number of IPD iteration is ended. */
        if (pHandle->NumZeroSpeedStepValid >= pHandle->NumZeroSpeedStepValidThreshold)
        {
          /* Stop the BEMF measure with Motor running configuration. */
          BADC_Stop();
          BADC_Clear(&Bemf_ADC_M1);
          IPD_6step_BemfStart();
          pHandle->IPDRunning = false;
        }
        else
        {
          /* Continue the IPD algorithm. */
        }
      }
      else
      {
        /* Continue the IPD algorithm. */
      }
    }
    else
    {
      /* Continue the IPD algorithm. */
    }

    /* Check if enough ZeroSpeed sample detected, or not enaugh compared to number of tries. */
    if (pHandle->IPDRunning && pHandle->IPDBEMFMeasured)
    {
      pHandle->IPDPulseRunState = true;             /* IPD onePulse Mode running. */
      Bemf_ADC_M1.SpeedTimerState = LFTIM_IDLE; /* Come back to Idle Mode for IPD. */
      pHandle->IPDBEMFMeasured = false;         /* Clear the BEMF measured flag. */

      if (!pHandle->IPDDebug)                   /* Debug mode to start with no IPD algo. */
      {
        /* Stop the BEMF measure with Motor running configuration. */
        BADC_Stop();
        BADC_Clear(&Bemf_ADC_M1);

        /* Rotor Position Detection Phase execution */
        pHandle->ElecAngle = IPD_6S_ZeroSpeedRotorAngleCheck(pHandle, PwmcHandle);

        /* step number vs elec rotor angle
           STEP_1    0 --          60 deg
           STEP_2    1 --         120 deg
           STEP_3    2 --         180 deg
           STEP_4    3 --         240 deg
           STEP_5    4 --         300 deg
           STEP_6    5 --           0 deg */

        if (pHandle->ElecAngle <= -150)     /* -150, -180 */
        {
          PwmcHandle->Step = STEP_6;
        }
        else if (pHandle->ElecAngle <= -90) /* -90, -120 */
        {
          PwmcHandle->Step = STEP_1;
        }
        else if (pHandle->ElecAngle <= -30) /* -30, -60 */
        {
          PwmcHandle->Step = STEP_2;
        }
        else if (pHandle->ElecAngle <= 30)  /* 0, 30 */
        {
          PwmcHandle->Step = STEP_3;
        }
        else if (pHandle->ElecAngle <= 90) /* 60, 90 */
        {
          PwmcHandle->Step = STEP_4;
        }
        else if (pHandle->ElecAngle <= 150) /* 120, 150 */
        {
          PwmcHandle->Step = STEP_5;
        }
        else
        {
          PwmcHandle->Step = STEP_6;
        }

        /* Update step detected. */
        if (pHandle->previous_angle != PwmcHandle->Step)
        {
          pHandle->previous_angle = PwmcHandle->Step;
        }
        else
        {
          /* Nothing to do. */
        }

        /* Stop ADC IPD conversion. */
        BADC_Stop();
        BADC_Clear(&Bemf_ADC_M1);
      }
      else
      {
        /* Debug Mode no IPD algorithm. */
      }

      /* Start BEMF ADC conversion. */
      pHandle->IPDPulseRunState = false;                  /* End of IPD process. */
      IPD_6step_BemfStart();

      if (pHandle->IPDDebug)
      {
        pHandle->IPDRunning = false;
      }
      else
      {
        /* Nothing to do. */
      }
    }
    else
    {
      /* Continue BEMF read for next check, or end process.*/
    }
    returnValue = pHandle->IPDRunning;
#ifdef NULL_PTR_CHECK_IPD_6S
  }
#endif
  return(returnValue);
}

/**
  * @brief  Execute the IPD startup PWM pulse generation.
  * @param  pHandle: Pointer on Handle structure of PWMc controller.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
int16_t IPD_6S_ZeroSpeedRotorAngleCheck(IPD_6S_Handle_t *pIPDHandle, PWMC_Handle_t *pHandle)
{
  PWMC_IPD_PWMTimerSave(pHandle);

  /* Initialise the PWM Timer for the Pulse mode sequence. */
  PWMC_IPD_OnePulsePwmInit(pHandle);

  /* Set compare value for output channel 4 (TIMx_CCR4) for ADC current sampling in min TOff time */
  PWMC_SetIPDADCTriggerChannel(pHandle, IPD_PWM_DUTY_CYCLE - BEMF_SAMPLING_DELAY_TNOISE_TIME_AFTER_SWITCH_MIN_PWM_OFF_TIME - IPD_ADC_SAMPLING_TSAMPLING_TIM_TICKS);

  /* Run the one Pulse mode timer sequence. */
  PWMC_IPD_OnePulsePwmRun(&pIPDHandle->ADC_JDR_Currents[0], pHandle);

  /* Clear any pending IT flags. */
  BADC_IPD_Clear();

  /* Clear Pulse Mode and enable timer. */
  PWMC_IPD_CleartOnePulseMode(pHandle->pParams_str->TIMx);

  PWMC_IPD_PWMTimerRestore(pHandle);

  /* Compute the ADC level of each angle and select the position. */
  /* Conpute the average ADC value of the 6 defined angles. */
  int32_t average = 0;

  for (uint8_t i = 0U; i <= 5U; ++i)
  {
    average += pIPDHandle->ADC_JDR_Currents[i];
  }
  average = average / 6;

  /* Cnmpute the flux of all the defined step angles. */
  pIPDHandle->Flux_6_Steps[0]  = pIPDHandle->ADC_JDR_Currents[0] - average;  /*   0 */
  pIPDHandle->Flux_6_Steps[2]  = pIPDHandle->ADC_JDR_Currents[2] - average;  /*  60 */
  pIPDHandle->Flux_6_Steps[4]  = pIPDHandle->ADC_JDR_Currents[4] - average;  /* 120 */
  pIPDHandle->Flux_6_Steps[6]  = pIPDHandle->ADC_JDR_Currents[1] - average;  /* 180 */
  pIPDHandle->Flux_6_Steps[8]  = pIPDHandle->ADC_JDR_Currents[3] - average;  /* 240 */
  pIPDHandle->Flux_6_Steps[10] = pIPDHandle->ADC_JDR_Currents[5] - average;  /* 300 */

  /* Compute flux of intermediary step angles. */
  pIPDHandle->Flux_6_Steps[1]  = ((pIPDHandle->Flux_6_Steps[0]  + pIPDHandle->Flux_6_Steps[2])  / 2);  /* (  0 +  60) / 2 =  30 */
  pIPDHandle->Flux_6_Steps[3]  = ((pIPDHandle->Flux_6_Steps[2]  + pIPDHandle->Flux_6_Steps[4])  / 2);  /* ( 60 + 120) / 2 =  90 */
  pIPDHandle->Flux_6_Steps[5]  = ((pIPDHandle->Flux_6_Steps[4]  + pIPDHandle->Flux_6_Steps[6])  / 2);  /* (120 + 280) / 2 = 150 */
  pIPDHandle->Flux_6_Steps[7]  = ((pIPDHandle->Flux_6_Steps[6]  + pIPDHandle->Flux_6_Steps[8])  / 2);  /* (180 + 240) / 2 = 210 */
  pIPDHandle->Flux_6_Steps[9]  = ((pIPDHandle->Flux_6_Steps[8]  + pIPDHandle->Flux_6_Steps[10]) / 2);  /* (240 + 300) / 2 = 270 */
  pIPDHandle->Flux_6_Steps[11] = ((pIPDHandle->Flux_6_Steps[10] + pIPDHandle->Flux_6_Steps[0])  / 2);  /* (360 +  0)  / 2 = 330 */


  int32_t max = S32_MIN;
  int32_t sum;
  int16_t angle = 0;

  /*   0 +  30 +  60 -> angle =  30
   *  30 +  60 +  90 -> angle =  60
   *  60 +  90 + 120 -> angle =  90
   *  90 + 120 + 150 -> angle = 120
   * 120 + 150 + 180 -> angle = 150
   * 150 + 180 + 210 -> angle = 180
   * 180 + 210 + 240 -> angle = 210
   * 210 + 240 + 270 -> angle = 240
   * 240 + 270 + 300 -> angle = 270
   * 270 + 300 + 330 -> angle = 300
   */
  for (uint8_t i = 0U; i <= 9U; ++i)
  {
    sum = pIPDHandle->Flux_6_Steps[i] + pIPDHandle->Flux_6_Steps[i + 1U] + pIPDHandle->Flux_6_Steps[i + 2U];
    if (max < sum)
    {
      max = sum;
      angle = 30 * ((int16_t)i + 1);
    }
    else
    {
        /* Nothing to do. */
    }
  }

  /* 300 + 330 + 0 -> angle = 330 */
  sum = pIPDHandle->Flux_6_Steps[10] + pIPDHandle->Flux_6_Steps[11] + pIPDHandle->Flux_6_Steps[0];
  if (max < sum)
  {
    max = sum;
    angle = 330;
  }
  else
  {
    /* Nothing to do. */
  }

  /* 330 + 0 + 60 -> angle = 0 */
  sum = pIPDHandle->Flux_6_Steps[11] + pIPDHandle->Flux_6_Steps[0] + pIPDHandle->Flux_6_Steps[1];
  if (max < sum)
  {
    angle = 0;
  }
  else
  {
    /* Nothing to do. */
  }

  /* Elec_Angle is the electric angle deduced from position of magnet with 30 degrees resolution.
   *  30 - 180 = -150
   *  60 - 180 = -120
   *  90 - 180 =  -90
   * 120 - 180 =  -60
   * 150 - 180 =  -30
   * 180 - 180 =    0
   * 210 - 180 =   30
   * 240 - 180 =   60
   * 270 - 180 =   90
   * 300 - 180 =  120
   * 330 - 180 =  150
   * 0   - 180 = -180
   */
  return (angle - 180);
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
