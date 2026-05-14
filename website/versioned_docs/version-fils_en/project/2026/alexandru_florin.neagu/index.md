# SteadyFrame
A handheld 3-axis camera stabilizer controlled by an STM32 board and programmed in Rust.

:::info 

**Author**: Neagu Alexandru-Florin \
**Group**: 1222EEB \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-ithinktoomuch05

:::

<!-- do not delete the \ after your name -->

## Description

This project represents a **3-axis camera stabilizer** designed to keep a relatively small DSLR/Mirrorless camera steady while the user is moving - My **Canon EOS R camera** paired with an **EF-S 18-55mm f/3.5-5.6 IS II Canon lens** (total weight: ~1kg). 

The stabilizer uses an STM32 NUCLEO-U545RE-Q development board, an IMU sensor, brushless motors, motor drivers, a joystick and a LiPo battery.

## Motivation

I chose this project because I am currently learning videography and cinematography and the prices of camera equipment are simply too much for a freelancing student (even more than the price of the standalone components, *huh, that's unbelievable!*, **I know**, until you search for how much actually good equipment costs). As such, I have started on a quest of creating my own personal equipment, starting with maybe the most important tool - stabilization (because my hands aren't the steadiest out there and it makes a world of difference on film).

## Architecture 

This is the initial diagram, subject to change as I develop the project, regarding how I think my project should be organized/work:

![Architecture diagram](./steadyframe.svg)

## Main components:

## Log

<!-- write your progress here every week -->
### Week 6: 30 March - 6 April

Coming up with possible project ideas.

### Week 7: 6 - 12 April

Conceptual stage, thinking how I would want my gimbal project to work, coming up with ideas regarding components and spending a **lot** of time looking for correct/compatible components, as I think this project is a bit complex for my current level of understading.

### Week 8: 12 - 18 April

Ordering a big part of components (motors, drivers+encoders, STM32 board). More research.

### Week 9: 19 - 25 May

Ordering next batch of components (IMU, battery and charger). More research x2. Changed to attempting CAN connection (lord have mercy).

## Hardware

Hardware used for creating this project (list currently WIP): STM32 NUCLEO-U545RE-Q, GM5208-24 motors, MKS XDRIVE MINI drivers with AS5047P on board, ICM-20948 IMU, LiPo GENS ACE G-Tech Soaring 4S 14.8 V 2200mA battery with appropriate charger and 2 Axis Joystick. Photos of components will be added after they arrive (currently still being delivered).

### Schematics


### Bill of Materials
#### --- WORK IN PROGRESS, NOT FINISHED ---
<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/resource/en/data_brief/nucleo-c031c6.pdf) | The microcontroller - computes everything and sends the appropriate signals to all the components | [105 RON](https://eu.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&utm_id=6470900573&utm_source=google&utm_medium=cpc&utm_marketing_tactic=emeacorp&gad_source=1&gad_campaignid=6470900573&gbraid=0AAAAADn_wf1J6XpRotkoYj96_ZbUSaPnH&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV8YyFRvdTHnnm_d9qDLFYVD-gl4i7w7_-Op_zdGssVpHLPnJSMNH3saAlkOEALw_wcB) |
| [3x GM5208-24](https://tyi-model.en.made-in-china.com/product/owetZdnOMvTG/China-Iflight-Ipower-GM5208-24-Brushless-Gimbal-Motor-12-5mm-Metal-Plastic-Slipring-for-Drones-Remote-Control-Adapter-Upgrade.html) | The motors - stabilize the camera along the 3 axis and move in required directions when requested to | [702 RON](https://www.aliexpress.com/item/32900557812.html?spm=a2g0o.order_list.order_list_main.11.66b61802kM3UOQ) |
| [3x MKS XDRIVE MINI with AS5047P on board](https://makerbase3d.com/product/makerbase-xdrive-mini-high-precision-brushless-servo-motor-controller-based-on-odrive3-6-with-as5047p-on-board/?srsltid=AfmBOooAoRHQBeOUDcw-KHU8tu3dG715gBSeGb9DsnB4TYA0q9W50RMU) | The motor drivers and encoders - understand motor position and rotation (magnetic rotary encoders) and communicates with the motors when to rotate in order to be stabilized or moved when requested to (drivers) | [534 RON](https://www.aliexpress.com/item/1005006480243178.html?spm=a2g0o.order_list.order_list_main.5.66b61802kM3UOQ#nav-specification) |
| [AS5047P specs](https://look.ams-osram.com/m/d05ee39221f9857/original/AS5047P-DS000324.pdf) | The motor encoders | [included with drivers](https://www.aliexpress.com/item/1005006480243178.html?spm=a2g0o.order_list.order_list_main.5.66b61802kM3UOQ#nav-specification) |
| [SN65HVD230](https://www.ti.com/lit/ds/symlink/sn65hvd230.pdf?ts=1777190583955&ref_url=https%253A%252F%252Fwww.ti.com%252Fsitesearch%252Fen-us%252Fdocs%252Funiversalsearch.tsp%253FlangPref%253Den-US%2526nr%253D1%2526searchTerm%253Dsn65hvd230dr) | CAN communication module (transceiver) for motor drivers | [11 RON](https://sigmanortec.ro/modul-comunincare-can-sn65hvd230-33v?SubmitCurrency=1&id_currency=2&gad_source=1&gad_campaignid=23069763085&gbraid=0AAAAAC3W72PYOVYYJVcWF-co_NlzFjur9&gclid=Cj0KCQjw77bPBhC_ARIsAGAjjV9MDgruKgz9tddDMBOUHy-TL0Vv_MkhyJ5ICtPL0P5spm6g2YHFDGQaAuh-EALw_wcB) |
| [ICM-20948](https://product.tdk.com/system/files/dam/doc/product/sensor/mortion-inertial/imu/data_sheet/ds-000189-icm-20948-v1.5.pdf) | The 9-Axis Inertial Measurement Unit - understands the position of the camera and helps the microcontroller understand required position and rotation changes | [37 RON](https://ardushop.ro/ro/groundstudio/1163-modul-9-axe-icm-20948-groundstudio-6427854000699.html) |
| [LiPo GENS ACE G-Tech Soaring 4S 14.8 V 2200mA](https://gensace.de/pages/lipo-battery-guide) | The battery - supplies power to all components | [150 RON](https://www.emag.ro/acumulator-lipo-gens-ace-g-tech-soaring-14-8-v-2200-ma-30c-xt60-men-ip-415004/pd/DF4XMTYBM/) |
| [LM2596S](https://www.ti.com/lit/ds/symlink/lm2596.pdf) | DC-DC Buck Step Down Convertor LM2596S 4.0~40V to 1.25-37V - for supplying correct voltage to STM32 board| [48 RON](https://www.emag.ro/modul-dc-dc-buck-step-down-lm2596s-dc-dc-4-0-40v-la-1-25-37v-regulator-de-tensiune-reglabil-cu-voltmetru-led-stlxy-741050522578/pd/DKNQT83BM/?ref=sponsored_products_search_f_b_1_5&recid=recads_1_b90d01a332c40f582acfccf6bf3bca72edcfc042cd11701d2d49ef94121cef69_1777211422&aid=549a3d7e-f438-11f0-801c-06eaf0d4245d&oid=302862900&scenario_ID=1) |
| [Gens Ace iMars mini G-Tech](https://gensace.de/products/gens-ace-imars-mini-g-tech-usb-c-2-4s-60w-rc-battery-charger-with-power-supply-adapter-and-adpter-cable-eu) | The battery charger - charges the battery when needed | [245 RON](https://www.autorc.ro/incarcatoare-acumulatori/10560-incarcator-acumulatori-gens-ace-imars-mini-g-tech-usb-c-2-4s-60w.html) |
| [2 Axis Joystick](https://www.laskakit.cz/user/related_files/joystick_module.pdf) | The joystick - human input to "move" (point) the camera in a specified direction | [6 RON](https://sigmanortec.ro/Modul-joystick-doua-axe-XY-p126458908#) |
| []() | Total: | [~1850 RON]() |
NOT CONFIRMED YET:
| [ILI9341](https://cdn-shop.adafruit.com/datasheets/ILI9341.pdf) | The LCD 3.2 inch touch display - would be used for observing information displayed by the microcontroller regarding position and rotation, plus would help navigate a menu for parameters - lots of stuff to do here, not all figured out *yet* | [117 RON](https://sigmanortec.ro/display-lcd-32-inch-240x320-touch-ili9341-sd-mega) |
| [Wires]() | No exact count, still figuring out | []() |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://crates.io/crates/embassy-stm32) | HAL for STM32 microcontrollers, with drivers for GPIO, ADC, SPI, I2C, UART, timers, and CAN/FDCAN | Used as the main hardware abstraction layer for the STM32 NUCLEO-U545RE-Q, including IMU communication, joystick ADC reading, timers, and communication with the XDrive Mini controllers |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async executor for embedded Rust | Used to run concurrent tasks such as IMU sampling, joystick reading, control-loop updates, and telemetry/debug tasks | # no heap is required and tasks are statically allocated.
| [embassy-time](https://crates.io/crates/embassy-time) | Timekeeping, delays, and timeout utilities for Embassy-based applications | Used for periodic control-loop scheduling, sensor polling intervals, debounce timing and startup/calibration mode |
| [embedded-hal](https://crates.io/crates/embedded-hal) | Common hardware abstraction traits for embedded systems | Used as the generic interface layer for reusable drivers, especially for the IMU and other peripherals |
| [heapless](https://crates.io/crates/heapless) | static-friendly data structures that do not require dynamic memory allocation | Used for fixed-capacity queues, buffers, message passing, and storing small packets or samples without a heap |
| [static_cell](https://crates.io/crates/static_cell) | Runtime-initialized static storage for no_std applications | Used for safely allocating shared static resources such as buses, drivers, and global state needed by Embassy tasks - apparently |
| [defmt](https://crates.io/crates/defmt) | Compact logging framework | Used for debug logging, calibration output, fault reporting and runtime diagnostics during development | #for resource-constrained embedded targets
| [panic-probe](https://crates.io/crates/panic-probe) | Panic handler | Used to report panics cleanly during firmware development and debugging | #commonly used with probe-based embedded debugging 
| [embedded-can](https://crates.io/crates/embedded-can) | Abstraction layer for CAN communication | Used for communication by the STM32 with the three XDrive Mini boards over CAN. |
| [icm20948-rs](https://crates.io/crates/icm20948-rs)| Platform-agnostic driver for the ICM-20948 9-axis IMU | Used to initialize the ICM-20948, read accelerometer/gyroscope/magnetometer data, and expose it to the stabilization algorithm |
NOT CONFIRMED YET:
| [ili9341](https://crates.io/crates/ili9341) | Crates for display | Should be used for displaying information regarding position parameters and menu controls |
| [Probably way more crates needed ] | Work in progress | - |
## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Additional CAN tutorials god help me](https://source-robotics.github.io/Spectral-BLDC-docs/apage7_can/)
2. [Rust PID control](https://docs.rs/pid/latest/pid/)
3. [FOC and Gimbal Stabilization](https://lup.lub.lu.se/luur/download?func=downloadFile&recordOId=9092924&fileOId=9094241)
4. [IMU understanding](https://wolles-elektronikkiste.de/en/icm-20948-9-axis-sensor-part-i)
5. [Powering help](https://howtomechatronics.com/projects/diy-arduino-gimbal-self-stabilizing-platform/)

...
