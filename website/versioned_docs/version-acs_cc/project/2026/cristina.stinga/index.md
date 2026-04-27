# Rusty Goals

A dual-controller foosball game

:::info

**Author**: Cristina Stîngă \
**Github Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Cristinaaa12

:::

## Description

The project is an interactive foosball game designed for a dynamic two-player experience. Each player uses a custom-built remote controller to command their digital striker on the field. The strikers are powered by stepper motors and move along the OX axis to position themselves  and execute rapid striking maneuvers to kick the ball.

Using a set of arcade-style buttons, players can move their striker left or right to block shots and trigger a strike action to score a goal. The system features a distributed architecture, where a central STM32 master coordinates the movement while two Raspberry Pi Pico units handle player input and real-time scoring updates on OLED displays. To ensure reliability and precision, the machine is equipped with endstops to recognize its frame boundaries and infrared sensors to automatically detect goals.

This project combines hardware engineering and real-time control into a fun, high-speed test of player coordination.

## Motivation

I wanted to create an interactive project that I could test and enjoy with my classmates. After researching various concepts, I was inspired by a unique foosball mechanism I discovered on social media.

## Architecture

The project uses a modular design where different parts work together to control the game and track the score in real-time.

**Main Components:**

* **Processing Units**
  * STM32 Nucleo-U545RE-Q: This is the main controller. It handles the game logic and monitors all sensors at high speed.
  * Raspberry Pi Pico H: These boards manage the buttons and screens for the players. They send player inputs to the STM32 and update the displays based on the information received back.

* **Movement & Action**
  * Attacker Mechanism: This part uses stepper motors and linear rails. The motors are connected to the STM32 through A4988 drivers, which translate digital signals into precise physical movement.

* **Sensors & Feedback**
  * Goal Detection: Infrared sensors are placed in the goals. When the ball passes, the system automatically updates the score.
  * OLED Displays: Small screens that show the current score.
  * Endstops: Limit switches that tell the motors to stop at the edges.

* **User Controls**
  * Arcade Buttons: High-quality buttons that players use to move and shoot. They are designed for fast and repeated use.

![Diagram](images/architecture.svg)

## Log

### Week 6 April - 12 April

* Thought about the game concept and made a first list of needed parts.

* Looked into how the players should move and kick.

### Week 13 April - 19 April

* Checked if the STM32 and Pico work well together.

* Figured out how to link the controllers to the main board.

* Decided to use buttons instead of joysticks for better control.

### Week 20 - 26 April

* Ordered all the necessary electronic components and hardware parts.

* Sent the custom-designed slider parts for 3D printing to begin the mechanical assembly.

## Hardware

The system uses one main controller (STM32 Nucleo-U545RE-Q) and two secondary boards (Raspberry Pi Pico H) to handle the game smoothly.

* **Main Controller (STM32 Nucleo-U545RE-Q)**: This is the brain of the project. It runs the game logic and tells the motors how to move. It communicates with the player boards using Serial communication via USB-C modules and sends signals to the motor drivers.

* **Player Boards (Raspberry Pi Pico)**: Each player has a Pico board. Its job is to read the Arcade Buttons and show the score on the OLED screens.

* **Motors & Movement:** I use NEMA 17 motors with a GT2 belt system to move the players. Linear bearings and steel rods make the movement smooth, while Endstops stop the motors safely at the edges.

* **Sensors:** IR Sensors are placed in the goals. When the ball passes through, the sensor sends a signal to the STM32 to increase the score.

* **Power Supply:** A 12V adapter powers the motors. I use an LM2596 converter to change 12V into 5V for the controllers.

### Schematics

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| STM32 | The microcontroller | - |
| [Raspberry Pi Pico H](https://www.bitmi.ro/placi-de-dezvoltare/placa-de-dezvoltare-raspberry-pi-pico-h-rp2040-264kb-ram-10848.html) | Boards for the players to read button inputs and control the screens | 41.14 RON x 2 |
| [NEMA 17 Stepper Motor](https://sigmanortec.ro/Nema17-1-5A-p125805542) | Provide the physical force for moving and kicking the ball | 67.06 RON x 2 |
| [A4988 Drivers](https://sigmanortec.ro/Driver-stepper-A4988-Radiator-p125711037) | Act as intermediaries to control the motors based on STM32 signals | 8.09 RON x 2 |
| [A4988 Expansion Board](https://sigmanortec.ro/placa-expansiune-driver-motor-stepper-drv8825-si-a4988-5v?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72PYOVYYJVcWF-co_NlzFjur9&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV-6k3v8b6p63zTdHONLPGIyYjfyJo4oz8hcuQrUugva4Ze6RGrzyyIaAlCVEALw_wcB) | Simplifies wiring and protects drivers from voltage spikes | 9.97 RON x 2|
| [OLED Displays (0.96")](https://sigmanortec.ro/display-oled-096-i2c-iic-alb) | Small screens that show the live score to each player | 16.95 RON x 2 |
| [Infrared obstacle sensor](https://sigmanortec.ro/Senzor-obstacol-IR-p125423458) | Detect when the ball enters the goal to update the score automatically | 3.12 RON x 2 |
| [Endstops SS-5GL2](https://sigmanortec.ro/Endstop-mecanic-SS-5GL2-p136284192) | Small switches that detect when the slider reaches its physical limit | 5.23 RON x 4 |
| [Arcade Buttons](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1852-buton-arcade-iluminat-24mm-galben.html) | Durable buttons used by players to move and shoot | 9.99 RON x 6 |
| [LM2596 Step-Down Module](https://sigmanortec.ro/Modul-coborator-tensiune-adjustabil-LM2596-DC-DC-4-5-40V-3A-p134532509) | Converts 12V to stable 5V output | 6.69 RON |
| [GT2 Timing Pulley](https://sigmanortec.ro/Fulie-dintata-GT2-20-dinti-ax-5mm-p125814315) | Used to drive the belt and move the players | 4.67 RON x 2 |
| [GT2 Timing Belt](https://sigmanortec.ro/Curea-GT2-6mm-p125814436) | Transfers rotation into linear movement | 6.70 RON x 2 |
| [LM8UU Linear Bearings](https://sigmanortec.ro/Rulment-liniar-LM8UU-p130575745) | Ensure smooth sliding on the steel rods | 4.90 RON x 8 |
| [GT2 Idler Pulley](https://sigmanortec.ro/Fulie-rola-intinzatoare-GT2-tip-20-fara-dinti-5mm-p148570815) | Keeps the belt tensioned and aligned | 10.89 RON x 2 |
| [Type-C Panel Module](https://sigmanortec.ro/modul-type-c-la-pini-de-panou-tensiune-si-date-usb-31) | Professional connection for USB-C cables | 11.53 RON x 4 |
| 3D Printed Parts | Custom mounts and sliders | - |
| [Linear Shafts (8mm x 300mm)](https://www.temu.com/goods.html?goods_id=601105217889870) | Precision steel rods for the track | 28.49 RON x 4 |
| [USB-C Braided Cables Set](https://www.temu.com/goods.html?goods_id=601099799616821) | Connect controllers to the STM32 | 9.15 RON |
| [12V 5A Power Adapter](https://www.emag.ro/alimentator-12v-5a-cu-mufa-5-5-2-1-mm-cablu-de-alimentare-inclus-ev-5a/pd/DY6PTDBBM/) | Main power source for the entire system | 50.70 RON |
| [DC Female Jack Adapter](https://www.emag.ro/mufa-alimentare-mama-cu-surub-201801013096/pd/D9FK8GBBM/) | Connects power adapter to wiring | 4.14 RON |
| [Resistor Kit](https://sigmanortec.ro/kit-rezistori-30-valori-20-bucati?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72PYOVYYJVcWF-co_NlzFjur9&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV8M3bdlOn3xPhfKleAuuOxXfd9dJIwNgQctf_sCVCF7DhOuy3iUe1UaAqs0EALw_wcB) | Used for current limiting, signal conditioning and pull-up/down configurations | 15.16 RON |
| [Mini Breadboard (170 pts)](https://sigmanortec.ro/Breadboard-170-puncte-diferite-culori-p126177349?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72PYOVYYJVcWF-co_NlzFjur9&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV9Gx6bAxHp8FiXbmVABCp3OWb9gofa_dZ5HeVo4je3Yepal9UyQ3ssaApKjEALw_wcB) | Provides a stable mounting point for the Pico H and connects arcade buttons/OLEDs | 2.42 RON x 2 |
| [Breadboard (830 pts)](https://www.emag.ro/breadboard-h-hct-tronic-830-puncte-de-conectare-abs-200x630-puncte-034-066/pd/DBNQ7R3BM/?ref=history-shopping_485525269_1558_1) | Main hub for STM32 integration | 10.00 RON |
| [Electrolytic Capacitor](https://www.emag.ro/condensator-electrolitic-100uf-25v-dc-105-c-aishi-t128297/pd/D911MSMBM/?ref=history-shopping_486202414_7656_1) | Filters power supply noise and protects drivers from voltage spikes | 3.03 RON x 3 |
| **TOTAL** | **Estimated Total Project Cost** | **727.09 RON** |

## Software


| Library | Description | Usage |
|--------|--------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | Hardware Abstraction Layer for STM32 microcontrollers | Used to control the motor pins, read the IR sensors and handle Serial communication |
| [embassy-rp](https://crates.io/crates/embassy-rp) | Hardware Abstraction Layer for the Raspberry Pi Pico | Used on the player boards to read the arcade buttons and drive the OLED displays |
| [embassy-executor](https://crates.io/crates/embassy-executor) | An async/await executor for embedded systems | Used to run multiple tasks at once, like moving the motor while checking for button presses |
| [embedded-graphics](https://crates.io/crates/embedded-graphics) |	A 2D graphics library for embedded screens |	Used to draw the score on the OLED displays |
| [ssd1306](https://crates.io/crates/ssd1306)	| OLED Display driver	| This is the specific driver that talks to my 0.96" screens via I2C |
| [embassy-time](https://crates.io/crates/embassy-time) | Handles timing and delays | Used to control the motor speed and to make sure a button press is counted only once |
| [defmt](https://crates.io/crates/defmt)	| A highly efficient logging framework for resource-constrained devices |	Used for debugging |
| [panic-probe](https://crates.io/crates/panic-probe)	| Panic handler	| Used for debugging crashes


## Links

* **Project Concept:** https://www.instagram.com/reel/DVoDEazjhOp/?igsh=cjRhNHViMnJhc3Nu
* **Movement Mechanism:** https://www.instagram.com/reel/DXIX2Q3Efan/?igsh=MTh3N2l1MmZrN3hyOQ==
