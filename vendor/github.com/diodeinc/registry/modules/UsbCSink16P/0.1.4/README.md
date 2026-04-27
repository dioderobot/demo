# UsbCSink16P

USB Type-C sink (UFP) front-end with 16-pin connector.

For simple 5V sinks without PD negotiation. Includes connector,
CC termination, ESD protection, and VBUS clamping.

For PD sinks, use a connector (`connectors/UsbC16P`) directly
with `modules/UsbPdController` instead.
