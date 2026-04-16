/**
  ******************************************************************************
  * @file    speed_pos_fdbk_sixstep.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides firmware functions that implement the  features
  *          of the Speed & Position Feedback component of the Motor Control SDK.
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
  * @ingroup speed_pos_fdbk_sixstep
  */

/* Includes ------------------------------------------------------------------*/
#include "speed_pos_fdbk_sixstep.h"

/** @addtogroup MCSDK
  * @{
  */

  /** @addtogroup SixStep
  * @{
  */

/** @addtogroup SIXSTEP_MCLLAPI
  * @{
  */

/** @defgroup speed_pos_fdbk_sixstep Speed & Position Feedback
  *
  * @brief 6Step Speed & Position Feedback components of the Motor Control SDK
  *
  * These components provide the speed and the angular position of the rotor of a motor (both
  * electrical and mechanical). These informations are expressed in units defined into [measurement units](measurement_units.md).
  *
  * Several implementations of the Speed and Position Feedback feature are provided by the Motor
  * to account for the specificities of the motor used on the application:
  *
  *
  *   For more information see the user manual [Speed position and sensorless BEMF reconstruction](speed_pos_sensorless_bemf_reconstruction.md).
  * @{
  */



/**
  * @brief  Returns the last computed average mechanical speed, expressed in
  *         the unit defined by [SPEED_UNIT](measurement_units.md).
  * @param  pHandle: handler of the current instance of the SpeednPosFdbk component.
  */
__weak int16_t SPD_GetAvrgMecSpeedUnit(const SpeednPosFdbk_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  return ((MC_NULL == pHandle) ? 0 : pHandle->hAvrMecSpeedUnit);
#else
  return (pHandle->hAvrMecSpeedUnit);
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
  * @brief  Computes and returns through parameter @ref SpeednPosFdbk_6S_Handle_t::pMecSpeedUnit "pMecSpeedUnit",
  *         the rotor average mechanical speed, expressed in the unit defined by
  *         @ref SpeednPosFdbk_6S_Handle_t::SpeedUnit "SpeedUnit".
  * @param  pHandle: handler of the current instance of the SpeednPosFdbk component.
  * @param  MecSpeedUnit: pointer to int16_t, used to return the rotor average
  *         mechanical speed (expressed in the unit defined by [SPEED_UNIT](measurement_units.md).
  * @retval none
  *
  * - Computes and returns the reliability state of the sensor. Reliability is measured with
  * reference to parameters @ref SpeednPosFdbk_6S_Handle_t::hMinReliableMecSpeedUnit "hMinReliableMecSpeedUnit",
  * @ref SpeednPosFdbk_6S_Handle_t::hMaxReliableMecSpeedUnit "hMaxReliableMecSpeedUnit",
  * @ref SpeednPosFdbk_6S_Handle_t::bMaximumSpeedErrorsNumber "bMaximumSpeedErrorsNumber"\n
  * -- true  = sensor information is reliable.\n
  * -- false = sensor information is not reliable.\n
  * - Called at least with the same periodicity on which speed control is executed.
  *         -

  */
__weak bool SPD_IsMecSpeedReliable(SpeednPosFdbk_6S_Handle_t *pHandle, int16_t MecSpeedUnit)
{
  bool SpeedSensorReliability = true;
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  if (MC_NULL == pHandle)
  {
    SpeedSensorReliability = false;
  }
  else
  {
#endif
    uint16_t hAbsMecSpeedUnit;
    int16_t hAux;
    uint8_t bSpeedErrorNumber;
    uint8_t bMaximumSpeedErrorsNumber = pHandle->bMaximumSpeedErrorsNumber;
    bool SpeedError = false;

    bSpeedErrorNumber = pHandle->bSpeedErrorNumber;

    /* Compute absoulte value of mechanical speed */
    if (MecSpeedUnit < 0)
    {
      hAux = -(MecSpeedUnit);
      hAbsMecSpeedUnit = (uint16_t)hAux;
    }
    else
    {
      hAbsMecSpeedUnit = (uint16_t)MecSpeedUnit;
    }

    if (hAbsMecSpeedUnit > pHandle->hMaxReliableMecSpeedUnit)
    {
      SpeedError = true;
    }
    else
    {
      /* Nothing to do */
    }

    if (hAbsMecSpeedUnit < pHandle->hMinReliableMecSpeedUnit)
    {
      SpeedError = true;
    }
    else
    {
      /* Nothing to do */
    }

    if (true == SpeedError)
    {
      if (bSpeedErrorNumber < bMaximumSpeedErrorsNumber)
      {
        bSpeedErrorNumber++;
      }
      else
      {
        /* Nothing to do */
      }
    }
    else
    {
      if (bSpeedErrorNumber < bMaximumSpeedErrorsNumber)
      {
        bSpeedErrorNumber = 0u;
      }
      else
      {
        /* Nothing to do */
      }
    }

    if (bSpeedErrorNumber == bMaximumSpeedErrorsNumber)
    {
      SpeedSensorReliability = false;
    }
    else
    {
      /* Nothing to do */
    }

    pHandle->bSpeedErrorNumber = bSpeedErrorNumber;
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  }
#endif
  return (SpeedSensorReliability);
}

/**
  * @brief  Returns the coefficient used to transform electrical to
  *         mechanical quantities and viceversa. It usually coincides with motor
  *         pole pairs number.
  * @param  pHandle: handler of the current instance of the SpeednPosFdbk component.
  * @retval uint8_t The motor pole pairs number.
  *
  * - Called by motor profiling functions and for monitoring through motorPilote.
  */
__weak uint8_t SPD_GetElToMecRatio(const SpeednPosFdbk_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  return ((MC_NULL == pHandle) ? 0U : pHandle->bElToMecRatio);
#else
  return (pHandle->bElToMecRatio);
#endif
}

/**
  * @brief  Sets the coefficient used to transform electrical to
  *         mechanical quantities and viceversa. It usually coincides with motor
  *         pole pairs number.
  * @param  pHandle: handler of the current instance of the SpeednPosFdbk component.
  * @param  bPP The motor pole pairs number to be set.
  *
  * - Called only for monitoring through motorPilote.
  */
__weak void SPD_SetElToMecRatio(SpeednPosFdbk_6S_Handle_t *pHandle, uint8_t bPP)
{
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    pHandle->bElToMecRatio = bPP;
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  }
#endif
}


/**
  * @}
  */

/**
  * @}
  */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
