/**
  ******************************************************************************
  * @file    mc_app_hooks.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file implements default motor control app hooks.
  *
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; Copyright (c) 2023 STMicroelectronics.
  * All rights reserved.</center></h2>
  *
  * This software component is licensed by ST under Ultimate Liberty license
  * SLA0044, the "License"; You may not use this file except in compliance with
  * the License. You may obtain a copy of the License at:
  *                             www.st.com/SLA0044
  *
  ******************************************************************************
  * @ingroup MCAppHooks
  */

/* Includes ------------------------------------------------------------------*/
#include "mc_type.h"
#include "mc_app_hooks.h"
#include "mc_api.h"
#include "mc_config.h"

#define AUTORUN_START_DELAY_TICKS 300U
#define AUTORUN_RETRY_DELAY_TICKS 500U
#define AUTORUN_TARGET_SPEED_RPM  3800.0f
#define AUTORUN_RAMP_MS           3400U

static uint16_t s_delay_ticks = AUTORUN_START_DELAY_TICKS;
static uint16_t s_refresh_ticks = 0U;

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup MCTasks
  * @{
  */

/**
 * @defgroup MCAppHooks Motor Control Applicative hooks
 * @brief User defined functions that are called in the Motor Control tasks.
 *
 *
 * @{
 */

/**
 * @brief Hook function called right before the end of the MCboot function.
 *
 *
 *
 */
__weak void MC_APP_BootHook(void)
{
/* USER CODE BEGIN BootHook */
  s_delay_ticks = AUTORUN_START_DELAY_TICKS;
  s_refresh_ticks = 0U;

/* USER CODE END BootHook */
}

/**
 * @brief Hook function called right after the Medium Frequency Task for Motor 1.
 *
 *
 *
 */
__weak void MC_APP_PostMediumFrequencyHook_M1(void)
{
/* USER SECTION BEGIN PostMediumFrequencyHookM1 */
  MCI_State_t state = MC_GetSTMStateMotor1();

  if ((state == FAULT_NOW) || (state == FAULT_OVER))
  {
    (void)MC_AcknowledgeFaultMotor1();
    s_delay_ticks = AUTORUN_RETRY_DELAY_TICKS;
    s_refresh_ticks = 0U;
    return;
  }

  if (s_delay_ticks > 0U)
  {
    s_delay_ticks--;
    return;
  }

  if (state == IDLE)
  {
    MC_ProgramSpeedRampMotor1_F(AUTORUN_TARGET_SPEED_RPM, AUTORUN_RAMP_MS);
    (void)MC_StartMotor1();
    s_delay_ticks = AUTORUN_RETRY_DELAY_TICKS;
    s_refresh_ticks = 0U;
    return;
  }

  if (state == RUN)
  {
    if (s_refresh_ticks == 0U)
    {
      MC_ProgramSpeedRampMotor1_F(AUTORUN_TARGET_SPEED_RPM, 200U);
      s_refresh_ticks = 50U;
    }
    else
    {
      s_refresh_ticks--;
    }
    s_delay_ticks = 0U;
  }

/* USER SECTION END PostMediumFrequencyHookM1 */
}

/** @} */

/** @} */

/** @} */

/************************ (C) COPYRIGHT 2023 STMicroelectronics *****END OF FILE****/
