
/**
  ******************************************************************************
  * @file    regular_conversion_manager.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides firmware functions that implement the following features
  *          of the regular_conversion_manager component of the Motor Control SDK:
  *           Register conversion
  *           Execute regular conv directly from Temperature and VBus sensors
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
  */

/* Includes ------------------------------------------------------------------*/
#include "regular_conversion_manager.h"
#include "mc_config.h"

/** @addtogroup MCSDK
  * @{
  */

/** @addtogroup COMMON_MC
  * @{
  */

/** @defgroup RCM Regular Conversion Manager
  * @brief Regular Conversion Manager component of the Motor Control SDK
  *
  * MotorControl SDK makes an extensive usage of ADCs. Some conversions are timing critical
  * like current reading, and some have less constraints. If an ADC offers both Injected and Regular,
  * channels, critical conversions will be systematically done on Injected channels, because they
  * interrupt any ongoing regular conversion so as to be executed without delay.
  * Others conversions, mainly Bus voltage, and Temperature sensing are performed with regular channels.
  * If users wants to perform ADC conversions with an ADC already used by MC SDK, they must use regular
  * conversions. It is forbidden to use Injected channel on an ADC that is already in use for current reading.
  * As usera and MC-SDK may share ADC regular scheduler, this component intents to manage all the
  * regular conversions.
  *
  * If users wants to execute their own conversion, they first have to register it through the
  * RCM_RegisterRegConv() API. Multiple conversions can be registered in the RCM array (up to #RCM_MAX_CONV),
  * which is processed in a circular way but only one can be scheduled at a time.
  *
  * User regular conversion, as well as Vbus and temperature, are executed by the high frequency task. Each high
  * frequency task executes one of the conversion registered in the RCM array.
  *
  * To retrieve the result of a conversion the user must use  RCM_GetRegularConv() API.
  *
  * Example: of conversion registration:
  *
  * RegConv_t UserConv =
  * {
  *  .regADC                = ADC1,
  *  .channel               = LL_ADC_CHANNEL_0,
  *  .samplingTime          = LL_ADC_SAMPLINGTIME_6CYCLES_5,
  * };
  *
  * result = RCM_RegisterRegConv(&UserConv);
  *
  * Example of conversion reading:
  *
  * result = RCM_GetRegularConv(&UserConv);
  *
  * @note: conversions must registered before the enabling of the Irq that triggers the high frequency task
  *        or speed timer task
  *
  * @{
  */

/* Private typedef -----------------------------------------------------------*/

/* Private defines -----------------------------------------------------------*/
/**
  * @brief Number of regular conversion allowed By default.
  *
  * In single drive configuration, it is defined to 4. 2 of them are consumed by
  * Bus voltage and temperature reading. This leaves 2 handles available for
  * user conversions
  *
  * In dual drives configuration, it is defined to 6. 2 of them are consumed by
  * Bus voltage and temperature reading for each motor. This leaves 2 handles
  * available for user conversion.
  *
  * Defined to 4 here.
  */
#define RCM_MAX_CONV  4U

/* Global variables ----------------------------------------------------------*/

static RegConv_t *RCM_handle_array[RCM_MAX_CONV];
static uint8_t RCM_array_index = 0U; /*!< handled by RCM to point on the element for conversion. */
static uint8_t RCM_conversion_nb = 0U; /*!< total number of valid element in the array */

/* Private function prototypes -----------------------------------------------*/

/* Private functions ---------------------------------------------------------*/

/**
  * @brief  Registers a regular conversion.
  *
  * This function registers a regular ADC conversion that can be later scheduled for execution. It
  * returns the status of the registration.
  *
  * The registration may fail if there is no space left for additional conversions. The
  * maximum number of regular conversion that can be registered is defined by #RCM_MAX_CONV.
  *
  * @note if HSO is used as sensor-less algortihm, the registration shall be done with an ADC
  *       not used for phase current and phase voltage sensing.
  *
  * @param  regConv Pointer to the regular conversion parameters.
  *         Contains ADC, Channel and sampling time to be used.
  *
  * @retval bool true if the conversion is registered correctly, false otherwise still ongoing.
  *
  */
bool RCM_RegisterRegConv(RegConv_t *regConv)
{
  bool retVal = true;
#ifdef NULL_PTR_CHECK_REG_CON_MNG
  if (MC_NULL == regConv)
  {
    retVal = false;
  }
  else
  {
#endif

    if (RCM_conversion_nb < RCM_MAX_CONV)
    {
      RCM_handle_array[RCM_conversion_nb] = regConv;
      RCM_handle_array[RCM_conversion_nb]->id = RCM_conversion_nb;
      RCM_conversion_nb++;

      if (0U == LL_ADC_IsEnabled(regConv->regADC))
      {
        LL_ADC_DisableIT_EOC(regConv->regADC);
        LL_ADC_ClearFlag_EOC(regConv->regADC);
        LL_ADC_DisableIT_JEOC(regConv->regADC);
        LL_ADC_ClearFlag_JEOC(regConv->regADC);

        LL_ADC_StartCalibration(regConv->regADC, LL_ADC_SINGLE_ENDED);
        while (1U == LL_ADC_IsCalibrationOnGoing(regConv->regADC))
        {
          /* Nothing to do */
        }
        /* ADC Enable (must be done after calibration) */
        /* ADC5-140924: Enabling the ADC by setting ADEN bit soon after polling ADCAL=0
        * following a calibration phase, could have no effect on ADC
        * within certain AHB/ADC clock ratio
        */
        while (0U == LL_ADC_IsActiveFlag_ADRDY(regConv->regADC))
        {
          LL_ADC_Enable(regConv->regADC);
        }

      }
      else
      {
        /* Nothing to do */
      }
      LL_ADC_REG_SetSequencerLength(regConv->regADC, LL_ADC_REG_SEQ_SCAN_DISABLE);
      /* Configure the sampling time (should already be configured by for non user conversions) */
      LL_ADC_SetChannelSamplingTime (regConv->regADC, __LL_ADC_DECIMAL_NB_TO_CHANNEL(regConv->channel),
                                     regConv->samplingTime);
    }
    else
    {
      retVal = false;
    }
#ifdef NULL_PTR_CHECK_REG_CON_MNG
  }
#endif
  return retVal;
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section (".ccmram")))
#endif
#endif
/*
 * Starts the next scheduled regular conversion
 *
 * This function does not poll on ADC read and is foreseen to be used inside
 * high frequency task where ADC are shared between currents reading
 * and user conversion.
 *
 * @note: This function is not part of the public API and users should not call it.
 */
void RCM_ExecNextConv(void)
{
  if (RCM_conversion_nb > 0u)
  {

    LL_ADC_REG_SetSequencerRanks(RCM_handle_array[RCM_array_index]->regADC,
                                 LL_ADC_REG_RANK_1,
                                 __LL_ADC_DECIMAL_NB_TO_CHANNEL(RCM_handle_array[RCM_array_index]->channel));

    (void)LL_ADC_REG_ReadConversionData12L(RCM_handle_array[RCM_array_index]->regADC);

    LL_ADC_ClearFlag_EOC(RCM_handle_array[RCM_array_index]->regADC);
    LL_ADC_REG_StartConversion(RCM_handle_array[RCM_array_index]->regADC);

  }
  else
  {
     /* no conversion registered */
  }
}

#if defined (CCMRAM)
#if defined (__ICCARM__)
#pragma location = ".ccmram"
#elif defined (__CC_ARM) || defined(__GNUC__)
__attribute__((section (".ccmram")))
#endif
#endif
/*
 * Reads the result of the ongoing regular conversion
 *
 * This function is foreseen to be used inside
 * high frequency task where ADC are shared between current reading
 * and user conversion.
 *
 * @note: This function is not part of the public API and users should not call it.
 */
void RCM_ReadOngoingConv(void)
{
  uint32_t result;

  if (RCM_conversion_nb > 0u)
  {
    result = LL_ADC_IsActiveFlag_EOC(RCM_handle_array[RCM_array_index]->regADC);
    if ( 0U == result )
    {
      /* Nothing to do */
    }
    else
    {
      /* Reading of ADC Converted Value */
      RCM_handle_array[RCM_array_index]->data
                    = LL_ADC_REG_ReadConversionData12L(RCM_handle_array[RCM_array_index]->regADC);
    }

    /* Prepare next conversion */
    if (RCM_array_index == (RCM_conversion_nb - 1U))
    {
      RCM_array_index = 0U;
    }
    else
    {
      RCM_array_index++;
    }
  }
  else
  {
     /* no conversion registered */
  }
}

/*
 * This function is used to exeute a regular conversion.
 * This function polls on the ADC end of conversion.
 * If the ADC is already in use for phase currents or phase voltage sensing, the regular conversion can not
 * be executed instantaneously, therefore this function shall not be used.
 * If it is possible to execute the conversion instantaneously, it will be executed, and result returned.
 *
 * @note: This function is not part of the public API and users should not call it.
 */
uint16_t RCM_ExecRegularConv(RegConv_t *regConv)
{
  uint16_t result;
#ifdef NULL_PTR_CHECK_REG_CON_MNG
  if (MC_NULL == regConv)
  {
    result = 0U;
  }
  else
  {
#endif

  LL_ADC_REG_SetSequencerRanks(regConv->regADC,
                               LL_ADC_REG_RANK_1,
                               __LL_ADC_DECIMAL_NB_TO_CHANNEL(regConv->channel));

  (void)LL_ADC_REG_ReadConversionData12L(regConv->regADC);

  LL_ADC_ClearFlag_EOC(regConv->regADC);
  LL_ADC_REG_StartConversion(regConv->regADC);

  while (LL_ADC_IsActiveFlag_EOC(regConv->regADC) == 0U )

  {
    /* wait for end of conversion */
  }

  /* Reading of ADC Converted Value */
  result = LL_ADC_REG_ReadConversionData12L(regConv->regADC);
#ifdef NULL_PTR_CHECK_REG_CON_MNG
  }
#endif
  return result;

}

/*
 * This function is used to wait for the result of a regular conversion.
 * @note: This shall be used only right after a call to RCM_ExecNextConv routine.
 *
 */
void RCM_WaitForConv(void)
{
  if (RCM_conversion_nb > 0u)
  {
    while (LL_ADC_IsActiveFlag_EOC(RCM_handle_array[RCM_array_index]->regADC) == 0U )
    {
      /* wait for end of conversion */
    }
  }
  else
  {
     /* no conversion registered */
  }
}

/**
  * @}
  */

/**
  * @}
  */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/

