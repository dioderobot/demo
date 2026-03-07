cmake_minimum_required(VERSION 3.12)

# Fetch Pico SDK from git by default if not specified
if (NOT DEFINED PICO_SDK_PATH AND NOT DEFINED PICO_SDK_FETCH_FROM_GIT)
    set(PICO_SDK_FETCH_FROM_GIT ON)
endif()

include(pico_sdk_import.cmake)

set(FREERTOS_KERNEL_PATH ${CMAKE_CURRENT_LIST_DIR}/freertos)
include(FreeRTOS_Kernel_import.cmake)

project(dm0002_debugprobe)

pico_sdk_init()

set(DM0002_FW_VERSION "dev" CACHE STRING "DM0002 firmware version string")

if (${PICO_SDK_VERSION_MAJOR} LESS 2)
	message(SEND_ERROR "Version 2 of the Pico SDK is required to compile this project. Please update your installation at ${PICO_SDK_PATH}")
endif ()

add_executable(dm0002_debugprobe
        src/probe_config.c
        src/main.c
        src/usb_descriptors.c
        src/probe.c
        src/cdc_uart.c
        src/get_serial.c
        src/sw_dp_pio.c
        src/tusb_edpt_handler.c
)

target_sources(dm0002_debugprobe PRIVATE
        CMSIS_DAP/CMSIS/DAP/Firmware/Source/DAP.c
        CMSIS_DAP/CMSIS/DAP/Firmware/Source/JTAG_DP.c
        CMSIS_DAP/CMSIS/DAP/Firmware/Source/DAP_vendor.c
        CMSIS_DAP/CMSIS/DAP/Firmware/Source/SWO.c
        #CMSIS_DAP/CMSIS/DAP/Firmware/Source/SW_DP.c
        )

target_include_directories(dm0002_debugprobe PRIVATE
        CMSIS_DAP/CMSIS/DAP/Firmware/Include/
        CMSIS_DAP/CMSIS/Core/Include/
        include/
        )

target_compile_options(dm0002_debugprobe PRIVATE -Wall)

pico_generate_pio_header(dm0002_debugprobe ${CMAKE_CURRENT_LIST_DIR}/src/probe.pio)
pico_generate_pio_header(dm0002_debugprobe ${CMAKE_CURRENT_LIST_DIR}/src/probe_oen.pio)

target_include_directories(dm0002_debugprobe PRIVATE src)

target_compile_definitions (dm0002_debugprobe PRIVATE
	PICO_RP2040_USB_DEVICE_ENUMERATION_FIX=1
    PROBE_FW_VERSION=\"${DM0002_FW_VERSION}\"
)

set_target_properties(dm0002_debugprobe PROPERTIES
    OUTPUT_NAME "dm0002-debugprobe"
)

option (DEBUG_ON_PICO "Compile firmware for the Pico instead of Debug Probe" OFF)
if (DEBUG_ON_PICO)
    target_compile_definitions (dm0002_debugprobe PRIVATE
	DEBUG_ON_PICO=1
    )
    if (PICO_BOARD STREQUAL "pico")
        set_target_properties(dm0002_debugprobe PROPERTIES
            OUTPUT_NAME "debugprobe_on_pico"
        )
    elseif (PICO_BOARD STREQUAL "pico2")
        set_target_properties(dm0002_debugprobe PROPERTIES
            OUTPUT_NAME "debugprobe_on_pico2"
        )
    else ()
        message(SEND_ERROR "Unsupported board ${PICO_BOARD}")
    endif ()
endif ()


target_link_libraries(dm0002_debugprobe PRIVATE
        pico_multicore
        pico_stdlib
        pico_unique_id
        tinyusb_device
        tinyusb_board
        hardware_pio
        FreeRTOS-Kernel
        FreeRTOS-Kernel-Heap1
)

pico_set_binary_type(dm0002_debugprobe copy_to_ram)

pico_add_extra_outputs(dm0002_debugprobe)
