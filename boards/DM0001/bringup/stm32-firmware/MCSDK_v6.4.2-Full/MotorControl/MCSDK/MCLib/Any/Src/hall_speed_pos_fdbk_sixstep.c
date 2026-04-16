
/**
  ******************************************************************************
  * @file    hall_speed_pos_fdbk_sixstep.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides firmware functions that implement the features of
  *          the Hall Speed & Position Feedback component of the Motor Control SDK.
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
  * @ingroup hall_speed_pos_fdbk_sixstep
  */

/* Includes ------------------------------------------------------------------*/
#include "hall_speed_pos_fdbk_sixstep.h"
#include "mc_type.h"

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup SixStep
  * @{
  */


/** @addtogroup SIXSTEP_MCLLAPI
  * @{
  */

/**
  * @defgroup hall_speed_pos_fdbk_sixstep Hall Speed & Position Feedback
  *
  * @brief Hall Sensor based Speed & Position Feedback implementation
  *
  * This component requires a motor equipped with Hall effect sensors. 
  * It uses the sensors to provide a measure of the speed and the position of the rotor of the motor.
  *
  * More detail in [Hall sensor Feedback processing](rotor_speed_pos_feedback_hall.md).
  *
  * @{
  */

/* Private defines -----------------------------------------------------------*/

/* Lower threshold to reques a decrease of clock prescaler */
#define LOW_RES_THRESHOLD   ((uint16_t)0x5500U)

#define HALL_COUNTER_RESET  ((uint16_t)0U)

#define STATE_0 (uint8_t)0
#define STATE_1 (uint8_t)1
#define STATE_2 (uint8_t)2
#define STATE_3 (uint8_t)3
#define STATE_4 (uint8_t)4
#define STATE_5 (uint8_t)5
#define STATE_6 (uint8_t)6
#define STATE_7 (uint8_t)7

#define NEGATIVE              (int8_t)-1
#define POSITIVE              (int8_t)1
#define MIN_MEAS_FACT_SPEED   4U
#define MAX_MEAS_FACT_SPEED   2U

/* With digit-per-PWM unit (here 2*PI rad = 0xFFFF): */
#define HALL_MAX_PSEUDO_SPEED ((int16_t)0x7FFF)

static void HALL_ReadState(HALL_6S_Handle_t *pHandle);
static void HALL_State2Step(HALL_6S_Handle_t *pHandle);

/**
  * @brief  It initializes the hardware peripherals (TIMx, GPIO and NVIC)
            required for the speed position sensor management using HALL
            sensors.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component
  */
__weak void HALL_Init(HALL_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    TIM_TypeDef *TIMx = pHandle->TIMx;

    uint16_t hMinReliableElSpeedUnit = pHandle->_Super.hMinReliableMecSpeedUnit * pHandle->_Super.bElToMecRatio;
    uint16_t hMaxReliableElSpeedUnit = pHandle->_Super.hMaxReliableMecSpeedUnit * pHandle->_Super.bElToMecRatio;

    /* Adjustment factor: minimum measurable speed is x time less than the minimum
    reliable speed. */
    hMinReliableElSpeedUnit /= MIN_MEAS_FACT_SPEED;

    /* Adjustment factor: maximum measurable speed is x time greater than the
    maximum reliable speed. */
    hMaxReliableElSpeedUnit *= MAX_MEAS_FACT_SPEED;

    pHandle->OvfFreq = (uint16_t)(pHandle->TIMClockFreq / 65536U);

    /* SW Init. */
    if (0U == hMinReliableElSpeedUnit)
    {
      /* Set fixed to 150 ms. */
      pHandle->HallTimeout = 150U;
    }
    else
    {
      /* Set accordingly the min reliable speed */
      /* 1000 comes from mS
      * 6 comes from the fact that sensors are toggling each 60 deg = 360/6 deg. */
      pHandle->HallTimeout = (1000U * (uint16_t)SPEED_UNIT) / (6U * hMinReliableElSpeedUnit);
    }

    /* Compute the prescaler to the closet value of the TimeOut (in mS). */
    pHandle->HALLMaxRatio = (pHandle->HallTimeout * pHandle->OvfFreq) / 1000U ;

    /* Align MaxPeriod to a multiple of Overflow. */
    pHandle->MaxPeriod = pHandle->HALLMaxRatio * 65536UL;

    if (0U == hMaxReliableElSpeedUnit)
    {
      pHandle->MinPeriod = ((uint32_t)SPEED_UNIT * (pHandle->TIMClockFreq / 6UL));
    }
    else
    {
      pHandle->MinPeriod = (((uint32_t)SPEED_UNIT * (pHandle->TIMClockFreq / 6UL)) / hMaxReliableElSpeedUnit);
    }

    /* Reset speed reliability. */
    pHandle->SensorIsReliable = true;

    /* Set IC filter for Channel 1 (ICF1). */
    LL_TIM_IC_SetFilter(TIMx, LL_TIM_CHANNEL_CH1, (uint32_t)(pHandle->ICx_Filter) << 20U);

    /* Force the TIMx prescaler with immediate access (gen update event). */
    LL_TIM_SetPrescaler(TIMx, pHandle->HALLMaxRatio);
    LL_TIM_GenerateEvent_UPDATE(TIMx);

    /* Clear the TIMx's pending flags. */
    WRITE_REG(TIMx->SR, 0);

    /* Selected input capture and Update (overflow) events generate interrupt. */

    /* Source of Update event is only counter overflow/underflow. */
    LL_TIM_SetUpdateSource(TIMx, LL_TIM_UPDATESOURCE_COUNTER);

    LL_TIM_EnableIT_CC1(TIMx);
    LL_TIM_EnableIT_UPDATE(TIMx);
    LL_TIM_SetCounter(TIMx, HALL_COUNTER_RESET);

    LL_TIM_CC_EnableChannel(TIMx, LL_TIM_CHANNEL_CH1);
    LL_TIM_EnableCounter(TIMx);

#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
}

/**
  * @brief  Clear software FIFO where are "pushed" latest speed information
  *         This function must be called before starting the motor to initialize
  *         the speed measurement process.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component.
  */
__weak void HALL_Clear(HALL_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    TIM_TypeDef *TIMx = pHandle->TIMx;

    pHandle->RatioDec = false;
    pHandle->RatioInc = false;

    /* Reset speed reliability. */
    pHandle->SensorIsReliable = true;

    pHandle->FirstCapt = 0U;
    pHandle->BufferFilled = 0U;
    pHandle->OVFCounter = 0U;
    pHandle->PhaseShiftCorrection = 0U;
    pHandle->StepCorrection = 0U;

    /* Initialize speed buffer index. */
    pHandle->SpeedFIFOIdx = 0U;

    /* Clear speed error counter */
    pHandle->_Super.bSpeedErrorNumber = 0;

    /* Re-initialize partly the timer. */
    LL_TIM_SetPrescaler(TIMx, pHandle->HALLMaxRatio);

    LL_TIM_SetCounter(TIMx, HALL_COUNTER_RESET);

    LL_TIM_EnableCounter(TIMx);

    HALL_State2Step(pHandle);
    
    /* Reset the SensorSpeed buffer. */
    uint8_t bIndex;
    for (bIndex = 0U; bIndex < pHandle->SpeedBufferSize; bIndex++)
    {
      pHandle->SensorPeriod[bIndex]  = (int32_t)pHandle->MaxPeriod * (int32_t)pHandle->Direction;
    }

    int32_t tempReg = (int32_t)pHandle->MaxPeriod * (int32_t)pHandle->SpeedBufferSize * (int32_t)pHandle->Direction;
    pHandle->ElPeriodSum = tempReg;
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
}

/**
  * @brief  This method must be called - at least - with the same periodicity
  *         on which speed control is executed.
  *         This method compute and store rotor istantaneous el speed (express
  *         in dpp considering the measurement frequency) in order to provide it
  *         to HALL_CalcElAngle function and SPD_GetElAngle.
  *         Then compute rotor average el speed (express in dpp considering the
  *         measurement frequency) based on the buffer filled by IRQ, then - as
  *         a consequence - compute, store and return - through parameter
  *         hMecSpeedUnit - the rotor average mech speed, expressed in Unit.
  *         Then check, store and return the reliability state of
  *         the sensor; in this function the reliability is measured with
  *         reference to specific parameters of the derived
  *         sensor (HALL) through internal variables managed by IRQ.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component
  * @param  hMecSpeedUnit pointer to int16_t, used to return the rotor average
  *         mechanical speed (expressed in the unit defined by #SPEED_UNIT).
  * @retval true = sensor information is reliable.
  *         false = sensor information is not reliable.
  */
__weak bool HALL_CalcAvrgMecSpeedUnit(HALL_6S_Handle_t *pHandle)
{
  bool bReliability;
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (MC_NULL == pHandle)
  {
    bReliability = false;
  }
  else
  {
#endif
    uint32_t wCaptBuf;
    int16_t MecSpeedUnit = pHandle->_Super.hAvrMecSpeedUnit;

    if (pHandle->SensorPeriod[pHandle->SpeedFIFOIdx] < 0)
    {
      wCaptBuf = (uint32_t) (- pHandle->SensorPeriod[pHandle->SpeedFIFOIdx]);
    }
    else
    {
      wCaptBuf = (uint32_t) (pHandle->SensorPeriod[pHandle->SpeedFIFOIdx]);
    }      
    
    /* Filtering to0 fast speed... could be a glitch? */
    /* The MAX_PSEUDO_SPEED is temporary in the buffer, and never included in average computation. */
    if ((wCaptBuf < pHandle->MinPeriod) || (wCaptBuf == pHandle->MaxPeriod))
    {
      if (pHandle->BufferFilled < pHandle->SpeedBufferSize)
      {
        MecSpeedUnit = 0;
      }
      else
      {  
        /* Nothing to do. */
      }
    }
    else
    {
      if (pHandle->BufferFilled < pHandle->SpeedBufferSize)
      {
        MecSpeedUnit = (int16_t)(((int32_t)pHandle->_Super.speedConvFactor) / pHandle->SensorPeriod[pHandle->SpeedFIFOIdx]) ;
      }
      else
      {
        /* Average speed allow to smooth the mechanical sensors misalignement. */
        MecSpeedUnit = (int16_t)((int32_t)pHandle->_Super.speedConvFactor /
                                 (pHandle->ElPeriodSum / (int32_t)pHandle->SpeedBufferSize)); /* Average value. */
      }
    }
    
    bReliability = SPD_IsMecSpeedReliable(&pHandle->_Super, MecSpeedUnit);
    pHandle->_Super.hAvrMecSpeedUnit = MecSpeedUnit;
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
  return (bReliability);
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section(".ccmram")))
#endif
#endif
/**
  * @brief  Example of private method of the class HALL to implement an MC IRQ function
  *         to be called when TIMx capture event occurs.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component.
  */
__weak void HALL_TIMx_CC_IRQHandler(HALL_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    uint32_t wCaptBuf;
    uint16_t hPrscBuf;
    uint16_t hHighSpeedCapture = 0U; /* In case of Sensor Is not Reliable or first capture cases. */
    TIM_TypeDef *TIMx = pHandle->TIMx;
    
    /* A capture event generated this interrupt. */
    HALL_State2Step(pHandle);
    
    if (pHandle->SensorIsReliable)
    {
      /* Discard first capture. */
      if (0U == pHandle->FirstCapt)
      {
        pHandle->FirstCapt++;
        (void)LL_TIM_IC_GetCaptureCH1(TIMx);
      }
      else
      {
        /* used to validate the average speed measurement. */
        if (pHandle->BufferFilled < pHandle->SpeedBufferSize)
        {
          pHandle->BufferFilled++;
        }
        else
        {
          /* Nothing to do. */
        }
        
        /* Store the latest speed acquisition. */
        hHighSpeedCapture = (uint16_t)LL_TIM_IC_GetCaptureCH1(TIMx);
        wCaptBuf = (uint32_t)hHighSpeedCapture;
        hPrscBuf = (uint16_t)LL_TIM_GetPrescaler(TIMx);
        /* Add the numbers of overflow to the counter. */
        wCaptBuf += ((uint32_t)pHandle->OVFCounter) * 0x10000UL;
        
        if (pHandle->OVFCounter != 0U)
        {
          /* Adjust the capture using prescaler. */
          uint16_t hAux;
          hAux = hPrscBuf + 1U;
          wCaptBuf *= hAux;
          
          if (pHandle->RatioInc)
          {
            pHandle->RatioInc = false;  /* Previous capture caused overflow. */
            /* Don't change prescaler (delay due to preload/update mechanism). */
          }
          else
          {
            if (LL_TIM_GetPrescaler(TIMx) < pHandle->HALLMaxRatio) /* Avoid OVF w/ very low freq. */
            {
              pHandle->Prescaler = (LL_TIM_GetPrescaler(TIMx) + 1U); /* To avoid OVF during speed decrease. */
              pHandle->RatioInc = true;   /* new prsc value updated at next capture only. */
            }
            else
            {
              /* Nothing to do. */
            }
          }
        }
        else
        {
          /* If prsc preload reduced in last capture, store current register + 1. */
          if (pHandle->RatioDec) /* and don't decrease it again. */
          {
            /* Adjust the capture using prescaler. */
            uint16_t hAux;
            hAux = hPrscBuf + 2U;
            wCaptBuf *= hAux;
            
            pHandle->RatioDec = false;
          }
          else  /* If prescaler was not modified on previous capture. */
          {
            /* Adjust the capture using prescaler. */
            uint16_t hAux = hPrscBuf + 1U;
            wCaptBuf *= hAux;
            
            if (hHighSpeedCapture < LOW_RES_THRESHOLD) /* If capture range correct. */
            {
              if (LL_TIM_GetPrescaler(TIMx) > 0U) /* or prescaler cannot be further reduced. */
              {
                pHandle->Prescaler = (LL_TIM_GetPrescaler(TIMx) - 1U); /* Increase accuracy by decreasing prsc. */
                /* Avoid decrementing again in next capt.(register preload delay). */
                pHandle->RatioDec = true;
              }
              else
              {
                /* Nothing to do. */
              }
            }
            else
            {
              /* Nothing to do. */
            }
          }
        }
        
        /* Filtering to fast speed... could be a glitch? */
        /* the HALL_MAX_PSEUDO_SPEED is temporary in the buffer, and never included in average computation. */
        if (wCaptBuf < pHandle->MinPeriod)
        {
          /* Nothing to do. */
        }
        else
        {
          pHandle->SpeedFIFOIdx++;
          if (pHandle->SpeedFIFOIdx == pHandle->SpeedBufferSize)
          {
            pHandle->SpeedFIFOIdx = 0U;
          }
          pHandle->ElPeriodSum -= pHandle->SensorPeriod[pHandle->SpeedFIFOIdx]; /* value we gonna removed from the accumulator. */
          if (wCaptBuf >= pHandle->MaxPeriod)
          {
            pHandle->SensorPeriod[pHandle->SpeedFIFOIdx] = (int32_t)pHandle->MaxPeriod * pHandle->Direction;
          }
          else
          {
            pHandle->SensorPeriod[pHandle->SpeedFIFOIdx] = (int32_t)wCaptBuf ;
            pHandle->SensorPeriod[pHandle->SpeedFIFOIdx] *= pHandle->Direction;
          }
          pHandle->ElPeriodSum += pHandle->SensorPeriod[pHandle->SpeedFIFOIdx];
          /* Update pointers to speed buffer. */
        }
    
        /* Used to validate the average speed measurement. */
        if (pHandle->BufferFilled < pHandle->SpeedBufferSize)
        {
          pHandle->BufferFilled++;
        }
        else
        {
          /* Nothing to do. */
        }
        /* Reset the number of overflow occurred. */
        pHandle->OVFCounter = 0U;
      }
    }
    if ((0U == pHandle->PhaseShift) || (pHandle->PhaseShift >= 30U))
    {
      LL_TIM_DisableIT_CC2(pHandle->TIMx);
    }
    else
    {
      /* Set the remaining degrees after STEP_SHIFT set up into HALL_State2Step. */
      /* Compute the counter value % to speed vs 60 degrees (step). */
      pHandle->PhaseShiftCorrection = ((pHandle->PhaseShift >= 30U) ? 0U : pHandle->PhaseShift);
      uint32_t PhaseShiftTemp = ((pHandle->PhaseShiftCorrection * hHighSpeedCapture) / 60U);
      LL_TIM_OC_SetCompareCH2(pHandle->TIMx, PhaseShiftTemp);
      LL_TIM_EnableIT_CC2(pHandle->TIMx);
    }
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section(".ccmram")))
#endif
#endif
/**
  * @brief  Example of private method of the class HALL to implement an MC IRQ function
  *         to be called when TIMx update event occurs.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component.
  */
__weak void HALL_TIMx_UP_IRQHandler(HALL_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    TIM_TypeDef *TIMx = pHandle->TIMx;
    
    if (pHandle->SensorIsReliable)
    {
      uint16_t hMaxTimerOverflow;
      /* An update event occured for this interrupt request generation. */
      pHandle->OVFCounter++;
      
      hMaxTimerOverflow = (uint16_t)(((uint32_t)pHandle->HallTimeout * pHandle->OvfFreq)
                                     / ((LL_TIM_GetPrescaler(TIMx) + 1U) * 1000U));
      if (pHandle->OVFCounter >= hMaxTimerOverflow)
      {
        /* Reset the overflow counter. */
        pHandle->OVFCounter = 0U;
        
        /* Reset first capture flag. */
        pHandle->FirstCapt = 0U;
        pHandle->BufferFilled = 0U;
        
        /* Initialize speed buffer index. */
        pHandle->SpeedFIFOIdx = 0U;
        
        /* Reset the SensorSpeed buffer. */
        uint8_t bIndex;
        for (bIndex = 0U; bIndex < pHandle->SpeedBufferSize; bIndex++)
        {
          pHandle->SensorPeriod[bIndex]  = (int32_t)pHandle->MaxPeriod * pHandle->Direction;
        }
        
        int32_t tempReg = (int32_t)pHandle->MaxPeriod * (int32_t)pHandle->SpeedBufferSize * pHandle->Direction;
        pHandle->ElPeriodSum = tempReg;
      }
      else
      {
        /* Nothing to do. */
      }
    }
    else
    {
      /* Nothing to do. */
    }
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
}

/**
  * @brief  Read the logic level of the three Hall sensor and individuates in this
  *         way the position of the rotor. Electrical angle is then initialized.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component.
  */
void HALL_State2Step(HALL_6S_Handle_t *pHandle)
{
  HALL_ReadState(pHandle);

  if (pHandle->PhaseShift >= 30U)
  {
    pHandle->StepCorrection = 1U;
  }
  else
  {
    pHandle->StepCorrection = 0U;
  }

  switch (pHandle->HallState)
  {
    case STATE_5:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection  + 1U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 4U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      break;
    }

    case STATE_1:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 2U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 5U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      break;
    }

    case STATE_3:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 3U;
        if (pHandle->Step >= 6U)
        {
         pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection;
      }
      break;
    }

    case STATE_2:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 4U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 1U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      break;
    }

    case STATE_6:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 5U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 2U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      break;
    }

    case STATE_4:
    {
      if (pHandle->Direction == POSITIVE)
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection;
      }
      else
      {
        pHandle->Step = pHandle->StepShift + pHandle->StepCorrection + 3U;
        if (pHandle->Step >= 6U)
        {
          pHandle->Step -= 6U;
        }
        else
        {
          /* Nothing to do. */
        }
      }
      break;
    }

    default:
    {
      /* Bad hall sensor configutarion so update the speed reliability. */
      pHandle->SensorIsReliable = false;
      break;
    }
  }
}

/**
  * @brief  Read the logic level of the three Hall sensor and individuates in this
  *         way the position of the rotor (+/- 30ÃƒÂ¯Ã‚Â¿Ã‚Â½). Electrical angle is then
  *         initialized.
  * @param  pHandle: handler of the current instance of the hall_speed_pos_fdbk component.
  */
static void HALL_ReadState(HALL_6S_Handle_t *pHandle)
{
  bool SensorReliable = true;
  uint8_t bPrevHallState = pHandle->HallState;
  if (DEGREES_120 == pHandle->SensorPlacement)
  {
    pHandle->HallState  = (uint8_t)((LL_GPIO_IsInputPinSet(pHandle->H3Port, pHandle->H3Pin) << 2U)
                                  | (LL_GPIO_IsInputPinSet(pHandle->H2Port, pHandle->H2Pin) << 1U)
                                  | LL_GPIO_IsInputPinSet(pHandle->H1Port, pHandle->H1Pin));
  }
  else
  {
    pHandle->HallState  = (uint8_t)(((LL_GPIO_IsInputPinSet(pHandle->H2Port, pHandle->H2Pin) ^ 1U) << 2U)
                                   | (LL_GPIO_IsInputPinSet(pHandle->H3Port, pHandle->H3Pin) << 1U)
                                   | LL_GPIO_IsInputPinSet(pHandle->H1Port, pHandle->H1Pin));
  }

  switch (pHandle->HallState)
  {
    case STATE_5:
    {
      if (((STATE_4 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_1 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
      else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }

    case STATE_1:
    {
      if (((STATE_5 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_3 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
      else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }

    case STATE_3:
    {
      if (((STATE_1 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_2 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
      else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }

    case STATE_2:
    {
      if (((STATE_3 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_6 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
      else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }

    case STATE_6:
    {
      if (((STATE_2 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_4 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
        else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }
    case STATE_4:
    {
      if (((STATE_6 == bPrevHallState) && (pHandle->Direction == POSITIVE)) ||
          ((STATE_5 == bPrevHallState) && (pHandle->Direction == NEGATIVE)))
      {
        /* Nothing to do. */
      }
      else
      {
        /* Bad hall sensor configutarion so update the speed reliability. */
        SensorReliable = false;
      }
      break;
    }

    default:
    {
      /* Bad hall sensor configutarion so update the speed reliability. */
      SensorReliable = false;
      break;
    }
  }
  pHandle->SensorIsReliable = SensorReliable;
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
/** @} */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
