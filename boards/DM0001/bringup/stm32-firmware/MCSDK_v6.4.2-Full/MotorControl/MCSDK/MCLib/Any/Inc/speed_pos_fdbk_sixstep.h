/**
  ******************************************************************************
  * @file    speed_pos_fdbk_sixstep.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides all definitions and functions prototypes
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

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef SPEEDNPOSFDBK6S_H
#define SPEEDNPOSFDBK6S_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* Includes ------------------------------------------------------------------*/
/* Already into mc_type.h */
/* #include "stdint.h" */
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

/** @addtogroup speed_pos_fdbk_sixstep
  * @{
  */

/* Exported types ------------------------------------------------------------*/
/**
  * @brief  SpeednPosFdbk_6S_Handle_t  handles definitions of mechanical and electrical speed, mechanical acceleration, mechanical
  *                        and electrical angle and all constants and scale values for a reliable measure and
  *                        computation in appropriated unit.
  */
typedef struct
{

  uint8_t bSpeedErrorNumber;          /*!< Number of time the average mechanical speed is not valid. */
  uint8_t bElToMecRatio;              /*!< Coefficient used to transform electrical to mechanical quantities and
                                           viceversa. It usually coincides with motor pole pairs number. */
  uint8_t bMaximumSpeedErrorsNumber;  /*!< Maximum value of not valid speed measurements before an error is reported.*/
  int16_t hAvrMecSpeedUnit;           /*!< Average mechanical speed expressed in the unit defined by
                                           [SPEED_UNIT](measurement_units.md). */
  uint16_t hMaxReliableMecSpeedUnit;  /*!< Maximum value of measured mechanical speed that is considered to be valid.
                                           Expressed in the unit defined by [SPEED_UNIT](measurement_units.md). */
  uint16_t hMinReliableMecSpeedUnit;  /*!< Minimum value of measured mechanical speed that is considered to be valid.
                                           Expressed in the unit defined by [SPEED_UNIT](measurement_units.md).*/
  uint32_t speedConvFactor;           /*!< Conversion factor between time interval Delta T
                                          between bemf zero crossing points, express in timer
                                          counts, and electrical rotor speed express in dpp.
                                          Ex. Rotor speed (dpp) = wPseudoFreqConv / Delta T
                                           It will be ((CKTIM / 6) / (SAMPLING_FREQ)) * 65536.*/
} SpeednPosFdbk_6S_Handle_t;

int16_t SPD_GetAvrgMecSpeedUnit(const SpeednPosFdbk_6S_Handle_t *pHandle);

bool SPD_IsMecSpeedReliable(SpeednPosFdbk_6S_Handle_t *pHandle, int16_t MecSpeedUnit);

uint8_t SPD_GetElToMecRatio(const SpeednPosFdbk_6S_Handle_t *pHandle);

void SPD_SetElToMecRatio(SpeednPosFdbk_6S_Handle_t *pHandle, uint8_t bPP);

/**
  * @brief  Returns the result of the last reliability check performed.
  * @param  pHandle: handler of the current instance of the SpeednPosFdbk component.
  * @retval bool sensor reliability state.
  *
  * - Reliability is measured with reference to parameters
  * @ref SpeednPosFdbk_Handle_t::hMaxReliableMecSpeedUnit "hMaxReliableMecSpeedUnit",
  * @ref SpeednPosFdbk_Handle_t::hMinReliableMecSpeedUnit "hMaxReliableMecSpeedUnit",
  * @ref SpeednPosFdbk_Handle_t::bMaximumSpeedErrorsNumber "bMaximumSpeedErrorsNumber".
  * - If the number of time the average mechanical speed is not valid matches the
  *  maximum value of not valid speed measurements, sensor information is not reliable.
  * - Embedded into construction of the MC_GetSpeedSensorReliabilityMotor API.
  * - The return value is a boolean that expresses:\n
  * -- true  = sensor information is reliable.\n
  * -- false = sensor information is not reliable.
  */
static inline bool SPD_Check(const SpeednPosFdbk_6S_Handle_t *pHandle)
{
  bool SpeedSensorReliability = true;
#ifdef NULL_PTR_CHECK_SPD_POS_FBK
  if ((MC_NULL == pHandle) || (pHandle->bSpeedErrorNumber == pHandle->bMaximumSpeedErrorsNumber))
#else
  if (pHandle->bSpeedErrorNumber == pHandle->bMaximumSpeedErrorsNumber)
#endif
  {
    SpeedSensorReliability = false;
  }
  else
  {
    /* Nothing to do */
  }
  return (SpeedSensorReliability);
}

/**
  * @}
  */

/**
  * @}
  */

#ifdef __cplusplus
}
#endif /* __cpluplus */

#endif /* SPEEDNPOSFDBK6S_H */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
