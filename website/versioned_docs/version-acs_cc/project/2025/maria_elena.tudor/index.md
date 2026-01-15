# 2000s Romanian Pop Music Player
A coin-operated music player that plays Romanian pop hits from the 2000s and displays track data on an OLED screen.

:::info

**Author**: Maria-Elena Tudor \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-mariacthi

:::

## Description

This project combines the nostalgic feel of classic jukeboxes with a curated playlist of Romanian pop music from the 2000s. Users insert a coin to unlock playback; track information (title and artist) is shown on an OLED display. Physical buttons allow play/pause, previous/next, volume control and shuffling the playlist.

## Motivation

At every party I attend, Romanian hits from the 2000s always come on—and I love that era’s pop sound. I’ve also always been fascinated by the jukeboxes featured in movies. Merging these two ideas into a single device seemed like a fun, tangible way to share my favorite decade of music with friends.

## Architecture

![Schematic diagram](diagrama.webp)

The player reads WAV files from an SD card, decodes and streams audio via I²S implemented in PIO, displays track information on an SSD1306 OLED, and supports play/pause, volume control, next/previous, and shuffle buttons. A simple ultrasonic “coin” sensor starts playback when a coin is detected.

  **Microcontroller Raspberry Pi Pico 2W** runs embedded Rust firmware, coordinating all peripherals.
  - **Tasks**:
	- Handles peripheral I/O (SPI, GPIO, I2S)
	- Manages shuffle queue and WAV file parsing
	- Debounces buttons & validates coin input

 **OLED Display** shows metadata over I2C (song title, artist name).

 **Tactile Buttons** read GPIO inputs for user controls.

 **Coin Validator Sensor** detects insertion of a valid coin/token and signals the microcontroller to unlock playback.

 **Audio Amplifier + Speaker** decodes WAV through I2S and streams the audio.

 **SD Card Module** stores WAV files; accessed over SPI.

## Log

### Week 5 - 11 May

All components arrived and I started working on basic test codes to make sure everything is in working condition.
I managed to do the sensor and the display this week.

### Week 12 - 18 May

I managed basic reading from the SD card and sending audio output to a speaker, then I started working on making all the components interact with one another and work at the same time. Lastly, I added the buttons.

### Week 19 - 25 May
I worked on being able to go through the playlist of songs as I previously could only listen to one at a time. I also handled all the problems that I could find in the code and tried optimising the audio output as much as possible. I started working on a case for the project and added new songs.

## Hardware

The entire system is built around a Raspberry Pi Pico 2 W running embedded Rust, which reads .WAV files off a microSD card, drives a compact SSD1306 OLED screen for track info, and monitors both tactile buttons and a coin‐validator sensor for user input. Audio is streamed to an amplifier and speaker, while everything is powered from a laptop and prototyped on a breadboard for easy iteration.
Youtube link for the hardware milestone: https://www.youtube.com/shorts/a_Oh2vH7xY4

![Hardware Milestone](pic1.webp)
![Hardware Milestone](pic2.webp)

### Schematics

Schematic after Hardware Milestone
![Kicad Schematic after Hardware Milestone](kicad_sch.svg)

### Bill of Materials

| Device                                                  | Usage                        | Price                           |
|---------------------------------------------------------|------------------------------|---------------------------------|
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) | The microcontroller |  [39.66 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html) | Debug |  [39.66 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [Display OLED 128x64 - 0.96" I2C](https://www.micros.com.pl/mediaserver/OLED12864-0.96-W-2_0001.pdf) | User interface display       | [16.35 RON](https://sigmanortec.ro/Display-OLED-0-96-I2C-IIC-Albastru-p135055705) |
| [MicroSD Card Module](https://github.com/GroundStudio/GroundStudio_MicroSD_module/tree/main/Hardware) | Storage for MP3 files        | [7.14 RON](https://ardushop.ro/ro/module/1553-groundstudio-microsd-module-6427854023056.html) |
| [Card MicroSD 32Gb - clasa 10](https://ardushop.ro/ro/raspberry-pi/636-card-microsd-32gb-clasa-10-6427854007919.html) | Memory Card | [42.89 RON](https://ardushop.ro/ro/raspberry-pi/636-card-microsd-32gb-clasa-10-6427854007919.html) |
| [Amplificator 3W I2S - MAX98357A](https://ardushop.ro/ro/module/1549-amplificator-3w-i2s-max98357a-clasa-d-6427854022967.html)   | Audio output | [17.03 RON](https://ardushop.ro/ro/module/1549-amplificator-3w-i2s-max98357a-clasa-d-6427854022967.html) |
| [Difuzor 50mm - 2W - 32ohm](https://ardushop.ro/ro/electronica/1962-difuzor-50mm-2w-32ohm-6427854029898.html)   | Audio output device          | [4.82 RON](https://ardushop.ro/ro/electronica/1962-difuzor-50mm-2w-32ohm-6427854029898.html)  |
| [Modul senzor Ultrasonic - detector distanta HC-SR04](https://ardushop.ro/ro/electronica/2289-modul-senzor-ultrasonic-detector-distanta-hc-sr04-6427854030726.html) | Coin sensor | [9.75 RON](https://ardushop.ro/ro/electronica/2289-modul-senzor-ultrasonic-detector-distanta-hc-sr04-6427854030726.html) |
| Electronic components: buttons, wires, pins and Breadboard | Electronic Components | 100 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy](https://embassy.dev/) | Async executor and HAL integration for embedded | Provides task scheduling, timers, and async support   |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) |  Hardware Abstraction Layer for embedded Rust | Unifies SPI, I2C, GPIO and PWM drivers |
| [display-interface](https://docs.rs/display-interface/latest/display_interface/) | Generic display API | Abstracts over SSD1306 driver for easy text & graphics    |
| [ssd1306](https://github.com/rust-embedded-community/ssd1306) |  SSD1306 OLED display driver | Initializes display, draws frames, and manages framebuffer |
| [rand](https://docs.rust-embedded.org/cortex-m-rt/0.6.0/rand/index.html) | Random number generation | Implements shuffle logic for playback queue |
| [embassy-executor](https://docs.rs/embassy-executor/latest/embassy_executor/) | 	Task executor for async Rust on embedded |  embedded	Spawns and runs the #[embassy_executor::task] tasks |
| [embassy-time](https://docs.rs/embassy-time/latest/embassy_time/) | Async timing utilities | 	Provides Timer, Delay, and Instant for non-blocking delays |
| [emabssy_rp](https://docs.rs/embassy-rp/latest/embassy_rp/) | RP2040 board support for Embassy | 	HAL for GPIO, SPI, I2C, PIO, DMA on the RP2040 microcontroller |
| [static-cell](https://crates.io/crates/static-cell) | Runtime-initialized static storage | Safely initializes globals like BUS and FS_MGR at runtime |
| [embedded_sdmmc](https://docs.rs/embedded-sdmmc/latest/embedded_sdmmc/) | FAT filesystem and SD-card driver | Interfaces with SD card over SPI, manages volumes and files |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics for embedded displays| Renders text and primitives onto the OLED buffer |

## Links

1. https://www.hackster.io/Ramji_Patel/raspberry-pi-pico-and-button-321059
2. https://embeddedcomputing.com/technology/processing/interface-io/simple-mp3-audio-playback-with-raspberry-pi-pico
3. https://pmrust.pages.upb.ro/docs/acs_cc/category/lab
4. https://esp32.implrust.com/sdcard/read-sdcard.html
5. https://orionrobots.co.uk/2023/09/04/i2s-playback.html