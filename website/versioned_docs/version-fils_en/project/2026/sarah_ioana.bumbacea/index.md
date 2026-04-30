# Sakura Desk Lamp
A Desk Lamp

# Project Name
A multifunctional lamp

:::info 

**Author**: Bumbacea Ioana Sarah \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-sarahioana05-ship-it

:::

<!-- do not delete the \ after your name -->

## Description

This project consists of a smart decorative lamp built using using a recycled aluminum can, with RGB LEDs placed inside to create lighting effects. The base of the lamp is inspired by a cherry blossom tree design.

## Motivation

The idea for this project started from the desire to create a unique and visually appealing desk decoration using everyday materials. I also wanted the project to be more than just decorative, so I added features such as lighting effects, a timer, and a display, making it useful as a study lamp.

## Architecture 

The system is structured into several main functional components that interact with each other to provide the final behavior.
 ```

                +----------------------+
                |     Input Module     |
                |----------------------|
                | - Potentiometers     |
                | - Touch Sensor       |
                | - Button             |
                +----------+-----------+
                           |
                           v
                +----------------------+
                |    Control Logic     |
                |----------------------|
                | - Mode Selection     |
                | - Timer Management   |
                | - State Handling     |
                +----+-----------+-----+
                     |           |
         +-----------+           +-----------+
         v                                   v
+----------------------+         +----------------------+
|   LED Control        |         |   Display & Alerts   |
|----------------------|         |----------------------|
| - RGB PWM Control    |         | - LCD Update         |
| - Effects (rainbow,  |         | - Show time/date     |
|   blinking, etc.)    |         | - Buzzer control     |
+----------------------+         +----------------------+

 ```
 
## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
draw the schematic and planned how to create the lamp support and other decorative elements

### Week 12 - 18 May
tested some of the components

### Week 19 - 25 May
github assignment and started building the model

## Hardware

The project is built around an STM32 microcontroller and includes multiple input and output components to create an interactive lighting system. The power supply is based on two 18650 Li-ion batteries, connected through a boost converter to provide 12V for the LEDs and a step-down regulator to supply 5V to the microcontroller and other components.
STM32 Nucleo board – main controller
RGB LEDs – main light source
MOSFET transistors (IRF1405) – used to control LED channels
Potentiometers (x4) – color and brightness control
Capacitive touch sensor (TTP223) – mode switching
LCD display – system feedback (brightness, time, etc.)
Buzzer – timer alerts
DC motor – interactive energy generation element
Boost converter (XL6009) – steps voltage up to 12V
Step-down converter (LM2596) – regulates voltage to 5V
18650 batteries + holder + charger – portable power supply



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
|--------|--------|-------|
| [STM32 Nucleo-U545RE] | The microcontroller | [125 RON](https://ro.mouser.com/ProductDetail/511-NUCLEO-U545RE-Q) |
| [LCD de 1.44'' pentru STC, STM32 și Arduino (5 V)] | the lcd screen | [34,99 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/8589-lcd-de-144-pentru-stc-stm32-i-arduino-5-v.html) |
| [Rezistor Variabil 10k WH148] | The potentiometers | [2,39*4 RON](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/12360-rezistor-variabil-10k-wh148-poteniometru-fara-aiba-i-piulia.html) |
| [Buton 6x6x6] | five buttons | [1,80 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html) |
| [Modul DC-DC Boost XL6009] | voltage booster to 12V | [9,99 RON](https://www.optimusdigital.ro/ro/surse-ridicatoare-reglabile/160-modul-dc-dc-boost-xl6009.html) |
| [Tranzistor Mosfet IRF1405 (Canal N, 200W, 55V)] | three mosfets for the leds | [29,97 RON](https://www.optimusdigital.ro/ro/componente-electronice-tranzistoare/11856-tranzistor-mosfet-irf1405-canal-n-200w-55v.html) |
| [Suport de Baterii 2 x 18650] | Battery support | [3,99 RON](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html) |
| [Modul cu Senzor Capacitiv TTP223] | touch sensor | [2,97 RON](https://www.optimusdigital.ro/ro/senzori-senzori-de-atingere/861-modul-cu-senzor-capacitiv-ttp223.html) |
| [Baterie Reincarcabila 18650 ISR Li-Ion 3.6V 2600mAh] | power supply | [38,96 RON](https://www.emag.ro/baterie-reincarcabila-18650-isr-li-ion-3-6v-2600mah-2-6a-cu-descarcare-maxima-7-65a-ted-electric-a0116012-01/pd/DYFNB73BM/) |
| [Modul DC-DC Step Down LM2596] | the voltage regulator for 5V | [9,46 RON](https://www.emag.ro/modul-dc-dc-step-down-lm2596-765464701237/pd/DWHHRGBBM/?utm_source=mobile%20app&utm_medium=ios&utm_campaign=share%20product) |
| [Intrerupator Negru On/Off] | Button for the power supply | [1,98 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/7434-buton-negru-onoff.html?search_query=Intrerupator+Negru+On%2FOff&results=1) |
| [Buzzer Pasiv de 3.3V sau 3V] | buzzer for the alarm | [1,98 RON](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html?search_query=Buzzer+Pasiv+de+3.3V+sau+3V&results=1) |
| [Breadboard 90 x 52 x 8.5 mm] | two breadboards | [11,98 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/13247-breadboard-90-x-52-x-85-mm.html?search_query=Breadboard+90+x+52+x+8.5+mm&results=91) |
| [	Set de LED-uri Asortate de 5 mm si 3 mm (310 buc) cu Rezistoare Bonus] | LEDs and resistors | [26,99 RON](https://www.optimusdigital.ro/ro/kituri-optimus-digital/9517-set-de-led-uri-asortate-de-5-mm-si-3-mm-310-buc-cu-rezistoare-bonus.html?search_query=Set+de+LED-uri+Asortate+de+5+mm+si+3+mm+%28310+buc%29+cu+Rezistoare+Bonus&results=1) |
| [Set Fire pentru Breadboard] | wires for components | [7,99 RON](cannot find the link anymore) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) |Async HAL for STM32 microcontrollers | Main framework for handling tasks, timers, and peripherals |
| [embassy-time](https://github.com/embassy-rs/embassy) | Time handling for async systems | Used for delays, timers, and scheduling (lamp timer) |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used to draw text and UI on the LCD display |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Used for general GPIO, ADC, and PWM control |

| [lcd driver]() | LCD driver | Used to control the LCD display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [STM32U5 Documentation](https://www.st.com/resource/en/data_brief/nucleo-c031c6.pdf)
2. [monster can](https://www.ifit.ee/en/a/monster-ultra-500ml-black-cherry)
...
