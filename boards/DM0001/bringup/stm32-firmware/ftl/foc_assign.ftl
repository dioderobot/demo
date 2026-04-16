<#if MC.M1_CS_OPAMP_PHASE_SHARED!="NONE"><#assign M1_PHASE_SHARED = MC.M1_CS_OPAMP_PHASE_SHARED><#else><#assign M1_PHASE_SHARED = MC.M1_CS_ADC_PHASE_SHARED></#if>
<#if MC.M2_CS_OPAMP_PHASE_SHARED!="NONE"><#assign M2_PHASE_SHARED = MC.M2_CS_OPAMP_PHASE_SHARED><#else><#assign M2_PHASE_SHARED = MC.M2_CS_ADC_PHASE_SHARED></#if>

<#if CondFamily_STM32C0 == true>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_2">
<#assign R1_DMA_CH_ADC_M1 = "LL_DMA_CHANNEL_1">
</#if><#-- CondFamily_STM32C0 == true -->

<#if CondFamily_STM32F0 == true>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_5">
<#assign R1_DMA_CH_AUX_M1 = "LL_DMA_CHANNEL_4">
<#assign R1_DMA_CH_ADC_M1 = "LL_DMA_CHANNEL_1">
</#if><#-- CondFamily_STM32F0 == true -->

<#if CondFamily_STM32F3>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M1_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->
  <#if MC.M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M2_PHASE_SHARED!="V"><#assign M2_ADC = MC.M2_CS_ADC_V><#else><#assign M2_ADC = MC.M2_CS_ADC_U></#if>
  <#else><#-- (M2_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M2_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M2_ADC = MC.M2_CS_ADC_U>
  </#if><#-- M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->

  <#if MC.M1_PWM_TIMER_SELECTION == "PWM_TIM1">
<#assign DMA_nb   = "1">
<#assign DMA_channel   = "4"> 
  </#if>
  <#if MC.M1_PWM_TIMER_SELECTION == "PWM_TIM8">
<#assign DMA_nb   = "2">
<#assign DMA_channel   = "2">   
  </#if>
  <#if MC.M2_PWM_TIMER_SELECTION == "PWM_TIM1">
<#assign DMA_nb2   = "1">
<#assign DMA_channel2   = "4"> 
  </#if>
  <#if MC.M2_PWM_TIMER_SELECTION == "PWM_TIM8">
<#assign DMA_nb2   = "2">
<#assign DMA_channel2   = "2">   
  </#if>

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_4">
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_2">
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32F3 -->

<#if CondFamily_STM32F4 == true>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if MC.M1_CS_ADC_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->
  <#if MC.M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M2_PHASE_SHARED!="V"><#assign M2_ADC = MC.M2_CS_ADC_V><#else><#assign M2_ADC = MC.M2_CS_ADC_U></#if>
  <#else><#-- (M2_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M2_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M2_ADC = MC.M2_CS_ADC_U>
  </#if><#-- M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->
<#function _last_word text sep="_"><#return text?split(sep)?last></#function>

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_STREAM_5">
<#assign R1_DMA_CH_AUX_M1 = "LL_DMA_STREAM_4">
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_STREAM_1">
<#assign R1_DMA_CH_AUX_M1 = "LL_DMA_STREAM_7">
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32F4 == true -->

<#if CondFamily_STM32F7>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if MC.M1_CS_ADC_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->
<#function _last_word text sep="_"><#return text?split(sep)?last></#function>

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1'>
<#assign TIMx     = "TIM1">
<#assign Stream   = "4">
<#assign DMAIrq   = "DMA2_Stream4_IRQHandler">
  </#if>
  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8'>
<#assign TIMx     = "TIM8">
<#assign Stream   = "7">
<#assign DMAIrq   = "DMA2_Stream7_IRQHandler">
  </#if>

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_STREAM_4">
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_STREAM_7">
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32F7 -->

<#if CondFamily_STM32G0 == true>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_4">
<#assign R1_DMA_CH_ADC_M1 = "LL_DMA_CHANNEL_1">
</#if><#-- CondFamily_STM32G0 == true -->

<#if CondFamily_STM32G4>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M1_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->
  <#if MC.M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M2_PHASE_SHARED!="V"><#assign M2_ADC = MC.M2_CS_ADC_V><#else><#assign M2_ADC = MC.M2_CS_ADC_U></#if>
  <#else><#-- (M2_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M2_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M2_ADC = MC.M2_CS_ADC_U>
  </#if><#-- M2_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_1">
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_1">
  </#if>

   <#if MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M2 = "DMA1">
<#assign R1_DMA_CH_M2 = "LL_DMA_CHANNEL_1">
   <#elseif MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M2_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M2 = "DMA2">
<#assign R1_DMA_CH_M2 = "LL_DMA_CHANNEL_1">
   </#if><#-- MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32G4 -->

<#if CondFamily_STM32H5 == true>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if MC.M1_CS_ADC_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "GPDMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_0"> 
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "GPDMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_1"> 
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32H5 == true -->

<#if CondFamily_STM32H7>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if M1_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->

  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1'>
<#assign TIMx     = "TIM1">
<#assign Stream   = "4">
<#assign DMAIrq   = "DMA2_Stream4_IRQHandler">
  </#if>
  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8'>
<#assign TIMx     = "TIM8">
<#assign Stream   = "7">
<#assign DMAIrq   = "DMA2_Stream7_IRQHandler">
  </#if>
</#if><#-- CondFamily_STM32H7 -->

<#if CondFamily_STM32L4>
  <#if MC.M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT'> 
    <#if MC.M1_CS_ADC_PHASE_SHARED!="V"><#assign M1_ADC = MC.M1_CS_ADC_V><#else><#assign M1_ADC = MC.M1_CS_ADC_U></#if>
  <#else><#-- (M1_CURRENT_SENSING_TOPO == 'SINGLE_SHUNT') || (M1_CURRENT_SENSING_TOPO == 'ICS_SENSORS') -->
<#assign M1_ADC = MC.M1_CS_ADC_U>
  </#if><#-- M1_CURRENT_SENSING_TOPO == 'THREE_SHUNT' -->

<#assign R1_DMA_STATUS_REG_M1 = "ISR">
<#assign R1_DMA_CLEAR_REG_M1 = "IFCR">
  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "DMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_4">
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "DMA2">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_2">
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32L4 -->

<#if CondFamily_STM32C0 || CondFamily_STM32G0 || CondFamily_STM32F3 || CondFamily_STM32L4 || CondFamily_STM32F0 || CondFamily_STM32G4 || CondFamily_STM32H5 || CondFamily_STM32U5>
<#assign currentFactor = "">
<#elseif CondFamily_STM32F4 || CondFamily_STM32F7>
<#assign currentFactor = "*2">
</#if>
<#if CondFamily_STM32F3 || CondFamily_STM32L4 || CondFamily_STM32F4 || CondFamily_STM32F7 || CondFamily_STM32G4 || CondFamily_STM32H5 || CondFamily_STM32U5>
<#assign HAS_ADC_INJ = true>
<#else>
<#assign HAS_ADC_INJ = false>
</#if><#-- CondFamily_STM32F3 || CondFamily_STM32L4 || CondFamily_STM32F4 ||  CondFamily_STM32F7
          || CondFamily_STM32G4 || CondFamily_STM32U5-->
<#if CondFamily_STM32F4 || CondFamily_STM32F7>
<#assign DMA_TYPE = "Stream">
<#else><#-- CondFamily_STM32F4 == false && CondFamily_STM32F7 == false -->
<#assign DMA_TYPE = "Channel">
</#if><#-- CondFamily_STM32F4 || CondFamily_STM32F7 -->
<#if CondFamily_STM32F3 || CondFamily_STM32L4 || CondFamily_STM32G4 || CondFamily_STM32F7 || CondFamily_STM32H5>
<#assign HAS_BRK2 = true>
<#else><#-- CondFamily_STM32F3 == false && CondFamily_STM32L4 == false && CondFamily_STM32G4 == false
         && CondFamily_STM32F7 == false -->
<#assign HAS_BRK2 = false>
</#if><#-- CondFamily_STM32F3 || CondFamily_STM32L4 || CondFamily_STM32G4 || CondFamily_STM32F7 -->

<#if MC.DRIVE_NUMBER != "1">
  <#if CondFamily_STM32F3 == true>
    <#if MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M2 = "DMA1">
<#assign R1_DMA_CH_M2 = "LL_DMA_CHANNEL_5">
    <#elseif MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M2_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M2 = "DMA2">
<#assign R1_DMA_CH_M2 = "LL_DMA_CHANNEL_2">
     </#if><#-- MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1' -->
  </#if><#-- CondFamily_STM32F3 == true -->
  
  <#if CondFamily_STM32F4 == true>
    <#if MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M2 = "DMA2">
<#assign R1_DMA_CH_M2 = "LL_DMA_STREAM_5">
<#assign R1_DMA_CH_AUX_M2 = "LL_DMA_STREAM_4">
    <#elseif MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M2_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M2 = "DMA2">
<#assign R1_DMA_CH_M2 = "LL_DMA_STREAM_1">
<#assign R1_DMA_CH_AUX_M2 = "LL_DMA_STREAM_7">
    </#if><#-- MC.M2_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M2_PWM_TIMER_SELECTION == 'TIM1' -->
  </#if><#-- CondFamily_STM32F4 == true -->
</#if><#-- MC.DRIVE_NUMBER > 1 -->

<#if CondFamily_STM32U5 == true>
<#assign M1_ADC = MC.M1_CS_ADC_U>
  <#if MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1'>
<#assign R1_DMA_M1 = "GPDMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_0"> 
  <#elseif MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM8' || MC.M1_PWM_TIMER_SELECTION == 'TIM8'>
<#assign R1_DMA_M1 = "GPDMA1">
<#assign R1_DMA_CH_M1 = "LL_DMA_CHANNEL_1"> 
  </#if><#-- MC.M1_PWM_TIMER_SELECTION == 'PWM_TIM1' || MC.M1_PWM_TIMER_SELECTION == 'TIM1' -->
</#if><#-- CondFamily_STM32U5 == true -->

<#if (MC.DRIVE_NUMBER != "1")>
  <#if (MC.ADC_PERIPH == MC.ADC_PERIPH2) >
    <#-- both motors shares same ADC -->
    <#assign HIGH_FREQ_TRIGGER = "TIM_UP_IT">
  <#else>
    <#-- check corner case. Ex dual three shunt 2ADC/1ADC, motor 1 uses ADCx and ADCy and motor 2 uses ADCy -->
    <#-- but WB sets MC.ADC_PERIPH = ADCx and MC.ADC_PERIPH2 = ADCy => false independent resources --> 
    <#if (MC.M1_CS_ADC_U == MC.ADC_PERIPH2) || (MC.M1_CS_ADC_V == MC.ADC_PERIPH2) || (MC.M1_CS_ADC_W == MC.ADC_PERIPH2)>
      <#assign HIGH_FREQ_TRIGGER = "TIM_UP_IT">
    <#else> 
    <#-- true independent resources --> 
      <#assign HIGH_FREQ_TRIGGER = "ADC_IT">
    </#if>
  </#if> 
<#else>
  <#assign HIGH_FREQ_TRIGGER = "TIM_UP_IT">
</#if>

