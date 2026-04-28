# project name

Slot Machine

:::info

**author**: Teodor Adrian Miron \
**github project link**: [https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann](https://github.com/UPB-PMRust-Students/acs-project-2026-teodoradriann)

:::

## description

My project consists in building a mini Slot Machine where you can insert coins and bet X amount of credited money. The system uses a load cell to count coins and three stepper motors to physically roll the reels. The game is initiated via a dedicated **Spin button**, and if the player wins, the amount is dispensed physically using a servo-driven payout mechanism.

## motivation

I wanted to do something fun and help students control their gambling addiction with tiny amounts of money.

## architecture

The software architecture is based on a Finite State Machine (FSM) managing the game flow, implemented in Rust using the Embassy framework for async multitasking.

### 1. Main Components

- **Core Logic / State Machine:** Manages game states (e.g., `IDLE`, `WAITING_FOR_BET`, `SPINNING`, `PAYOUT`) and coordinates all modules.
- **Input / Sensor Manager:** Handles debouncing for the **Spin and Bet buttons** and interprets weight signals from the load cell.
- **Random Number Generator (RNG):** Responsible for the fair generation of the spin outcome.
- **Motor / Actuator Controller:** Translates the RNG result into steps for the 3 stepper motors and PWM signals for the payout servo.
- **Display / UI Controller:** Manages the ST7735 LCD, displaying credit balance and bet amount.
- **Payout Manager:** Commands the servo motor to eject the coins.

### 2. How they connect with each other

- **Data Collection:** The `Input Manager` detects a coin (weight increase) or a button press. It sends an event to the `Core Logic`.
- **Processing:** `Core Logic` updates credit and instructs the `Display Controller` to refresh.
- **Triggering the Spin:** When the **Spin button** is pressed, `Core Logic` switches to `SPINNING` state and requests a value from the `RNG`.
- **Animation and Result:** The `Motor Controller` starts the 3 stepper motors and stops them sequentially based on the `RNG` result.
- **Payout:** If a win is detected, the `Payout Manager` activates the MG995 servo to slide coins out of the storage tube.

## log

### week 5 - 11 may

### week 12 - 18 may

### week 19 - 25 may

## hardware

### 1. Electronics & Control

- **Microcontroller:** Nucleo STM32 Development Board.
- **Display:** 1.44" Color TFT LCD (ST7735 driver, 128x128 px) using SPI.
- **Weight Sensor:** 1kg Load Cell + HX711 amplifier for 50 Bani coin detection.
- **Sound:** 5V Passive Buzzer for 8-bit game effects.
- **Input:**
  - **Buttons:** Tactile buttons to adjust the wager and spin button.
- **Actuators:**
  - **Stepper Motors (x3):** 28BYJ-48 + ULN2003 drivers for the physical reels.
  - **MG995 Servo/SG90:** Servo for the payout ejector.
- **Power Supply:** 5V 3A external DC power supply to handle motor peak currents.

### bill of materials

| Component | Description | Price / Link |
|---|---|---|
| [STM32U545](https://www.st.com/resource/en/user_manual/um3062-stm32u3u5-nucleo64-boards-mb1841-stmicroelectronics.pdf) | The microcontroller | [113 RON](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q) |
| [LCD SCREEN ST7735] | the LCD screen | [30 RON](https://www.optimusdigital.ro/3552-product.html) |
| [Load cell 1KG] | the scale sensor | [10 RON](https://ardushop.ro/ro/electronica/2418-1349-senzor-greutate.html) |
| [HX711] | the module used for the scale sensor | [10 RON](https://ardushop.ro/ro/groundstudio/2207-modul-citire-senzor-greutate-hx711-groundstudio-6427854000040.html) |
| [BUZZER] | used for simulating sounds of slots machines | [0 RON](i already have it) |
| [BUTTONS] | used for changing the bet amount and spin | [0 RON](i already have them) |
| [3 x stepper motors 28BYJ-48] | used to mechanically roll the fruits | [51 RON](https://www.optimusdigital.ro/en/stepper-motors/101-stepper-motor-with-uln2003-driver.html) |
| [Servomotor MG995] | used to push the coins out | [26 RON](https://ardushop.ro/ro/motoare-si-drivere/634-servomotor-mg995-semi-metal-6427854007889.html) |
| [Servomotor SG90] | lower power servo used to push the coins out | [14 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html) |
| [DC in adaptor] | used to power the mg995 | [1.5 RON](https://www.optimusdigital.ro/en/connectors/12499-dc-jack-connector-female-21mm-x-55mm.html) |
| [Power supply 5V 3A] | used to power the mg995 | [? RON](nu gasesc niciuna potrivita la momentul actual) |

## software

| Library | Description | Usage |
|---------|-------------|-------|
| **embassy-stm32** | STM32 hardware driver | Controlling pins, timers (PWM for servos), and SPI for the LCD |
| **embassy-time** | Time and delay management | Handling non-blocking delays for stepper motor steps and animations |
| **embassy-executor** | Async task scheduler | Running multiple tasks (motors, UI, sensors) concurrently |
| **embassy-sync** | Async sync primitives | Inter-task communication (e.g., sending coin weight data to the UI) |
| **cortex-m** | Core processor access | Managing interrupts and CPU-specific instructions |
| **cortex-m-rt** | Startup/Runtime for ARM | Initializing memory and the program entry point |
| **defmt** | Low-overhead logger | Fast logging for debugging sensor data and game states |
| **defmt-rtt** | RTT transport for logs | Viewing logs in real-time through the debugger |
| **embedded-graphics** | 2D graphics library | Core library for drawing fruit icons, text, and shapes on the screen |
| **st7735-lcd** | Display driver for ST7735 | Managing the specific command set for the 1.44" color TFT screen |
| **hx711** | Load cell driver | Reading digital weight values from the 50 Bani coin scale amplifier |
| **panic-probe** | Debug panic handler | Reporting and stopping the CPU safely if a runtime crash occurs |

## links

[Video Link](https://youtu.be/ihVHIpEZ-Pw?is=S4FJ2DUwAJGDrWJH)
