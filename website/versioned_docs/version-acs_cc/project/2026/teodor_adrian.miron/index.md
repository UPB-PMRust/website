# Slot Machine
A mini slot machine that accepts real coins, spins three physical reels, and dispenses a payout on win.

:::info

**Author**: Teodor Adrian Miron \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann](https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann)

:::

<!-- do not delete the \ after your name -->

## Description

My project consists in building a mini Slot Machine where you can insert coins and bet X amount of credited money. The system uses a load cell to count coins and three stepper motors to physically roll the reels. The game is initiated via a dedicated Spin button, and if the player wins, the amount is dispensed physically using a servo-driven payout mechanism.

## Motivation

I wanted to do something fun and help students control their gambling addiction with tiny amounts of money.

## Architecture

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The build uses a microcontroller board driving a small TFT display, an HX711 + load cell pair for detecting inserted coins by weight, three 28BYJ-48 stepper motors with ULN2003 drivers for the physical reels, a continuous-rotation servo for the payout mechanism, arcade buttons for player input, and a 5 V stabilized power supply to handle motor peak currents.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|-------|-------|
| [DS04-NFC Continous Rotation Servo](https://www.optimusdigital.ro/en/servomotors/1161-ds04-nfc-continous-rotation-servo.html) | Drives the coin payout mechanism | [39 RON](https://www.optimusdigital.ro/en/servomotors/1161-ds04-nfc-continous-rotation-servo.html) |
| [5 V 5000 mA Stabilized Power Supply](https://www.optimusdigital.ro/en/wall-socket-power-supplies/2890-5-v-5000-ma-stabilized-power-supply.html) | Powers the motors and the rest of the system | [30 RON](https://www.optimusdigital.ro/en/wall-socket-power-supplies/2890-5-v-5000-ma-stabilized-power-supply.html) |
| [1.44" SPI LCD Module with ST7735 Controller (128x128 px)](https://www.optimusdigital.ro/en/lcds/3552-modul-lcd-de-144-cu-spi-i-controller-st7735-128x128-px.html) | Displays credit balance, bet amount and reel animation | [30 RON](https://www.optimusdigital.ro/en/lcds/3552-modul-lcd-de-144-cu-spi-i-controller-st7735-128x128-px.html) |
| [ULN2003 Stepper Driver + 5V Stepper Motor](https://www.optimusdigital.ro/en/stepper-motors/101-stepper-motor-with-uln2003-driver.html) (x3) | Physically rolls the three reels | [17 RON](https://www.optimusdigital.ro/en/stepper-motors/101-stepper-motor-with-uln2003-driver.html) |
| [Arcade Button 24 mm - Green](https://www.optimusdigital.ro/en/buttons-and-switches/1851-buton-arcade-iluminat-24mm-verde.html) | Spin / bet input from the player | [10 RON](https://www.optimusdigital.ro/en/buttons-and-switches/1851-buton-arcade-iluminat-24mm-verde.html) |
| [HX711 GroundStudio Load Cell Amplifier](https://ardushop.ro/ro/groundstudio/2207-modul-citire-senzor-greutate-hx711-groundstudio-6427854000040.html) | Reads digital weight values from the load cell | [11 RON](https://ardushop.ro/ro/groundstudio/2207-modul-citire-senzor-greutate-hx711-groundstudio-6427854000040.html) |
| [Load Cell (max. 1 Kg)](https://ardushop.ro/ro/electronica/2418-1349-senzor-greutate.html#/246-greutate_maxima-1_kg) | Detects inserted coins by weight | [10 RON](https://ardushop.ro/ro/electronica/2418-1349-senzor-greutate.html#/246-greutate_maxima-1_kg) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 hardware driver | Controlling pins, timers (PWM for the servo) and SPI for the LCD |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time and delay management | Non-blocking delays for stepper motor steps and animations |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task scheduler | Running multiple tasks (motors, UI, sensors) concurrently |
| [embassy-sync](https://github.com/embassy-rs/embassy) | Async sync primitives | Inter-task communication (e.g. sending coin weight data to the UI) |
| [cortex-m](https://github.com/rust-embedded/cortex-m) | Core processor access | Managing interrupts and CPU-specific instructions |
| [cortex-m-rt](https://github.com/rust-embedded/cortex-m) | Startup/Runtime for ARM | Initializing memory and the program entry point |
| [defmt](https://github.com/knurling-rs/defmt) | Low-overhead logger | Fast logging for debugging sensor data and game states |
| [defmt-rtt](https://github.com/knurling-rs/defmt) | RTT transport for logs | Viewing logs in real-time through the debugger |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Drawing fruit icons, text and shapes on the screen |
| [st7735-lcd](https://crates.io/crates/st7735-lcd) | Display driver for ST7735 | Managing the command set for the 1.44" color TFT screen |
| [hx711](https://crates.io/crates/hx711) | Load cell driver | Reading digital weight values from the load cell amplifier |
| [panic-probe](https://github.com/knurling-rs/defmt) | Debug panic handler | Reporting and stopping the CPU safely if a runtime crash occurs |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Slot machine reference video](https://youtu.be/ihVHIpEZ-Pw)
