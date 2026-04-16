
/**
  ******************************************************************************
  * @file    hall_speed_pos_fdbk_sixstep.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains all definitions and functions prototypes for the
  *          Hall Speed & Position Feedback component of the Motor Control SDK.
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

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef HALL_SPEEDNPOSFDBK_H
#define HALL_SPEEDNPOSFDBK_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

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


/** @addtogroup hall_speed_pos_fdbk_sixstep
  * @{
  */

#define HALL_SPEED_FIFO_SIZE  ((uint8_t)18)

/* HALL SENSORS PLACEMENT ----------------------------------------------------*/
#define DEGREES_120 0u
#define DEGREES_60  1u

/* Exported types ------------------------------------------------------------*/

/**
  * @brief HALL component parameters definition
  *
  */

typedef struct
{
  SpeednPosFdbk_6S_Handle_t _Super;

  /* SW Settings */
  uint8_t SensorPlacement;                    /*!< Define here the mechanical position of the sensors
                                                   with reference to an electrical cycle.
                                                   Allowed values are: DEGREES_120 or DEGREES_60. */
  uint32_t PhaseShift;                        /*!< Define here in s16degree the electrical phase shift
                                                   between the low to high transition of signal H1 and
                                                   the maximum of the Bemf induced on phase A betwenn [0..59].*/
  uint8_t StepShift;                          /*!< Define here in s16degree the electrical phase shift
                                                   between the low to high transition of signal H1 and
                                                   the maximum of the Bemf induced on phase A in 60 degre step.*/
  uint8_t StepCorrection;                     /*!< Step shift correction after check of the PhaseShift 
                                                    if PhaseShift > 30 -> StepCorrection = 1.*/
  uint8_t PhaseShiftCorrection;               /*!< Remaining degress of PhaseShift after PhaseShiftCorrection computaton.
                                                    if PhaseShift > 30 -> PhaseShiftCorrection = 0.*/
  uint8_t SpeedBufferSize;                    /*!< Size of the buffer used to calculate the average
                                                   speed. It must be less than 18. */

  /* HW Settings */
  uint32_t TIMClockFreq;                      /*!< Timer clock frequency express in Hz. */
  TIM_TypeDef *TIMx;                          /*!< Timer used for HALL sensor management. */
  GPIO_TypeDef *H1Port;                       /*!< HALL sensor H1 channel GPIO input port (if used,
                                                   after re-mapping). It must be GPIOx x= A, B, ... */
  uint32_t  H1Pin;                            /*!< HALL sensor H1 channel GPIO output pin (if used,
                                                   after re-mapping). It must be GPIO_Pin_x x= 0, 1, ... */
  GPIO_TypeDef *H2Port;                       /*!< HALL sensor H2 channel GPIO input port (if used,
                                                   after re-mapping). It must be GPIOx x= A, B, ...*/
  uint32_t  H2Pin;                            /*!< HALL sensor H2 channel GPIO output pin (if used,
                                                   after re-mapping). It must be GPIO_Pin_x x= 0, 1, ... */
  GPIO_TypeDef *H3Port;                       /*!< HALL sensor H3 channel GPIO input port (if used,
                                                   after re-mapping). It must be GPIOx x= A, B, ... */
  uint32_t H3Pin;                             /*!< HALL sensor H3 channel GPIO output pin (if used,
                                                   after re-mapping). It must be GPIO_Pin_x x= 0, 1, ... */
  uint32_t ICx_Filter;                        /*!< Input Capture filter selection. */
  bool SensorIsReliable;                      /*!< Flag to indicate a wrong configuration
                                                   of the Hall sensor signanls. */
  volatile bool RatioDec;                     /*!< Flag to avoid consecutive prescaler decrement. */
  volatile bool RatioInc;                     /*!< Flag to avoid consecutive prescaler increment. */
  volatile uint8_t FirstCapt;                 /*!< Flag used to discard first capture for
                                                   the speed measurement. */
  volatile uint8_t BufferFilled;              /*!< Indicate the number of speed measuremt
                                                   present in the buffer from the start.
                                                   It will be max bSpeedBufferSize and it
                                                   is used to validate the start of speed
                                                   averaging. If bBufferFilled is below
                                                   bSpeedBufferSize the instantaneous
                                                   measured speed is returned as average
                                                   speed.*/
  volatile uint16_t OVFCounter;               /*!< Count overflows if prescaler is too low. */
  int32_t SensorPeriod[HALL_SPEED_FIFO_SIZE]; /*!< Holding the last period captures. */
  uint8_t SpeedFIFOIdx;                       /*!< Pointer of next element to be stored in
                                                   the speed sensor buffer. */
  int32_t ElPeriodSum;                        /* Period accumulator used to speed up the average speed computation. */
  int8_t Direction;                           /*!< Instantaneous direction of rotor between two captures. */
  uint8_t HallState;                          /*!< Current HALL state configuration. */
  uint16_t HALLMaxRatio;                      /*!< Max TIM prescaler ratio defining the lowest
                                                   expected speed feedback. */
  uint32_t MaxPeriod;                         /*!< Time delay between two sensor edges when the speed
                                                   of the rotor is the minimum realistic in the
                                                   application: this allows to discriminate too low freq for instance.
                                                   This period shoud be expressed in timer counts and it will be:
                                                   wMaxPeriod = ((10 * CKTIM) / 6) / MinElFreq(0.1Hz). */
  uint32_t MinPeriod;                         /*!< Time delay between two sensor edges when the speed
                                                   of the rotor is the maximum realistic in the
                                                   application: this allows discriminating glitches for instance.
                                                   This period shoud be expressed in timer counts and it will be:
                                                   wSpeedOverflow = ((10 * CKTIM) / 6) / MaxElFreq(0.1Hz). */
  uint16_t HallTimeout;                       /*!< Max delay between two Hall sensor signal to assert
                                                   zero speed express in milliseconds. */
  uint16_t OvfFreq;                           /*!< Frequency of timer overflow (from 0 to 0x10000)
                                                   it will be: hOvfFreq = CKTIM /65536. */
  uint8_t Step;                               /*!< Step */
  uint32_t Prescaler;                         /*!< Prescaler valut stored to be written at commutation */
} HALL_6S_Handle_t;

void HALL_TIMx_UP_IRQHandler(HALL_6S_Handle_t *pHandle);
void HALL_TIMx_CC_IRQHandler(HALL_6S_Handle_t *pHandle);
void HALL_Init(HALL_6S_Handle_t *pHandle);
void HALL_Clear(HALL_6S_Handle_t *pHandle);
bool HALL_CalcAvrgMecSpeedUnit(HALL_6S_Handle_t *pHandle);

static inline uint8_t HALL_GetStep(const HALL_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  return ((NULL == pHandle) ? 0U : pHandle->Step);
#else
  return (pHandle->Step);
#endif
}

/**
  * @brief  Forces the rotation direction.
  * @param  pHandle: handler of the current instance of the Bemf_ADC component.
  * @param  direction: imposed direction.
  */
static inline void HALL_SetDirection(HALL_6S_Handle_t *pHandle,  int8_t direction)
{
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->Direction = direction;
#ifdef NULL_PTR_CHECK_HALL_SPD_POS_FDB
  }
#endif
}
/**
  * @}
  */

/**
  * @}
  */

/** @} */

#ifdef __cplusplus
}
#endif /* __cpluplus */

#endif /*HALL_SPEEDNPOSFDBK_H*/
/******************* (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
