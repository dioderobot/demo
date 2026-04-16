/**
  ******************************************************************************
  * @file    bemf_speed_pos_fdbk.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains definitions and functions prototypes common to all
  *          six-step sensorless based Speed & Position Feedback components of the Motor
  *          Control SDK.
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
  * @ingroup SpeednPosFdbk_Bemf
  */


/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __BEMF_SPEEDNPOSFDBK_H
#define __BEMF_SPEEDNPOSFDBK_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* Includes ------------------------------------------------------------------*/
#include "speed_pos_fdbk.h"




/* Exported types ------------------------------------------------------------*/

/** @brief Sensorless Bemf Sensing component handle type */
typedef struct Bemf_Handle Bemf_Handle_t;

typedef void ( *Bemf_ForceConvergency1_Cb_t )( Bemf_Handle_t * pHandle );
typedef void ( *Bemf_ForceConvergency2_Cb_t )( Bemf_Handle_t * pHandle );
typedef void ( *Bemf_OtfResetPLL_Cb_t )( Bemf_Handle_t * pHandle );
typedef bool ( *Bemf_SpeedReliabilityCheck_Cb_t )( const Bemf_Handle_t * pHandle );

/**
  * @brief  SpeednPosFdbk  handle definition
  */
struct Bemf_Handle
{
  SpeednPosFdbk_Handle_t     *     _Super;
  Bemf_ForceConvergency1_Cb_t       pFctForceConvergency1;
  Bemf_ForceConvergency2_Cb_t       pFctForceConvergency2;
  Bemf_OtfResetPLL_Cb_t             pFctBemfOtfResetPLL;
  Bemf_SpeedReliabilityCheck_Cb_t   pFctBemf_SpeedReliabilityCheck;
};

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

#endif /*__BEMF_SPEEDNPOSFDBK_H*/

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
