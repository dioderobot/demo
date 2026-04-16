/**
  ******************************************************************************
  * @file   ipd_sixstep.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains all definitions and functions prototypes for the
  *          Six-step variant of the Initial Position Detection Control control
  *          component of the Motor Control SDK.
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
  * @ingroup InitialPositionDetect6S
  */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef IPD_SIXSTEP_H
#define IPD_SIXSTEP_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* Includes ------------------------------------------------------------------*/
#include "revup_ctrl_sixstep.h"
#include "bemf_ADC_fdbk_sixstep.h"
#include "pwmc_sixstep.h"

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup SixStep
  * @{
  */


/** @addtogroup SIXSTEP_MCLLAPI
  * @{
  */



/** @addtogroup InitialPositionDetect6S
 * @{
 */

/* Exported constants --------------------------------------------------------*/

/**
  * @brief IPD_6S_Handle_t handle definition
  *
  */
typedef struct
{
  uint32_t OnePulsePwmTimerDutyCycle;      /*!< One pulse mode PWM timer duty cycle. */
  uint32_t OnePulsePwmTimerFrequency;      /*!< One pulse mode PWM timer frequency. */
  uint32_t IpdPwmPeriodCycle;              /* Timer counter max value. */
  int32_t Flux_6_Steps[12];                /* Table of the Flux computation based on ADC Shunt measurements. */
  uint16_t ValideBemfDetectedThreshold;    /*!< Threshold corresponding of number of valid BEMF detected during scan phase. */
  uint16_t NumZeroSpeedSamples;            /* BEMF detected samples. */
  uint16_t NumZeroSpeedStepValid;          /* Valid IPD start before going to RUN state. */
  uint16_t NumZeroSpeedStepValidThreshold; /*!< Threshold of valid IPD start before going to RUN state. */
  int16_t ElecAngle;                       /* Electrical angle computed after Timer PWM sequence generation. */
  int16_t ADC_JDR_Currents[6];             /* Table that contains the shunt ADC measurement. */
  uint8_t PreviousStep;                    /* Current step number */
  uint8_t previous_angle;                  /* Last Step generated for BEMF measurements to validate angle. */
  bool IPDStartUpFlag;                     /*!< IPD StartUp flag true=enabled, false=disabled. */
  bool IPDRunning;                         /*!< true = IPD is on going, false = IPD has finished. */
  bool IPDBEMFMeasured;                    /* BEMF measurement occured. */
  bool IPDPulseRunState;                   /* IPD Pulse running. */
  bool IPDDebug;                           /*!< True = Start with no IPD and no REvUP. */
} IPD_6S_Handle_t;

typedef struct
{
  uint32_t BLDC_CCER_ZeroSpeed[6];
  uint32_t BLDC_CCMR1_ZeroSpeed[6];
  uint32_t BLDC_CCMR2_ZeroSpeed[6];
} IPD_TimerCfg_t;

/* Exported functions ------------------------------------------------------- */

/* Clears the internal OnTheFly controller state when procedure is aborted*/
void IPD_6S_Clear(IPD_6S_Handle_t *pHandle);

/* Execute the IPD startup Process. */
bool IPD_6S_Task(IPD_6S_Handle_t *pHandle, PWMC_Handle_t *PwmcHandle);

/**
  * @brief  Set the IPDStartUpFlag.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
static inline void IPD_6S_SetIPDStartUpFlag(IPD_6S_Handle_t *pHandle, uint8_t Data)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->IPDStartUpFlag = ((0U == Data) ? false : true);
#ifdef NULL_PTR_CHECK_IPD_6S
  }
#endif
}

/**
  * @brief  Set the IPDDebug.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
static inline void IPD_6S_SetIPDDebugFlag(IPD_6S_Handle_t *pHandle, uint8_t Data)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->IPDDebug = ((0U == Data) ? false : true);
#ifdef NULL_PTR_CHECK_IPD_6S
  }
#endif
}

/**
  * @brief  Get The IPDPulseRunState.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval bool IPDPulseRunState: true = enabled, false = disabled
  */
static inline bool IPD_6S_GetIPDPulseRunStateFlag(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? false : pHandle->IPDPulseRunState);
#else
  return (pHandle->IPDPulseRunState);
#endif
}

/**
  * @brief  Get The IPDDebug.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval bool IPDPulseRunState: true = enabled, false = disabled
  */
static inline bool IPD_6S_GetIPDDebugFlag(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? false : pHandle->IPDDebug);
#else
  return (pHandle->IPDDebug);
#endif
}

/**
  * @brief  Get The IPDStartUpFlag.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval bool IPDStartUpFlag: true = enabled, false = disabled
  */
static inline bool IPD_6S_GetIPDStartUpFlag(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? false : pHandle->IPDStartUpFlag);
#else
  return (pHandle->IPDStartUpFlag);
#endif
}

/**
  * @brief  Get the ValideBemfDetectedThreshold.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval uint16_t ValideBemfDetectedThreshold: number of BEMF sample to check before go into RUN state
  */
static inline uint16_t IPD_6S_GetNbBemfSampleToRun(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? 0U : pHandle->ValideBemfDetectedThreshold);
#else
  return (pHandle->ValideBemfDetectedThreshold);
#endif
}

/**
  * @brief  Get the NumZeroSpeedStepValidThreshold.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval uint16_t NumZeroSpeedStepValidThreshold: number of BEMF sample to check before go into RUN state
  */
static inline uint16_t IPD_6S_GetValidNbStepToRun(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? 0U : pHandle->NumZeroSpeedStepValidThreshold);
#else
  return (pHandle->NumZeroSpeedStepValidThreshold);
#endif
}

/**
  * @brief  Set the ValideBemfDetectedThreshold.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
static inline void IPD_6S_SetNbBemfSampleToRun(IPD_6S_Handle_t *pHandle, uint16_t Data)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->ValideBemfDetectedThreshold = Data;
#ifdef NULL_PTR_CHECK_IPD_6S
  }
#endif
}

/**
  * @brief  Set the ValideBemfDetectedThreshold.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  */
static inline void IPD_6S_SetNbStepToRun(IPD_6S_Handle_t *pHandle, uint16_t Data)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    pHandle->NumZeroSpeedStepValidThreshold = Data;
#ifdef NULL_PTR_CHECK_IPD_6S
  }
#endif
}

/**
  * @brief  Get the IPDRunning.
  * @param  pHandle: Pointer on Handle structure of IPD controller.
  * @retval bool IPDRunning: true = IPD algorithm running, false = IPD algorithm goes to RUN state.
  */
static inline bool IPD_6S_GetRunStateFlag(const IPD_6S_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_IPD_6S
  return ((MC_NULL == pHandle) ? false : pHandle->IPDRunning);
#else
  return (pHandle->IPDRunning);
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

#endif /* IPD_SIXSTEP_H */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
