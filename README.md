# Embedded Common

For STM32G4 microcontrollers.

Your project must use Bluesat's `stm32g4xx_hal` for this crate to work. You may
also need to make sure the versions of some crates (e.g. canadensis) match
those used by this crate.

Specify the chip type as a feature. Options: `stm32g431`, `stm32g474`.

- `argb.rs`: driver for WS2812 ARGB LEDs
- `uuid.rs`: support for getting a 128-bit ID
- `can.rs`: CAN FD driver for canadensis (Cyphal)
    - Enable with feature `can-fd`
- `clock.rs`: microsecond clock driver (consumes TIM2) for canadensis (Cyphal)
- `debug.rs`: ITM debug interface support
    - Enable with feature `dprintln`
- `tmc_registers.rs`: support for TMC5160 interface
    - Enable with feature `stepper-board`

`tools/` also has some OpenOCD config files, a GDB script and an SVD file for the G474.
