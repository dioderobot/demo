#ifndef BOARD_DIODE_PROBE_CONFIG_H_
#define BOARD_DIODE_PROBE_CONFIG_H_

#define PROBE_IO_RAW
#define PROBE_CDC_UART

// PIO config
#define PROBE_SM 0
#define PROBE_PIN_OFFSET 19
#define PROBE_PIN_SWCLK (PROBE_PIN_OFFSET + 0) // 19
#define PROBE_PIN_SWDIO (PROBE_PIN_OFFSET + 1) // 20
// Target reset config
#define PROBE_PIN_RESET 18

// UART config
#define PROBE_UART_TX 29
#define PROBE_UART_RX 28
#define PROBE_UART_INTERFACE uart1
#define PROBE_UART_BAUDRATE 115200

// VBUS detection for USB connection status
#define PROBE_USB_CONNECTED_PIN 13

// Status LEDs
#define PROBE_USB_CONNECTED_LED 17  // Blue LED
#define PROBE_DAP_CONNECTED_LED 16  // Green LED  
#define PROBE_DAP_RUNNING_LED 15    // Orange LED 1
#define PROBE_UART_RX_LED 14        // Orange LED 2
#define PROBE_UART_TX_LED 12        // Orange LED 3

#define PROBE_PRODUCT_STRING "Diode Debug Probe (CMSIS-DAP)"

#endif // BOARD_DIODE_PROBE_CONFIG_H_
