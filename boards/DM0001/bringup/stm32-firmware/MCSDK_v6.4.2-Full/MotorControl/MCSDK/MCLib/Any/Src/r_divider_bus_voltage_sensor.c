/**
  ******************************************************************************
  * @file    r_divider_bus_voltage_sensor.c
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file provides firmware functions that implement the  features
  *          of the Resistor Divider Bus Voltage Sensor component of the Motor
  *          Control SDK:
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
  * @ingroup RDividerBusVoltageSensor
  */

/* Includes ------------------------------------------------------------------*/
#include "mc_type.h"
#include "r_divider_bus_voltage_sensor.h"
#include "regular_conversion_manager.h"


/** @addtogroup MCSDK
  * @{
  */

  /** @addtogroup ConvFOC
  * @{
  */

/** @addtogroup MCLLAPI
  * @{
  */

/** @addtogroup BusVoltageSensor
  * @{
  */

/** @defgroup RDividerBusVoltageSensor Resistor Divider Bus Voltage Sensor
  * @brief Resistor Divider Bus Voltage Sensor implementation.
  *
  * @{
  */

static uint16_t RVBS_CheckFaultState(RDivider_Handle_t *pHandle);


/**
  * @brief  It initializes bus voltage conversion (ADC, ADC channel, conversion time.
    It must be called only after PWM_Init.
  * @param  pHandle related RDivider_Handle_t
  */
__weak void RVBS_Init(RDivider_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_RDIV_BUS_VLT_SNS
  if (MC_NULL == pHandle)
  {
    /* Nothing to do */
  }
  else
  {
#endif
    pHandle->_Super.AvBusVoltage_d = (pHandle->OverVoltageThreshold + pHandle->UnderVoltageThreshold) / 2U;
#ifdef NULL_PTR_CHECK_RDIV_BUS_VLT_SNS
  }
#endif
}

/**
  * @brief  It actually performes the Vbus ADC conversion and updates averaged
  *         value for all STM32 families except STM32F3 in u16Volt format.
  * @param  pHandle related RDivider_Handle_t
  * @retval uint16_t Fault code error
  */
__weak uint16_t RVBS_CalcAvVbus(RDivider_Handle_t *pHandle, uint16_t rawValue)
{
  uint16_t tempValue = 0U;
#ifdef NULL_PTR_CHECK_RDIV_BUS_VLT_SNS
  if (MC_NULL == pHandle)
  {
    /* Nothing to do. */
  }
  else
  {
#endif
    uint16_t hAux = rawValue;

    if (0xFFFFU == hAux)
    {
      /* Nothing to do */
    }
    else
    {
      pHandle->_Super.AvBusVoltage_d += (hAux - pHandle->_Super.AvBusVoltage_d) >> 8U;
      pHandle->_Super.FaultState = RVBS_CheckFaultState(pHandle);
      tempValue = pHandle->_Super.FaultState;
    }
#ifdef NULL_PTR_CHECK_RDIV_BUS_VLT_SNS
  }
#endif
  return (tempValue);
}

/**
  * @brief  It returns #MC_OVER_VOLT, #MC_UNDER_VOLT or #MC_NO_ERROR depending on
  *         bus voltage and protection threshold values
  * @param  pHandle related RDivider_Handle_t
  * @retval uint16_t Fault code error
  */
uint16_t RVBS_CheckFaultState(RDivider_Handle_t *pHandle)
{
  uint16_t fault;
  /* If both thresholds are equal, single threshold feature is used */
  if (pHandle->OverVoltageThreshold == pHandle->OverVoltageThresholdLow)
  {
    if (pHandle->_Super.AvBusVoltage_d > pHandle->OverVoltageThreshold)
    {
      fault = MC_OVER_VOLT;
    }
    else if (pHandle->_Super.AvBusVoltage_d < pHandle->UnderVoltageThreshold)
    {
      fault = MC_UNDER_VOLT;
    }
    else
    {
      fault = MC_NO_ERROR;
    }
  }
  else
  {
    /* If both thresholds are different, hysteresis feature is used (Brake mode) */
    if (pHandle->_Super.AvBusVoltage_d < pHandle->UnderVoltageThreshold)
    {
      fault = MC_UNDER_VOLT;
    }
    else if ( false == pHandle->OverVoltageHysteresisUpDir )
    {
      if (pHandle->_Super.AvBusVoltage_d < pHandle->OverVoltageThresholdLow)
      {
        pHandle->OverVoltageHysteresisUpDir = true;
        fault = MC_NO_ERROR;
      }
      else{
        fault = MC_OVER_VOLT;
      }
    }
    else
    {
      if (pHandle->_Super.AvBusVoltage_d > pHandle->OverVoltageThreshold)
      {
        pHandle->OverVoltageHysteresisUpDir = false;
        fault = MC_OVER_VOLT;
      }
      else{
        fault = MC_NO_ERROR;
      }
    }
  }
  return (fault);
}


/**
  * @}
  */

/**
  * @}
  */

/** @} */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/

