/**
  ******************************************************************************
  * @file    revup_ctrl_sixstep.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains all definitions and functions prototypes for the
  *          Six-step variant of the Rev up control component of the Motor Control 
  *          SDK.
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
  * @ingroup RevUpCtrl6S
  */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef REVUP_CTRL_SIXSTEP_H
#define REVUP_CTRL_SIXSTEP_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* Includes ------------------------------------------------------------------*/
#include "mc_type.h"
#include "speed_duty_ctrl.h"
#include "bus_voltage_sensor.h"
#include "power_stage_parameters.h"

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup SixStep
  * @{
  */


/** @addtogroup SIXSTEP_MCLLAPI
  * @{
  */


/** @addtogroup RevUpCtrl6S
 * @{
 */

/* Exported constants --------------------------------------------------------*/
/**
  * @brief Maximum number of phases allowed for RevUp process.
  *
  */
#define RUC_MAX_PHASE_NUMBER 5u

/**
  * @brief RevUpCtrl_PhaseParams_t structure used for phases definition
  *
  */
typedef struct
{
  uint16_t hDurationms;         /* Duration of the RevUp phase.
                                   This parameter is expressed in millisecond. */
  int16_t hFinalMecSpeedUnit;   /* Mechanical speed in expressed in SPEED_UNIT assumed by VSS at the end of
                                   the RevUp phase. */
  uint16_t hFinalPulse;         /**< @brief Final pulse factor according to sensed Vbus value
                                     This field parameter is dedicated to 6-Step use. */
  void * pNext;                 /* Pointer on the next phase section to proceed
                                   This parameter is NULL for the last element. */
} RevUpCtrl_6S_PhaseParams_t;

/**
* 
*
*/

typedef struct
{
  SpeednPosFdbk_6S_Handle_t _Super;
  uint16_t hRUCFrequencyHz;                                    /* Frequency call to main RevUp procedure RUC_Exec.
                                                                  This parameter is equal to speed loop frequency. */
  uint16_t hPhaseRemainingTicks;                               /* Number of clock events remaining to complete the phase. */
  int8_t hDirection;                                           /* Motor direction.
                                                                  This parameter can be any value -1 or +1 */
  RevUpCtrl_6S_PhaseParams_t *pCurrentPhaseParams;             /* Pointer on the current RevUp phase processed. */
  RevUpCtrl_6S_PhaseParams_t ParamsData[RUC_MAX_PHASE_NUMBER]; /* Start up Phases sequences used by RevUp controller.
                                                                 Up to five phases can be used for the start up. */
  uint16_t PulseUpdateFactor;                                  /**< @brief Used to update the 6-Step rev-up pulse to the actual Vbus value.
                                                                    This field parameter is dedicated to 6-Step use. */
  uint8_t bPhaseNbr;                                           /* Number of phases relative to the programmed RevUp sequence.
                                                                  This parameter can be any value from 1 to 5. */
  uint16_t hMinStartUpValidSpeed;                              /* Minimum rotor speed required to validate the startup.
                                                                  This parameter is expressed in SPED_UNIT. */
  uint8_t bStageCnt;                                           /**< @brief Counter of executed phases.
                                                                    This parameter can be any value from 0 to 5 */
  SpeednDutyCtrl_Handle_t *pSDC;                               /**< @brief Speed and torque controller object used by RevUpCtrl. */
  uint32_t ElAccSpeedRefUnit;                                  /*!< step time increment expressed in timer ticks. */
  uint32_t ElSpeedTimerDpp;                                    /*!< step time expressed in timer ticks. */
  uint32_t CurrentSpeedRefUnit;                                /*!< Backup of hFinalMecSpeed to be applied in
                                                                    the last step. */

} RevUpCtrl_6S_Handle_t;


/* Exported functions ------------------------------------------------------- */

/* Initializes the internal RevUp controller state */
void RUC_6S_Clear(RevUpCtrl_6S_Handle_t *pHandle, int16_t hMotorDirection);

/* Updates the pulse according to sensed Vbus value */
void RUC_6S_UpdatePulse(RevUpCtrl_6S_Handle_t *pHandle, BusVoltageSensor_Handle_t *BusVHandle);

/* Main Rev-Up controller procedure executing overall programmed phases. */
bool RUC_6S_Exec(RevUpCtrl_6S_Handle_t *pHandle);

/* @brief  It is used to check if this stage is used for align motor. */
bool RUC_6S_IsAlignStageNow(const RevUpCtrl_6S_Handle_t *pHandle);

bool RUC_6S_ObserverSpeedReached(const RevUpCtrl_6S_Handle_t *pHandle);

/*
  * @brief  Allow to exit from RevUp process at the current rotor speed.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  */
static inline void RUC_6S_Stop(RevUpCtrl_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    pHandle->pCurrentPhaseParams = MC_NULL;
    pHandle->hPhaseRemainingTicks = 0U;
    pHandle->ElAccSpeedRefUnit = 0U;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/*
  * @brief  Provide current state of revup controller procedure.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Boolean set to true when entire revup phases have been completed.
  */
static inline bool RUC_6S_Completed(const RevUpCtrl_6S_Handle_t *pHandle)
{
  bool retVal = false;

#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    if (MC_NULL == pHandle->pCurrentPhaseParams)
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

/*
  * @brief  Allow to configure a revUp phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Returns boolean set to true
  */
static inline bool RUC_6S_SetPhase(RevUpCtrl_6S_Handle_t *pHandle, uint8_t phaseNumber, RevUpCtrl_6S_PhaseParams_t *phaseData)
{
  bool retValue = true;

#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if ((MC_NULL == pHandle) || (MC_NULL == phaseData))
  {
    retValue = false;
  }
  else
  {
#endif
    pHandle->ParamsData[phaseNumber].hFinalPulse = phaseData->hFinalPulse;
    pHandle->ParamsData[phaseNumber].hFinalMecSpeedUnit = phaseData->hFinalMecSpeedUnit;
    pHandle->ParamsData[phaseNumber].hDurationms = phaseData->hDurationms;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
  return (retValue);
}

/*
  * @brief  Allow to read total number of programmed phases.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Returns number of phases relative to the programmed revup.
  */
static inline uint8_t RUC_6S_GetNumberOfPhases(const RevUpCtrl_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  return ((MC_NULL == pHandle) ? 0U : (uint8_t)pHandle->bPhaseNbr);
#else
  return ((uint8_t)pHandle->bPhaseNbr);
#endif
}

/*
  * @brief  Allow to read a programmed phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Returns number of phases relative to the programmed revup.
  */
static inline bool RUC_6S_GetPhase(RevUpCtrl_6S_Handle_t *pHandle, uint8_t phaseNumber, RevUpCtrl_6S_PhaseParams_t *phaseData)
{
  bool retValue = true;

#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if ((MC_NULL == pHandle) || (MC_NULL == phaseData))
  {
    retValue = false;
  }
  else
  {
#endif
    phaseData->hFinalPulse = pHandle->ParamsData[phaseNumber].hFinalPulse;
    phaseData->hFinalMecSpeedUnit = (int16_t)pHandle->ParamsData[phaseNumber].hFinalMecSpeedUnit;
    phaseData->hDurationms = (uint16_t)pHandle->ParamsData[phaseNumber].hDurationms;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
  return (retValue);
}

/*
  * @brief  Allows to modify duration (ms unit) of a selected phase. .
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase where new duration shall be modified.
  *         This parameter must be a number between 0 and 6.
  * @param  hDurationms: new duration value required for associated phase.
  *         This parameter must be set in millisecond.
  */
static inline void RUC_6S_SetPhaseDurationms(RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase, uint16_t hDurationms)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->ParamsData[bPhase].hDurationms = hDurationms;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/*
  * @brief  Allow to modify targeted mechanical speed of a selected phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase where new mechanical speed shall be modified.
  *         This parameter must be a number between 0 and 6.
  * @param  hFinalMecSpeed: new targeted mechanical speed.
  *         This parameter must be expressed in 0.1Hz.
  */
static inline void RUC_6S_SetPhaseFinalMecSpeedUnit(RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase, int16_t hFinalMecSpeedUnit)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->ParamsData[bPhase].hFinalMecSpeedUnit = hFinalMecSpeedUnit;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/**
  * @brief  Function used to set the final Pulse targeted at the end of a specific phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase where new the motor torque shall be modified.
  *         This parameter must be a number between 0 and 6.
  * @param  hFinalPulse: new targeted motor torque.
  */
static inline void RUC_6S_SetPhaseFinalPulse(RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase, uint16_t hFinalPulse)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->ParamsData[bPhase].hFinalPulse = hFinalPulse;
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  }
#endif
}

/*
  * @brief  Allow to read duration set in selected phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase from where duration is read.
  *         This parameter must be a number between 0 and 6.
  *  @retval Returns duration used in selected phase.
  */
static inline uint16_t RUC_6S_GetPhaseDurationms(const RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  return ((MC_NULL == pHandle) ? 0U : (uint16_t)pHandle->ParamsData[bPhase].hDurationms);
#else
  return ((uint16_t)pHandle->ParamsData[bPhase].hDurationms);
#endif
}

/*
  * @brief  Allow to read targeted rotor speed set in selected phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase from where targeted rotor speed is read.
  *         This parameter must be a number between 0 and 6.
  *  @retval Returns targeted rotor speed set in selected phase.
  */
static inline int16_t RUC_6S_GetPhaseFinalMecSpeedUnit(const RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  return ((MC_NULL == pHandle) ? 0 : (int16_t)pHandle->ParamsData[bPhase].hFinalMecSpeedUnit);
#else
  return ((int16_t)pHandle->ParamsData[bPhase].hFinalMecSpeedUnit);
#endif
}

/**
  * @brief  Function used to read the Final pulse factor of a specific RevUp phase.
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  * @param  bPhase: RevUp phase from where targeted motor torque is read.
  *         This parameter must be a number between 0 and 6.
  *  @retval Returns targeted motor torque set in selected phase.
  */
static inline int16_t RUC_6S_GetPhaseFinalPulse(const RevUpCtrl_6S_Handle_t *pHandle, uint8_t bPhase)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  return ((MC_NULL == pHandle) ? 0 : (int16_t)pHandle->ParamsData[bPhase].hFinalPulse);
#else
  return ((int16_t)pHandle->ParamsData[bPhase].hFinalPulse);
#endif
}

/**
  * @brief  Allow to read spinning direction of the motor
  * @param  pHandle: Pointer on Handle structure of RevUp controller.
  *  @retval Returns direction of the motor.
  */
static inline int16_t RUC_6S_GetDirection(const RevUpCtrl_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_REV_UP_CTL_6STEP
  return ((MC_NULL == pHandle) ? 0 : (int16_t)pHandle->hDirection);
#else
  return ((int16_t)pHandle->hDirection);
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

#ifdef __cplusplus
}
#endif /* __cpluplus */

#endif /* REVUP_CTRL_SIXSTEP_H */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
