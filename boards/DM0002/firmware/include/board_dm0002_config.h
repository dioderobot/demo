#ifndef BOARD_DM0002_CONFIG_H_
#define BOARD_DM0002_CONFIG_H_

#define PROBE_IO_RAW
#define PROBE_CDC_UART

// PIO config
#define PROBE_SM 0
#define PROBE_PIN_OFFSET 19
#define PROBE_PIN_SWCLK (PROBE_PIN_OFFSET + 0) // 19
#define PROBE_PIN_SWDIO (PROBE_PIN_OFFSET + 1) // 20

// Target reset is level-shifted and should behave as open-drain.
#define PROBE_PIN_RESET 18

// UART config
#define PROBE_UART_TX 29
#define PROBE_UART_RX 28
#define PROBE_UART_INTERFACE uart1
#define PROBE_UART_BAUDRATE 115200

// Status LEDs
#define PROBE_USB_CONNECTED_LED 17
#define PROBE_DAP_CONNECTED_LED 16
#define PROBE_DAP_RUNNING_LED 15
#define PROBE_UART_RX_LED 14
#define PROBE_UART_TX_LED 12

// USB / identification
#define PROBE_USB_CONNECTED_PIN 5
#define PROBE_MANUFACTURER_STRING "Diode Inc"

#ifndef PROBE_FW_VERSION
#define PROBE_FW_VERSION "dev"
#endif

#define PROBE_PRODUCT_STRING "Diode Probe " PROBE_FW_VERSION

#endif // BOARD_DM0002_CONFIG_H_
