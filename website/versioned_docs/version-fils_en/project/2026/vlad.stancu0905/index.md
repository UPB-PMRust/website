# Autonomous Context-Aware Thermal Rover

A mobile, dual-axis targeting and navigation system that autonomously patrols an environment, avoids obstacles, and tracks human heat signatures using Rust and the Embassy framework.

:::info

**Author**: Stancu Vlad-Gabriel \

**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Vledituz

:::

## Description
The Autonomous Context-Aware Thermal Rover navigates using an ultrasonic sensor to avoid collisions. When the onboard 8x8 infrared matrix detects a heat signature, the rover halts its patrol, and a motorized pan-tilt turret locks onto the target. Depending on the ambient room lighting (detected via a photoresistor), the system dynamically switches between a passive observation mode and an active "Night Watch" mode, engaging a laser diode payload. Live telemetry and the thermal map are displayed on a local SPI LCD and streamed over Wi-Fi to a remote web dashboard.

## Motivation
I was always fascinated with the world of security measures and embedded systems. I intend to make a project that would actually prove useful down the line after more work put into it in high risk areas such as museums, banks, etc.

## Architecture
The system relies on an STM32U545RE-Q acting as the central processing unit, running concurrent async tasks. 
* **Perception:** An AMG8833 Thermal Camera (I2C) scans for heat, an HC-SR04 Ultrasonic Sensor (GPIO/Timers) handles spatial distance, and an LDR (ADC) monitors ambient light.
* **Actuation:** Four DC motors are driven by an L298N Motor Driver receiving PWM signals for mobility. Two SG90 servos receive 50Hz PWM signals to aim the thermal turret. A 5V Laser Diode acts as the GPIO-controlled payload.
* **Interface & Connectivity:** An ST7735 1.44" LCD (SPI) provides local UI rendering. An ESP32-C3 co-processor communicates with the STM32 via USART AT commands to host a TCP web server for remote monitoring.
* **Power Management:** A 4xAA battery pack supplies power to the L298N driver, which handles the high-current DC motors and servos. The microcontrollers and logic-level sensors are powered independently to prevent voltage brownouts.


## Log

### Weeks 4-5
Project setup, `embassy-stm32` initialization, and establishing I2C communication with the AMG8833 sensor to output raw matrix data over the debugger.

### Weeks 5-6
Configuring PWM timers for the SG90 servos and the L298N motor driver; basic movement calibration and HC-SR04 distance integration.

### Weeks 7-8
ADC polling for the LDR, SPI configuration for the ST7735 display.

## Hardware
The rover's chassis will be created out of lego bricks to ensure it is both lightweight and sturdy. All components will be soldered on a prototyping board, with extension wires to make it aestethically more pleasing and allow breathing room for further expansion. The main sensor, the AMG8833, will be mounted at the very fron on a pan-tilt bracket with 2 SG90 servomotors to allow a wide scannable area.

### Schematics
*(Insert your KiCAD schematic here in SVG format)*

### Bill of Materials

| Device | Usage | Price |
| --- | --- | --- |
| [STM32U545RE-Q Nucleo](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q) | The main microcontroller running the Embassy async executor | [~110 RON](https://estore.st.com/en/nucleo-u545re-q-cpn.html?srsltid=AfmBOoo6z9udikrw838OhbOFJ-e2K6fEXFpE_ltR-TPn7sKhd3_Jq5nPfXU) |
| [ESP32-C3 SuperMini](https://ardushop.ro/ro/plci-de-dezvoltare/2224-placa-de-dezvoltare-esp32-c3-super-mini-6427854034298.html?gad_source=1&gad_campaignid=22058879462&gbraid=0AAAAADlKU-4NElOyc2EV7vGPeYGq1IQHi&gclid=CjwKCAjwnZfPBhAGEiwAzg-VzB-T9AQO1PbX_a1PN9ZTkpKJN3jFpwZhN2CkeITWTx8J_yac14b11BoCRb0QAvD_BwE) | Wi-Fi Bridge for the remote web dashboard | [~25 RON](https://ardushop.ro/ro/plci-de-dezvoltare/2224-placa-de-dezvoltare-esp32-c3-super-mini-6427854034298.html?gad_source=1&gad_campaignid=22058879462&gbraid=0AAAAADlKU-4NElOyc2EV7vGPeYGq1IQHi&gclid=CjwKCAjwnZfPBhAGEiwAzg-VzB-T9AQO1PbX_a1PN9ZTkpKJN3jFpwZhN2CkeITWTx8J_yac14b11BoCRb0QAvD_BwE)|
| [AMG8833 IR Sensor](https://www.bitmi.ro/modul-senzor-termic-infrarosu-gy-amg8833-12338.html?gad_source=1&gad_campaignid=21312430054&gbraid=0AAAAADLag-kY65avfg7fYwyFc_-mN_7T9&gclid=CjwKCAjwnZfPBhAGEiwAzg-VzBcPlBGxVjdTNe-U5mg6ELRsT9wxXzJfZWdGfikzsiX0iexwSyS-WBoC3ZMQAvD_BwE) | 8x8 I2C Thermal Camera for heat detection | [~200 RON](https://www.bitmi.ro/modul-senzor-termic-infrarosu-gy-amg8833-12338.html?gad_source=1&gad_campaignid=21312430054&gbraid=0AAAAADLag-kY65avfg7fYwyFc_-mN_7T9&gclid=CjwKCAjwnZfPBhAGEiwAzg-VzBcPlBGxVjdTNe-U5mg6ELRsT9wxXzJfZWdGfikzsiX0iexwSyS-WBoC3ZMQAvD_BwE)|
| [ST7735 1.44" SPI TFT LCD](https://www.aliexpress.com/item/1005008045403404.html?dp=553c5068a0a7781a6e771bc9774e4114&af=2159637&cv=47843&afref=https%3A%2F%2Feshopwell.xyz&mall_affr=pr3&utm_source=admitad&utm_medium=cpa&utm_campaign=2159637&utm_content=47843&dp=553c5068a0a7781a6e771bc9774e4114&af=2159637&cv=47843&afref=https%3A%2F%2Feshopwell.xyz&mall_affr=pr3&utm_source=admitad&utm_medium=cpa&utm_campaign=2159637&utm_content=47843&aff_fcid=c877ab0004e84518a8d98e0ec4e92187-1776728205870-06966-_ePNSNV&aff_fsk=_ePNSNV&aff_platform=portals-tool&sk=_ePNSNV&aff_trace_key=c877ab0004e84518a8d98e0ec4e92187-1776728205870-06966-_ePNSNV&terminal_id=585ea28564c543dd8448f66c4fc95084&afSmartRedirect=y) | Local UI and thermal map visualization | [~25 RON](https://www.aliexpress.com/item/1005008045403404.html?dp=553c5068a0a7781a6e771bc9774e4114&af=2159637&cv=47843&afref=https%3A%2F%2Feshopwell.xyz&mall_affr=pr3&utm_source=admitad&utm_medium=cpa&utm_campaign=2159637&utm_content=47843&dp=553c5068a0a7781a6e771bc9774e4114&af=2159637&cv=47843&afref=https%3A%2F%2Feshopwell.xyz&mall_affr=pr3&utm_source=admitad&utm_medium=cpa&utm_campaign=2159637&utm_content=47843&aff_fcid=c877ab0004e84518a8d98e0ec4e92187-1776728205870-06966-_ePNSNV&aff_fsk=_ePNSNV&aff_platform=portals-tool&sk=_ePNSNV&aff_trace_key=c877ab0004e84518a8d98e0ec4e92187-1776728205870-06966-_ePNSNV&terminal_id=585ea28564c543dd8448f66c4fc95084&afSmartRedirect=y)|
| [HC-SR04 Ultrasonic Sensor](https://www.aliexpress.com/item/1005008244300132.html?spm=a2g0o.productlist.main.3.263a3d78rtrTrW&algo_pvid=760114db-eb8c-4166-9cf2-87f153b3980d&algo_exp_id=760114db-eb8c-4166-9cf2-87f153b3980d-2&pdp_ext_f=%7B%22order%22%3A%22346%22%2C%22eval%22%3A%221%22%2C%22fromPage%22%3A%22search%22%7D&pdp_npi=6%40dis%21RON%2115.14%2115.14%21%21%2123.29%2123.29%21%402103985c17767282404337667ebbdc%2112000044353417795%21sea%21RO%212440347544%21X%211%210%21n_tag%3A-29919%3Bd%3A8711c890%3Bm03_new_user%3A-29895&curPageLogUid=dP0ep4P9zGLW&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005008244300132%7C_p_origin_prod%3A) | Obstacle avoidance / Distance measuring | [~15 RON](https://www.aliexpress.com/item/1005008244300132.html?spm=a2g0o.productlist.main.3.263a3d78rtrTrW&algo_pvid=760114db-eb8c-4166-9cf2-87f153b3980d&algo_exp_id=760114db-eb8c-4166-9cf2-87f153b3980d-2&pdp_ext_f=%7B%22order%22%3A%22346%22%2C%22eval%22%3A%221%22%2C%22fromPage%22%3A%22search%22%7D&pdp_npi=6%40dis%21RON%2115.14%2115.14%21%21%2123.29%2123.29%21%402103985c17767282404337667ebbdc%2112000044353417795%21sea%21RO%212440347544%21X%211%210%21n_tag%3A-29919%3Bd%3A8711c890%3Bm03_new_user%3A-29895&curPageLogUid=dP0ep4P9zGLW&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005008244300132%7C_p_origin_prod%3A)|
| [L298N Motor Driver](https://www.emag.ro/modul-driver-de-comanda-motor-cu-l298n-oky3195-ai0016-s190/pd/D7J1J7MBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_dc8f0f354b800cd0e3d3055da10d769438027f1b3b39fa232b82788fb46d52e5_1776728290&aid=19bb0f69-340e-11f1-801c-06eaf0d4245d&oid=122320892&scenario_ID=1) | High-current switching for chassis DC motors | [~14 RON](https://www.emag.ro/modul-driver-de-comanda-motor-cu-l298n-oky3195-ai0016-s190/pd/D7J1J7MBM/?ref=sponsored_products_search_f_b_1_1&recid=recads_1_dc8f0f354b800cd0e3d3055da10d769438027f1b3b39fa232b82788fb46d52e5_1776728290&aid=19bb0f69-340e-11f1-801c-06eaf0d4245d&oid=122320892&scenario_ID=1)|
| [4x TT DC Motors with Wheels](https://www.emag.ro/set-motoreductor-cu-roata-4-bucati-3874783591881/pd/DMXQ1DYBM/) | 4WD Rover Mobility | [~40 RON](https://www.emag.ro/set-motoreductor-cu-roata-4-bucati-3874783591881/pd/DMXQ1DYBM/)|
| [2x SG90 Micro-Servos with brackets](https://www.aliexpress.com/item/1005008956965692.html?spm=a2g0o.order_list.order_list_main.35.5d761802bvGp8x)| 180-degree Pan-Tilt Turret aiming | [~30 RON](https://www.aliexpress.com/item/1005008956965692.html?spm=a2g0o.order_list.order_list_main.35.5d761802bvGp8x)|
| [5V Laser Diode Module](https://www.aliexpress.com/item/1005007987491473.html?spm=a2g0o.order_list.order_list_main.29.5d761802bvGp8x) | Visual targeting payload | [~12 RON](https://www.aliexpress.com/item/1005007987491473.html?spm=a2g0o.order_list.order_list_main.29.5d761802bvGp8x)|
| Photoresistors & 10kΩ Resistors | Ambient light detection via ADC | Already owned |
| Protoboard, Headers, 4xAA Box | Physical assembly and power distribution | Already owned |

## Software

| Library | Description | Usage |
| --- | --- | --- |
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Core Hardware Abstraction Layer | Configures STM32 peripherals (I2C, SPI, UART, PWM, ADC) |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async Executor | Manages concurrent task spawning and non-blocking execution |
| [st7735-lcd](https://github.com/almindor/st7735-lcd) | Display driver for ST7735 | Pushing pixel data over SPI to the 1.44" screen |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Rendering the thermal grid and text onto the display |
| [heapless](https://github.com/japaric/heapless) | `no_std` data structures | Formatting JSON strings safely to send over UART |
| [defmt](https://github.com/knurling-rs/defmt) | Highly efficient logging framework | Hardware debugging over the Nucleo ST-LINK |

## Links
1. [Embassy Framework Documentation](https://embassy.dev/)
2. [STM32U545RE Reference Manual](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html)
3. [AMG8833 Documentation](https://cdn-learn.adafruit.com/downloads/pdf/adafruit-amg8833-8x8-thermal-camera-sensor.pdf)
