# Embedded Common

For STM32G4 microcontrollers.

- `argb.rs`: driver for WS2812 ARGB LEDs
- `can.rs`: CAN FD driver for canadensis (Cyphal)
- `clock.rs`: microsecond clock driver (consumes TIM2) for canadensis (Cyphal)
- `debug.rs`: ITM debug interface support
    - Enable with feature `dprintln-enabled`
- `stepper_bus.rs`, `tmc_registers.rs`: support for TMC5160 interface over SPI
    - Enable with feature `stepper-board`

`tools/` also has some OpenOCD config files, a GDB script and an SVD file for the G474.
