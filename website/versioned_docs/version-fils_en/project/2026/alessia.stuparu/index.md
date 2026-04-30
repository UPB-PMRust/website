# Adaptive Sunrise Alarm & Ambient Monitor
A web-managed, dual-architecture smart clock that features asynchronous timekeeping, environmental sensing and a light-and-sound wake-up sequence.

:::info

**Student:** Stuparu Alessia-Ștefania \
**GitHub Repository:** [https://github.com/UPB-PMRust-Students/fils-project-2026-alessiastuparu](https://github.com/UPB-PMRust-Students/fils-project-2026-alessiastuparu)

:::

## 1. Project Description

**I am implementing an internet-connected alarm that features a dashboard for management, an internal asynchronous software RTC and a hardware interface that handles tactile button inputs while triggering a LED ring, a buzzer, and an MP3 audio module.**

The system is split into 2 parts: a Wi-Fi-enabled chip that handles networking and UI, while an ARM-Cortex-M33 manages hardware tasks such as turning on the LED ring, sensors and audio outputs.

## 2. Motivation

I chose to do this project because I wanted to build a practical, daily-use tool while learning about cross-architecture communication and asynchronous embedded Rust. By using the RP2350 and STM32U5, I can better grasp the concepts of memory safety, state synchronization and custom protocol design.

## 3. Architecture

The system relies on two microcontrollers that work at the same time and communicate via a hardware UART bridge:

* **The Raspberry Pi Pico 2W:** Represents the networking layer as it hosts an Embassy-based web server to receive user commands via HTTP.
* **STM32:** Represents the control layer as it manages alarm state, local timekeeping, physical button inputs, and peripheral driving.
* **Communication Protocol:** To keep the two chips in sync, I am using the postcard serialization format so that commands sent from the Pico 2W to the STM32 are interpreted without data corruption or memory errors.

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
* Programmed the Pico 2W so that it acts like a Wi-Fi server and created the HTML/JS website.
* Established the UART serial connection so the Pico W can send data to the STM32.
* Programmed the STM32 to multitask. It ticks a clock in the background while listening for new commands, all while using a Mutex to share memory between tasks.
* Completed the Documentation Milestone.

## 5. Hardware

The system uses a Nucleo STM32U545RE-Q as the main controller, a Raspberry Pi Pico 2W for Wi-Fi networking and web hosting, a 1.8" TFT LCD for time and temperature visualization, a BME280 sensor for environmental monitoring, a WS2812B LED ring for sunrise simulation, an active buzzer and a DFPlayer Mini with a 3W speaker for audible alarms and tactile buttons for manual hardware control.

## 6. Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| [Raspberry Pi Pico 2W](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html?search_query=Raspberry+Pi+Pico+2W&results=24) | Wi-Fi network and web server hub | 39.66 RON |
| [GroundStudio BME280 3V3](https://ardushop.ro/en/groundstudio/2450-groundstudio-bme280-3v3-module-6427854034793.html) | Temperature and Humidity monitoring (I2C) | 32.67 RON |
| [DFPlayer Mini MP3 Player](https://www.aliexpress.com/item/1005008209885534.html?spm=a2g0o.productlist.main.7.4c1c4FK44FK4XX&algo_pvid=0fb614ed-e3ac-40f2-ba31-52c6502595c4&algo_exp_id=0fb614ed-e3ac-40f2-ba31-52c6502595c4-6&pdp_ext_f=%7B"order"%3A"1700"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%2132.00%2132.00%21%21%2149.13%2149.13%21%40211b80d117771077289833426ef6f6%2112000056419754306%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=Ny97ZaiHhzA9&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005008209885534%7C_p_origin_prod%3A) | Decodes and plays MP3 audio files for the alarm | 13.98 RON |
| [Mini Speaker 3W 8 Ohm](https://www.aliexpress.com/item/1005008392295953.html?spm=a2g0o.productlist.main.11.11ae62a1YVfZIQ&algo_pvid=f4d06e25-0545-4932-af8d-c3b21c650692&algo_exp_id=f4d06e25-0545-4932-af8d-c3b21c650692-10&pdp_ext_f=%7B"order"%3A"1088"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%216.41%216.82%21%21%211.44%211.53%21%40211b81a317771076342933384ebe5a%2112000044833084511%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=2LjlMxrqiXmN&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005008392295953%7C_p_origin_prod%3A) | Audio output for the DFPlayer | 16.67 RON |
| [MicroSD Card (8GB-32GB)](https://altex.ro/carduri-memorie-telefon/cpl/filtru/capacitate-stocare-1062/16-gb/) | Stores MP3 files for the DFPlayer | 32 RON |
| [Active Buzzer KY-012](https://www.aliexpress.com/item/1005009534810058.html?spm=a2g0o.productlist.main.7.4113645679z5Ge&algo_pvid=fd195258-8206-474a-9ee8-752f3f622379&algo_exp_id=fd195258-8206-474a-9ee8-752f3f622379-6&pdp_ext_f=%7B"order"%3A"29"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%2114.54%2115.25%21%21%2122.32%2123.41%21%40211b65de17771074461184264e640c%2112000049389570708%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=GVY0mnFDciHV&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005009534810058%7C_p_origin_prod%3A) | Backup audible alarm trigger | 13.21 RON |
| [1.8" LCD TFT SPI ST7735](https://www.aliexpress.com/item/1005010340668858.html?spm=a2g0o.productlist.main.11.373d7141JG1sCw&algo_pvid=fc5bec33-472e-49a4-95e5-a7ea4fe25e22&algo_exp_id=fc5bec33-472e-49a4-95e5-a7ea4fe25e22-10&pdp_ext_f=%7B"order"%3A"144"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%2118.04%2118.84%21%21%2127.70%2128.92%21%40211b61ae17771075032038758ee764%2112000052038900479%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=W5E3xfrO5AW5&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005010340668858%7C_p_origin_prod%3A) | Digital clock and UI display | 19.18 RON |
| [WS2812B LED Ring (16 LEDs)](https://www.aliexpress.com/item/1005002304069758.html?spm=a2g0o.productlist.main.56.34carNFurNFuge&algo_pvid=4dc903b0-1d0d-4602-b019-68a0c7939cca&algo_exp_id=4dc903b0-1d0d-4602-b019-68a0c7939cca-53&pdp_ext_f=%7B"order"%3A"7327"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%2117.09%2117.09%21%21%2126.24%2126.24%21%40211b615317771078492895180ee14d%2112000019997051113%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=9QqH51uveYAP&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A1005002304069758%7C_p_origin_prod%3A) | RGB LED ring for sunrise simulation | 35.80 RON |
| [Tactile Buttons 6x6x5mm](https://www.aliexpress.com/item/32873398324.html?spm=a2g0o.productlist.main.1.1aa9ccfOccfODk&algo_pvid=f568a48b-c18d-424f-b9af-4eefcf837f77&algo_exp_id=f568a48b-c18d-424f-b9af-4eefcf837f77-0&pdp_ext_f=%7B"order"%3A"386"%2C"eval"%3A"1"%2C"fromPage"%3A"search"%7D&pdp_npi=6%40dis%21RON%2120.04%2120.04%21%21%214.50%214.50%21%402103846917771075907586413e3878%2165602884059%21sea%21RO%212785812042%21ABX%211%210%21n_tag%3A-29910%3Bd%3A479b0467%3Bm03_new_user%3A-29895&curPageLogUid=hkBMiRZu80Dr&utparam-url=scene%3Asearch%7Cquery_from%3A%7Cx_object_id%3A32873398324%7C_p_origin_prod%3A) | Physical inputs for snooze and alarm cancel | 17.62 RON |
| [Breadboard 830 puncte MB-102](https://ardushop.ro/en/electronics/2297-breadboard-830-points-mb-102-6427854012265.html) | Hardware prototyping base | 21.18 RON |
| [MB102 Breadboard Power Supply](https://www.optimusdigital.ro/ro/electronica-de-putere-stabilizatoare-liniare/61-sursa-de-alimentare-pentru-breadboard.html?search_query=Sursa+de+alimentare+pentru+breadboard&results=6) | Regulates 3.3V/5V power for the breadboard rails | 4.69 RON |
| [9V 1A Power Adapter](https://ardushop.ro/en/electronics/1144-power-supply-adapter-9v-1a-6427854016027.html) | Main wall power source for the breadboard | 8.99 RON |
| [Dupont Wires (M-M & M-F)](https://www.optimusdigital.ro/ro/fire-fire-mufate/214-fire-colorate-mama-mama-10p.html?search_query=Fire+Colorate+Mama-Tata+%2810p%29+20+cm&results=6) | Hardware interconnections | 19.24 RON |
| [Nucleo STM32U545RE-Q](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&srsltid=AfmBOoqXGPqxB13PvgWCZHddM_5qXjGqEcsqiVKHqEcfBJFRI1cYmjSH&_gl=1*1q1geh*_ga*dW5kZWZpbmVk*_ga_15W4STQT4T*dW5kZWZpbmVk*_ga_1KQLCYKRX3*dW5kZWZpbmVk) | Main logic and peripheral controller | 140 RON |

## 7. Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://docs.embassy.dev/embassy-stm32/0.6.0/stm32u545re/index.html) | Hardware abstraction for STM32 | Used for configuring SPI, I2C, UART, and GPIO (for the tactile buttons) on the Nucleo |
| [embassy-rp](https://docs.embassy.dev/embassy-rp/0.9.0/rp235xa/index.html) | Hardware abstraction for Pico 2W | Used for configuring the RP2350 peripherals |
| [postcard](https://docs.rs/postcard-bytes/latest/postcard/) | no_std message serialization | Used to safely pack and unpack UART network commands between the two chips |
| [cyw43](https://docs.rs/cyw43/latest/cyw43/) | Pico W Wi-Fi driver | Used to establish the wireless network connection |
| [embassy-net](https://docs.rs/embassy-net/latest/embassy_net/index.html) | Asynchronous network stack | Used to serve the HTTP dashboard to the user |
| [st7735-lcd](https://docs.rs/st7735-lcd/latest/st7735_lcd/) | SPI display driver | Used for drawing the clock UI and temperature to the 1.8" TFT screen |
| [bme280-rs](https://docs.rs/bme280-rs/latest/bme280_rs/) | I2C sensor driver | Used to read room temperature and humidity |
| [smart-leds](https://docs.rs/smart-leds/latest/smart_leds/) | Addressable LED API | Used to control the WS2812B ring for the sunrise simulation |
| [dfplayer](https://docs.rs/dfplayer-async/latest/dfplayer_async/) | UART driver for DFPlayer Mini | Used to trigger MP3 audio playback for the alarm |

## 8. Links

1. [Embassy Framework Documentation](https://embassy.dev/book/)
2. [Postcard Protocol Specification](https://docs.rs/postcard/0.6.0/postcard/)
3. [The Rust on Embedded Devices Book](https://docs.rust-embedded.org/book/)
4. [DFPlayer Mini Manual & AT Commands](https://picaxe.com/docs/spe033.pdf)
5. [ST7735 Display Datasheet](https://www.alldatasheet.com/datasheet-pdf/pdf/326213/SITRONIX/ST7735.html)