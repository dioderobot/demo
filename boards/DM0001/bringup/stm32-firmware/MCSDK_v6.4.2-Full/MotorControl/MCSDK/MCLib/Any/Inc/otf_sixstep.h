/**
  ******************************************************************************
  * @file    otf_sixstep.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains all definitions and functions prototypes for the
  *          Six-step variant of the OnTheFly Rev up control component of the Motor Control 
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
  * @ingroup OnTheFly6S
  */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef OTF_SIXSTEP_H
#define OTF_SIXSTEP_H

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

/** @addtogroup OnTheFly6S
 * @{
 */

/* Exported constants --------------------------------------------------------*/

typedef struct
{
  uint32_t CCER_Brake_cfg;
  uint32_t CCER_cfg[6];
} OTF_TimerCfg_t;

/**
  * @brief OTF_6S_Handle_t handle definition
  *
  */
typedef struct
{
  RevUpCtrl_6S_Handle_t *pRevUp;  /* handle of the revup component. */
  PWMC_Handle_t         *pPwmc;   /* handle of the pwmc component. */
  Bemf_ADC_Handle_t     *pBADC;   /* handle of the BADC component. */
  int16_t AvrSpeed;               /* calculated average speed . */
  uint32_t speedTimerPsc;         /* prescaler of the speed timer. */
  bool OTFongoing;                /* flag raised when OTF procedure is ongoing. */
  bool OTFabort;                  /* flag raised when OTF procedure is aborted. */
  uint8_t maxConsecutiveBemfTransitions; /* number of consecutive consistent BEMF zero crossings to validate the startup. */
  uint8_t BemfErrors;       /* counter of the inconsistent bemf transitions*/
  uint16_t maxBemfErrors;    /* number of errors before aborting the procedure. */
  uint8_t ConsecutiveBemfTransitions; /* counter of the consistent bemf transitions*/
  uint16_t pSensing_ThresholdPerc; /* bemf threshold set to validate a bemf transition*/
  uint32_t LSDetectCnt;  /* duty cycle of the LS MOS during the detection phase */
  uint32_t LSBrakeCnt;   /* duty cycle of the LS MOS during the braking phase */
  uint32_t speedDutyConvFactor;  /* conversion factor between speed and duty cycle */
  OTF_TimerCfg_t * TimerCfg;
  uint32_t       aBuffer[8];                   /*!< Buffer used to compute average value of the speedDutyConvFactor.*/
  uint8_t        index;                      /*!< Index of last stored element in the average buffer.*/
  uint8_t       LowPassFilterBW;            /*!< first order software filter bandwidth */
} OTF_6S_Handle_t;


/* Exported functions ------------------------------------------------------- */

/* Initializes the internal OnTheFly controller state */
void OTF_6S_Init(OTF_6S_Handle_t *pHandle, BusVoltageSensor_Handle_t *BusVHandle);

/* Clears the internal OnTheFly controller state when procedure is aborted*/
void OTF_6S_Clear(OTF_6S_Handle_t *pHandle);

/* Manages the bemf zero crossing detection */
bool OTF_6S_Task(OTF_6S_Handle_t *pHandle);

/* Turns on the outputs if procedure is successful*/
void OTF_6S_SwitchOver(OTF_6S_Handle_t *pHandle);

/*Real time calculation of the convertion factor between speed and duty cycle */
void OTF_6S_UpdateDutyConv( OTF_6S_Handle_t *pHandle, uint16_t duty);

/*Returns the calculated duty cycle extimated from speed*/
uint16_t OTF_6S_CalcSpeedReference(OTF_6S_Handle_t *pHandle);

/**
  * @brief It returns the state of the OnTheFly procedure.
  * @param  pHandle: handler of the current instance of the OTF_6S_Handle_t component
  * @retval bool: true if procedure is ongoning.
  */
static inline bool OTF_6S_IsOngoing( OTF_6S_Handle_t *pHandle)
{
  return ((MC_NULL ==  pHandle) ? false : pHandle->OTFongoing);
}

/**
  * @brief It returns the abort flag.
  * @param  pHandle: handler of the current instance of the OTF_6S_Handle_t component
  * @retval bool: true if procedure was aborted.
  */
static inline bool OTF_6S_IsAborted( OTF_6S_Handle_t *pHandle)
{
  return ((MC_NULL ==  pHandle) ? false : pHandle->OTFabort);
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
