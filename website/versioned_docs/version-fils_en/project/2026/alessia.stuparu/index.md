# Adaptive Sunrise Alarm & Ambient Monitor
A web-managed, dual-architecture smart clock that features asynchronous timekeeping, environmental sensing and a light-and-sound wake-up sequence.

:::info

**Student:** Stuparu Alessia-Ștefania \
**GitHub Repository:** [https://github.com/UPB-PMRust-Students/fils-project-2026-alessiastuparu](https://github.com/UPB-PMRust-Students/fils-project-2026-alessiastuparu)

:::

## 1. Project Description

**I am implementing an internet-connected alarm that features a dashboard for management, an internal asynchronous software RTC, and a hardware interface that handles tactile button inputs while triggering a LED ring, a buzzer, and an MP3 audio module.**

The system is split into 2 parts: a Wi-Fi-enabled chip that handles networking and UI, while an ARM-Cortex-M33 manages hardware tasks such as turning on the LED ring, sensors, and audio outputs.

## 2. Motivation

I chose to do this project because I wanted to build a practical, daily-use tool while learning about cross-architecture communication and asynchronous embedded Rust. By using the RP2040 and STM32U5, I can better grasp the concepts of memory safety, state synchronization and custom protocol design.

## 3. Architecture

The system relies on two microcontrollers that work at the same time and communicate via a hardware UART bridge:

* **The Raspberry Pi Pico 2W:** Represents the networking layer as it hosts an Embassy-based web server to receive user commands via HTTP.
* **STM32:** Represents the control layer as it manages alarm state, local timekeeping, physical button inputs, and peripheral driving.
* **Communication Protocol:** To keep the two chips in sync, I am using the postcard serialization format so that commands sent from the Pico W to the STM32 are interpreted without data corruption or memory errors.

![Architecture Diagram](./architecture.drawio.svg)

## 4. Log

### Weeks 1-7
* Came up with the idea for the project and did needed research.
* Finished the project proposal and selected hardware components.
* Made final changes to the hardware part and ordered components.

### Week 8 
* Set up the Cargo workspace for both of the architectures so it handles cross-compiling.
* Designed the behavior of the chips and what each of them handles.
* Created a shared directory library so that specific commands are understood by both chips.

### Week 9
* Programmed the Pico W so that it acts like a Wi-Fi server and created the HTML/JS website.
* Established the UART serial connection so the Pico W can send data to the STM32.
* Programmed the STM32 to multitask. It ticks a clock in the background while listening for new commands, all while using a Mutex to share memory between tasks.
* Completed the Documentation Milestone.

## 5. Hardware

The system uses a Nucleo STM32U545RE-Q as the main controller, a Raspberry Pi Pico 2W for Wi-Fi networking and web hosting, a 1.8" TFT LCD for time and temperature visualization, a BME280 sensor for environmental monitoring, a WS2812B LED ring for sunrise simulation, an active buzzer and a DFPlayer Mini with a 3W speaker for audible alarms, and tactile buttons for manual hardware control.

## 6. Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/raspberry-pi/13247-raspberry-pi-pico-w.html) | Wi-Fi network and web server hub | 39.66 RON |
| [GroundStudio BME280 3V3](https://ardushop.ro/ro/electronica/1125-modul-senzor-de-presiune-barometrica-si-temperatura-bme280.html) | Temperature and Humidity monitoring (I2C) | 32.67 RON |
| [DFPlayer Mini MP3 Player](https://www.aliexpress.com/) | Decodes and plays MP3 audio files for the alarm | 13.98 RON |
| [Mini Speaker 3W 8 Ohm](https://www.aliexpress.com/) | Audio output for the DFPlayer | 16.67 RON |
| [MicroSD Card (8GB-32GB)](#) | Stores MP3 files for the DFPlayer | Already owned |
| [Active Buzzer KY-012](https://www.aliexpress.com/) | Backup audible alarm trigger | 13.21 RON |
| [1.8" LCD TFT SPI ST7735](https://www.aliexpress.com/) | Digital clock and UI display | 19.18 RON |
| [WS2812B LED Ring (16 LEDs)](https://www.aliexpress.com/) | RGB LED ring for sunrise simulation | 35.80 RON |
| [Tactile Buttons 6x6x5mm](https://www.aliexpress.com/) | Physical inputs for snooze and alarm cancel | 17.62 RON |
| [Breadboard 830 puncte MB-102](https://ardushop.ro/ro/electronica/10-breadboard-830-puncte-mb-102.html) | Hardware prototyping base | 21.18 RON |
| [MB102 Breadboard Power Supply](https://www.optimusdigital.ro/ro/surse-de-alimentare/28-sursa-de-alimentare-pentru-breadboard.html) | Regulates 3.3V/5V power for the breadboard rails | 4.69 RON |
| [9V 1A Power Adapter](https://ardushop.ro/ro/home/83-sursa-alimentare-9v-1a.html) | Main wall power source for the breadboard | 8.99 RON |
| [Dupont Wires (M-M & M-F)](https://www.optimusdigital.ro/) | Hardware interconnections | 19.24 RON |
| [Nucleo STM32U545RE-Q](https://ro.farnell.com/stmicroelectronics/nucleo-u545re-q/dev-board-32bit-arm-cortex-m33/dp/3953770) | Main logic and peripheral controller | Already owned |

## 7. Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.rs/embassy-stm32/) | Hardware abstraction for STM32 | Used for configuring SPI, I2C, UART, and GPIO (for the tactile buttons) on the Nucleo |
| [embassy-rp](https://docs.rs/embassy-rp/) | Hardware abstraction for Pico 2W | Used for configuring the RP2040/RP2350 peripherals |
| [postcard](https://docs.rs/postcard/) | no_std message serialization | Used to safely pack and unpack UART network commands between the two chips |
| [cyw43](https://docs.rs/cyw43/) | Pico W Wi-Fi driver | Used to establish the wireless network connection |
| [embassy-net](https://docs.rs/embassy-net/) | Asynchronous network stack | Used to serve the HTTP dashboard to the user |
| [st7735-lcd](https://docs.rs/st7735-lcd/) | SPI display driver | Used for drawing the clock UI and temperature to the 1.8" TFT screen |
| [bme280-rs](https://docs.rs/bme280-rs/) | I2C sensor driver | Used to read room temperature and humidity |
| [smart-leds](https://docs.rs/smart-leds/) | Addressable LED API | Used to control the WS2812B ring for the sunrise simulation |
| [dfplayer](https://docs.rs/dfplayer/) | UART driver for DFPlayer Mini | Used to trigger MP3 audio playback for the alarm |

## 8. Links

1. [Embassy Framework Documentation](https://embassy.dev/)
2. [Postcard Protocol Specification](https://postcard.jamesmunns.com/)
3. [The Rust on Embedded Devices Book](https://docs.rust-embedded.org/book/)
4. [DFPlayer Mini Manual & AT Commands](https://wiki.dfrobot.com/DFPlayer_Mini_SKU_DFR0299)
5. [ST7735 Display Datasheet](https://www.displayfuture.com/Display/datasheet/controller/ST7735.pdf)