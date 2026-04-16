/* rstemp.h */

#ifndef _RSTEMP_H_
#define _RSTEMP_H_

#include "fixpmath.h"

#include <stddef.h>
#include <stdbool.h>

typedef struct _RSTEMP_Params_
{
    float_t         FullScaleCurrent_A;
    float_t         FullScaleVoltage_V;
    float_t         FullScaleFreq_Hz;
    float_t         RatedCelcius;    	/* Temperature @ Rated resistance value (deg C) */
    float_t         RsRatedOhm;
    float_t         LsHenry;
    float_t         MotorMaxCurrent_A;
    float_t         F_background_Hz;
    float_t         F_interrupt_Hz;
} RSTEMP_Params;

typedef enum _RSTEMP_State_e_
{
    RSTEMP_STATE_Idle,
    RSTEMP_STATE_Starting,
    RSTEMP_STATE_PreActive,
    RSTEMP_STATE_Active,
} RSTEMP_State_e;

typedef enum _RSTEMP_Direction_e_
{
	RSTEMP_DIRECTION_Forwards,
	RSTEMP_DIRECTION_Reverse,
} RSTEMP_Direction_e;


#define RSTEMP_SIZE_WORDS		(200)	/* Size in words, plus a margin */

typedef struct _RSTEMP_Obj_
{
	/* Contents of this structure is not public */
	uint16_t reserved[RSTEMP_SIZE_WORDS];
} RSTEMP_Obj;


typedef RSTEMP_Obj* RSTEMP_Handle;

RSTEMP_Handle RSTEMP_init(void *pMemory, const size_t size);

void RSTEMP_setParams(RSTEMP_Handle handle, const RSTEMP_Params *pParams);

void RSTEMP_run(RSTEMP_Handle handle, const Vector_ab_t *pUab, const Currents_Iab_t *pIab, const FIXP_CosSin_t *pCossinPark, const fixp30_t freq_pu);

/* Background function (~1 kHz) now spread out in mainloop */
void RSTEMP_runBackground(RSTEMP_Handle handle, const float_t active_Rs_ohm, const bool flag_OKtoGo);

void RSTEMP_setFilterStates(const RSTEMP_Handle handle, bool running);

void RSTEMP_setFilterStart(const RSTEMP_Handle handle);

/* Getters */

float_t			RSTEMP_getBandwidth(const RSTEMP_Handle handle);
bool			RSTEMP_getFlagEnable(const RSTEMP_Handle handle);
bool			RSTEMP_getFlagUpdateEnable(const RSTEMP_Handle handle);
float_t			RSTEMP_getFreqXY_Hz(const RSTEMP_Handle handle);
float_t			RSTEMP_getIref_A(const RSTEMP_Handle handle);
float			RSTEMP_getRsOhm(const RSTEMP_Handle handle);
RSTEMP_State_e	RSTEMP_getState(const RSTEMP_Handle handle);
float_t			RSTEMP_getTempCelcius(const RSTEMP_Handle handle);
fixp20_t			RSTEMP_getTempCelcius_pu(const RSTEMP_Handle handle);
Vector_ab_t		RSTEMP_getLFabDuty(const RSTEMP_Handle handle);
Vector_dq_t		RSTEMP_getIdqLFref(const RSTEMP_Handle handle);

/* Setters */
void			RSTEMP_setBandwidth(RSTEMP_Handle handle, const float_t value); /* in rad/s */
void			RSTEMP_setFlagEnable(RSTEMP_Handle handle, const bool value);
void			RSTEMP_setFreqXY_Hz(RSTEMP_Handle handle, const float_t value);
void			RSTEMP_setIref_A(RSTEMP_Handle handle, const float_t value);
void			RSTEMP_setRsOhm(RSTEMP_Handle handle, const float_t rs_ohm);
void			RSTEMP_setRsRatedOhm(RSTEMP_Handle handle, const float_t value);
void			RSTEMP_setRsToRated(RSTEMP_Handle handle);
void			RSTEMP_setTempRfactor(RSTEMP_Handle handle, const float_t value);

#endif /* _RSTEMP_H_ */

/* end of rstemp.h */
