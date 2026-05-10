# Vending Machine System

A Rust-powered vending machine implemneted on an STM32 microcontroller that handles payments and dispenses products automatically

:::info

**Author**: Rusu Anna Cristina \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-annacristinarusu

:::

## Description

This automated vending machine system is built on a NUCLEO-U545RE-Q microcontroller and programmed in Rust. Allows users to browse and select different products through a digital interface that provides continuous communication and feedback throughout the process. To handle transactions, the machine supports two payment methods: coin insertion and card payment. In both cases, it verifies if the funds satisfy the specific price requirements of the selected item. Once the transaction conditions are met, the system automatically activates a dispensing mechanism to release the product to the user.

## Motivation

Vending machines are everywhere, yet few people think about the technology behind them. I was motivated by the challenge of simulating a real-world system from scratch, understanding every detail of how it works, from detecting a coin insertion to activating a motor to dispense a product.

## Architecture

```
                         +------------------+
                         |   Host PC (USB)  |
                         +--------+---------+
                                  |
                                  | USB (Power + Debug)
                                  |
              +-------------------v-------------------+
              |           NUCLEO-U545RE-Q             |
              |         (Main Microcontroller)        |
              +----+--------+--------+--------+-------+
                   |        |        |        |
                 [SPI]    [I2C]   [GPIO]    [PWM]
                   |        |        |        |
                   v        v        v        v
            +-------+  +-------+  +-------+  +----------+
            | RFID  |  | I2C   |  | 4x4   |  | 4x SG90  |
            | RC522 |  | Mod + |  | Matrix|  | Servo    |
            | Module|  | LCD   |  | Keypad|  | Motors   |
            +-------+  | 2004  |  +-------+  +----+-----+
                       +-------+  +-------+       |
                                  | 2x IR |       | (PWM signal)
                                  | Sensor|       |
                                  +-------+  +----v-----+
                                             | Power    |
                                             | Supply   |
                                             | Module   |
                                             +----+-----+
                                                  |
                                             +----v-----+
                                             | 9V       |
                                             | Battery  |
                                             | Holder   |
                                             +----------+
```

## Log

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **NUCLEO-U545RE-Q Microcontroller:** The central processing unit of the system. 
- **4x4 Matrix Keypad:** A tactile interface that allows users to enter selections and confirm or cancel their transactions.
- **LCD 2004 Display (20x4):** The main output interface for the user used to communicate system states, instructions, and feedback messages.
- **I2C LCD Interface Module:** Used to minimize the required wiring to the microcontroller.
- **4x SG90 360-Degree Servo Motors:** Each servo motor controls a spiral dispensing 
mechanism assigned to a specific product slot. When a transaction is completed, the 
corresponding motor rotates to release the selected product.
- **2x IR Obstacle Avoidance Sensors:** Used to detect coin insertion. Each time a 
coin passes through the sensor, a signal is sent to the microcontroller to update 
the current balance.
- **RFID RC522 Module:** Acts as the card payment interface of 
the vending machine, simulating a real-world contactless payment terminal.
- **Power Supply Module:** A dedicated power regulation unit used to provide stable voltage and current to the servo motors, preventing electrical overload on the microcontroller.
- **Battery Holder:** Provides a portable power source for the system.

## Schematics

Place your KiCAD or similar schematics here in SVG format.

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| STM32U545RE-Q |  central processing unit of the vending machine which controls all connected components | 130 lei
| 4x4 Matrix Keypad | selects a product by pressing the corresponding number key | 17.99 lei |
| Power Supply Module | powers servo motors with a stable 5V volage | 7.99 |
| 9V Battery Holder with DC jack 5.5x2.1 mm and switch | supplies 9V power to the power supply module | 8.99 |  
| 2x IR Obstacle Avoidance Sensor | detects coin insertion | 6,24 lei |
| 4x SG90 360-degree Continuous Servo Motor | rotates the spiral dispensing mechanism to release the selected product | 51,56 lei |
| I2C Interface Module | reduces the number of wires needed to connect the LCD display to the microcontroller | 5.11 lei |
| LCD 2004 Display | main communication interface between the vending machine and the user | 28.70 lei |
| RFID RC522 Module | simulates card payment | 10 lei |




## Software

| Library | Description | Usage |
|---------|-------------|-------|
| cortex-m | Low-level access to Cortex-M processors | Used to handle interrupts(NVIC) |
| cortex-m-rt | Runtime support for Cortex-M processors | Starts the vending machine program when the NUCLEO powers on |
| embedded-hal | A Hardware Abstraction Layer (HAL) for embedded systems | Used by the keypad, IR sensors, LCD, servo motors and RFID drivers |
| embassy-executor | Async executor for embedded systems | Runs keypad scanning, coin detection, RFID reading, LCD updating and servo control at the same time |
| embassy-time | Time management for embassy | Delays, timeouts and timers |
| embassy-stm32 | STM32 HAL for embassy | Configures GPIO for keypad and IR sensors, I2C for LCD, SPI for RFID and PWM for servo motors |
| embassy-sync | Task Synchronization | Shares data between concurrent tasks |
| defmt | Specialized logging framework | Sends live status updates to your PC for testing and debugging |
| defmt-rtt | RTT transport layer | The communication bridge that sends your debug logs to your PC over the USB cable |
| panic-probe | Panic handler for embedded | Safely stops everything and reports if the program hits a major error |
| hd44780-driver | Driver for HD44780 LCD displays | LCD 2004 display control |
| keypad | Matrix keypad driver | Detects which product number the user pressed on the keypad |
| mfrc522 | RFID RC522 driver | Reads the RFID card when the user pays by tapping instead of inserting coins |

## Links

1. https://www.youtube.com/watch?v=BHQBsswUeT0&t=378s
2. https://www.youtube.com/watch?v=C7CYOOfb10s
3. https://embassy.dev
