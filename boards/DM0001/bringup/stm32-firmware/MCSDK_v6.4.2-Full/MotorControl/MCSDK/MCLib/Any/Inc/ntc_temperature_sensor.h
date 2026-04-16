/**
  ******************************************************************************
  * @file    ntc_temperature_sensor.h
  * @author  Motor Control SDK Team, ST Microelectronics
  * @brief   This file contains all definitions and functions prototypes for the
  *          Temperature Sensor component of the Motor Control SDK.
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
  * @ingroup TemperatureSensor
  */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef TEMPERATURESENSOR_H
#define TEMPERATURESENSOR_H

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

/* Includes ------------------------------------------------------------------*/
#include "mc_type.h"

/** @addtogroup MCSDK
  * @{
  */


/** @addtogroup COMMON_MC
  * @{
  */

/** @addtogroup COMMON_MCLIB
  * @{
  */

/** @addtogroup TemperatureSensor
  * @{
  */



/**
  * @brief Structure used for temperature monitoring
  *
  */
typedef struct
{
  SensorType_t bSensorType;         /**< Type of instanced temperature.
                                         This parameter can be REAL_SENSOR or VIRTUAL_SENSOR. */
  uint16_t hAvTemp_d;               /**< It contains latest available average Vbus.
                                         This parameter is expressed in u16Celsius. */
  uint16_t hExpectedTemp_d;         /**< Default set when no sensor available (ie virtual sensor) */

  uint16_t hExpectedTemp_C;         /**< Default value when no sensor available (ie virtual sensor).
                                         This parameter is expressed in Celsius. */
  uint16_t hFaultState;             /**< Contains latest Fault code.
                                         This parameter is set to #MC_OVER_TEMP or #MC_NO_ERROR. */
  uint16_t hOverTempThreshold;      /**< Represents the over voltage protection intervention threshold.
                                         This parameter is expressed in u16Celsius through formula:
                                         hOverTempThreshold =
                                         (V0[V]+dV/dT[V/°C]*(OverTempThreshold[°C] - T0[°C]))* 65536 /
                                          MCU supply voltage. */
  uint16_t hOverTempDeactThreshold; /**< Temperature threshold below which an active over temperature fault is cleared.
                                         This parameter is expressed in u16Celsius through formula:
                                         hOverTempDeactThreshold =
                                         (V0[V]+dV/dT[V/°C]*(OverTempDeactThresh[°C] - T0[°C]))* 65536 /
                                         MCU supply voltage. */
  int16_t hSensitivity;             /**< NTC sensitivity
                                         This parameter is equal to MCU supply voltage [V] / dV/dT [V/°C] */
  uint32_t wV0;                     /**< V0 voltage constant value used to convert the temperature into Volts.
                                         This parameter is equal V0*65536/MCU supply
                                         Used in through formula: V[V]=V0+dV/dT[V/°C]*(T-T0)[°C] */
  uint16_t hT0;                     /**< T0 temperature constant value used to convert the temperature into Volts
                                         Used in through formula: V[V]=V0+dV/dT[V/°C]*(T-T0)[°C] */

} NTC_Handle_t;

/* Initialize temperature sensing parameters */
void NTC_Init(NTC_Handle_t *pHandle);

/* Temperature sensing computation */
uint16_t NTC_CalcAvTemp(NTC_Handle_t *pHandle, uint16_t rawValue);

/* Get averaged temperature measurement expressed in Celsius degrees */
int16_t NTC_GetAvTemp_C(NTC_Handle_t *pHandle);

/**
  * @brief  Returns latest averaged temperature measured expressed in u16Celsius
  *
  * @param pHandle : Pointer on Handle structure of TemperatureSensor component
  *
  * @retval AverageTemperature : Current averaged temperature measured (in u16Celsius)
  */
static inline uint16_t NTC_GetAvTemp_d(const NTC_Handle_t *pHandle) //cstat !MISRAC2012-Rule-8.13
{
#ifdef NULL_PTR_CHECK_NTC_TEMP_SENS
  return ((MC_NULL == pHandle) ? 0U : pHandle->hAvTemp_d);
#else
  return (pHandle->hAvTemp_d);
#endif
}

/**
  * @brief  Returns Temperature measurement fault status
  *
  * Fault status can be either #MC_OVER_TEMP when measure exceeds the protection threshold values or
  * #MC_NO_ERROR if it is inside authorized range.
  *
  * @param pHandle: Pointer on Handle structure of TemperatureSensor component.
  *
  * @retval Fault status : read internal fault state
  */
static inline uint16_t NTC_CheckTemp(const NTC_Handle_t *pHandle)
{
#ifdef NULL_PTR_CHECK_NTC_TEMP_SENS
  return ((MC_NULL == pHandle) ? 0U : pHandle->hFaultState);
#else
  return (pHandle->hFaultState);
#endif
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

#endif /* TEMPERATURESENSOR_H */

/************************ (C) COPYRIGHT 2026 STMicroelectronics *****END OF FILE****/
