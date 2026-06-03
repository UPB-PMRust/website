# Rusty Coffee
An automated coffee machine in which you can schedule your coffee and customize it to your heart's desire.

:::info 

*Author*: Lucian-Stefan Lesu \
*GitHub Project Link*: https://github.com/UPB-PMRust-Students/fils-project-2026-Lluke18

:::

<!-- do not delete the \ after your name -->

## Description

This smart coffee machine has lots of additional features and options when it comes to brewing your favorite refreshing hot drink - coffee. It also has a pulse sensor that, if your pulse is too high, won’t let you drink any coffee!

## Motivation

Because I love coffee. Of course, I'm not addicted. 

## Architecture 

Rusty Coffee is a smart, real-time espresso machine powered by an STM32 microcontroller running the Embassy async RTOS. By utilizing concurrent background tasks, the system seamlessly manages a custom touchscreen interface, wireless Bluetooth overrides, and a precision thermal controller simultaneously, ensuring the UI never lags or freezes.

The machine features a unique dual-sensor safety interlock. Before dispensing, an ultrasonic sensor physically verifies that a mug is present, while a medical-grade pulse oximeter acts as a biometric gatekeeper—calculating a real-time rolling average of the user's heart rate to ensure it is at a safe level. Once cleared, a software-defined PID algorithm dynamically pulses a high-voltage Solid State Relay, locking the espresso boiler precisely at 98.0°C for the perfect extraction.

Some photos of the project:



## Log

<!-- write your progress here every week -->

###  13 - 19 April
The main Stm32 arrived from mouser. Tested it, and everything works fine.
I disassembled the coffee machine that I bought for the heating element. At the lab, I'll desolder the wires connected to the relay module so that I can connect it to a way cooler one.

###  20 - 26 April
The rest of the components arrived. Aside from the pulse sensor, which doesn't work, apparently, everything seems to be good. Also, I started connecting the display. The LCD looks good, I started with basic shapes from the embedded graphics library. In the future, I'll also have to implement the touch component.

###  27 - 3 May

I finally managed to connect the touch as well. I had to be careful, because apparently the max freq of the touch chip is only 5Mhz, while the screen can go up to much higher. I also implemented the ultrasonic sensor, which was easy work.

###  4 - 10 May
Ok so I connected the thermocouple as near as I could get it to the heating element. I was lucky, as the coffee machine I bought ( the cheapest one I found, obviously...) had a little knob in which I fitted the screw. On the other end, I connected it to the max31855 module (it's basically a special ADC and amplifier for the signal)

###  11 - 17 May
Focused more on code, and also connected the newly arrived max30102 SPO2 sensor (thx Roy for the recommendation).

###  18 - 24 May
I managed to connect the bluetooth module as well (thx Mircea for the trade!). I also connected the lower voltage part of the SSR. Other than that, I wrote more code...

### 25 - 31 May
Time to bring it all together. I did the final connections of the heating element, using Lexman(basically Wago knockoffs) connectors. I also connected the 230V part to the heating element. I made extra sure everything is safe by isolating it with special tape. Finally, I Screwed the coffee machine back together. Well, let's hope it works at the Pm fair.


## Hardware

- STM32U545 microcontroller
- max30102 sensor
- TFT LCD Touchscreen
- heating element + case from a pre-existing coffee machine
- HC-SR04 ultrasonic sensor
- HC-05 bluetooth module
- type K thermocouple
- max31855 thermocouple module
- DA solid state relay 


### Schematics

![Schematic](projectscheme.svg)


### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 

| [Device](link://to/device) | This is used ... | [price](link://to/store) |



-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545](https://www.st.com/en/microcontrollers-microprocessors/stm32u535-545.html) | The microcontroller | [106.59 RON] (https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D) |
| [max30102] | pulse sensor | [32 RON] (https://www.emag.ro/modul-puls-si-spo2-max30102-multicolor-max30102-mod-black/pd/D3R798MBM/) |
| [ST7789 TFT LCD Screen] | tft lcd display | [50 RON] (https://www.emag.ro/display-tft-spi-2-8-inch-240x320-lcd-cu-touchscreen-driver-st7789v-arduino-emg359/pd/DP347SYBM/)|
| [HC-SR04] | ultrasonic sensor | [15 RON] (https://www.optimusdigital.ro/en/ultrasonic-sensors/2328-senzor-ultrasonic-de-distana-hc-sr04-compatibil-33-v-i-5-v.html?search_query=hc-sr04&results=15)|
| [max31855] | temperature sensor | [50 RON] (https://www.emag.ro/sezor-de-temperatura-max31855-breakout-pentru-termocuplu-tip-k-interfata-spi-dc-3-5v-bmx640/pd/DX7WTR3BM/)|
| [k-type thermocouple] | thermocouple | [15 RON] (https://www.optimusdigital.ro/en/temperature-sensors/5006-k-type-thermocouple-m6-1-m.html?search_query=thermocouple&results=40)|
| [SSR DA] | solid state relay | [40 RON] (https://www.emag.ro/releu-solid-state-ssr-25da-ssr-25-da-ac-control-3-32v-dc-pana-la-24-380v-ac-aur331/pd/DDK72P3BM/)|
| [Wires] | the Dupont wires | [5 RON] (https://www.optimusdigital.ro/en/wires-with-connectors/889-set-fire-tata-tata-10p-20-cm.html?search_query=wires&results=429)|
| [HC-05] | bluetooth module | [29 RON] (https://www.optimusdigital.ro/en/wireless-bluetooth/153-hc-05-master-slave-bluetooth-module-with-adapter-33v-and-5v-compatible.html?search_query=hc+05+bluetooth+module&results=65)|




## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [mipidsi](https://github.com/almindor/mipidsi) | Display driver for ST7789 | Used for the custom display of the coffee machine |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [max3010x](https://github.com/eldruin/max3010x-rs) | pulse sensor library | The strictly typed I2C driver configured for 18-bit Pw411 resolution and FIFO rollover to read the optical pulse data.
| [pid](https://github.com/braincore/pid-rs) | PID library| The mathematical controller calculating the exact PWM duty cycle needed to hold the boiler at 90.0°C |
| [heapless](https://github.com/rust-embedded/heapless) | heapless library| Providing statically allocated String and Vec buffers so the system never panics from dynamic memory allocation failures.
| [pid](https://github.com/braincore/pid-rs) | PID library| The mathematical controller calculating the exact PWM duty cycle needed to hold the boiler at 90.0°C |
| [defmt](https://www.google.com/search?q=%5Bhttps://github.com/knurling-rs/defmt%5D(https://github.com/knurling-rs/defmt)) | Deferred formatting logging framework | Providing ultra-fast, low-overhead logging for the pulse sensor diagnostics, touch interactions, and Bluetooth commands over your hardware debug probe. |
| [embassy](https://www.google.com/search?q=%5Bhttps://github.com/embassy-rs/embassy%5D(https://github.com/embassy-rs/embassy)) | Async embedded framework for Rust | Providing the real-time task scheduler (embassy-executor), timers (embassy-time), and hardware drivers (embassy-stm32) for the I2C, SPI, and UART peripherals. |
| [embassy-sync](https://www.google.com/search?q=%5Bhttps://github.com/embassy-rs/embassy/tree/main/embassy-sync%5D(https://github.com/embassy-rs/embassy/tree/main/embassy-sync)) | Async synchronization primitives | Managing the Watch variables (ALARM_TIME, LATEST_BPM, BREW_ACTIVE) to safely share state between the Bluetooth, Pulse, and Main UI tasks without locking up the CPU. |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Popular Coffee addict explains why cheap coffee machines suck](https://www.youtube.com/watch?v=P-Ga8SRhRrE&list=PLs-AM-gsRo5KqLKIKE5brhv4XxkQ8MSIq)
2. [Weirdo Coffee addict reviews an open-source coffee machine](https://www.youtube.com/watch?v=UaC4IQO2WCk&t=1176s)
3. [Hipster coffee addict builds his own hipster espresso machine](https://www.youtube.com/watch?v=JlgXGrb4lVE&list=PLs-AM-gsRo5KqLKIKE5brhv4XxkQ8MSIq&index=3)
