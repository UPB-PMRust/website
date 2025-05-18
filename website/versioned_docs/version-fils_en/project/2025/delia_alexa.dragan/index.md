# Music Player
A device that plays, stops and skips music.

:::info 

**Author**: Dragan Delia-Alexa \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/project-ddeliaffelia

:::

## Description

Simple, compact music player built using a Raspberry Pi Pico W, programmed in Rust. It will allow users to play,
pause, and skip songs stored on an SD card through a speaker, whose volume can be controlled using a
potentiometer. The LCD display is meant to show the playing song's title.

## Motivation

I tried to find a project idea that fits my interests and that I could enjoy working on. I have been passionate about music since I was little, and I can still recall the day my parents bought me a music player just like my brother's. Besides, I find it quite fun to work on a hardware project like this one.

## Architecture 

**Raspberry Pi Pico 2W:**
 - the centre of the project, controls all the other components
 - connections: SPI - SD card module, ADC - potentiometer, PWM - buttons, amplifier, I2C - LCD 

**LCD:**
 - displays the name of the song
 - connections: I2C - microcontroller 

**Amplifier:**
 - increases the quality and strength of the sound
 - connections: through wires - speaker, PWM - microcontroller 
 
**Speaker:**
 - emmits the music
 - connections: through wires - amplifier 

**Potentiometer:**
 - increases or decreases the volume
 - connections: ADC - microcontroller 

**SD card module:**
 - supplements storage space for the songs that will be played
 - connections: SPI - microcontroller 

![schematic.webp](schematic.webp)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

In this project, the core piece is a Raspberry Pi Pico 2W that gets its digital audio play-back from an SD card inserted in a module, interfaced via SPI protocol, and outputs it through a speaker, powered by a PAM8403D amplifier. A 1602 LCD displays via I2C protocol the currently playing song's title. The emmitted sound's volume is controlled by a manually adjustable variable resistor (potentiometer). There are also three buttons connected to the microcontroller's GPIO pins meant to play, pause and navigate through the songs. 

### Schematics

![KiCAD.webp](KiCAD.webp)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Breadboard kit](https://www.emag.ro/set-componente-electronice-led-uri-breadboard-830-puncte-componente-pentru-incepatori-compatibil-arduino-si-raspberry-pi-ouylaf-308-10149-409/pd/DH8RVLYBM/?ref=hdr-favorite_products) | Breadboard, resistors, cables, buttons | [71 RON](https://www.emag.ro/set-componente-electronice-led-uri-breadboard-830-puncte-componente-pentru-incepatori-compatibil-arduino-si-raspberry-pi-ouylaf-308-10149-409/pd/DH8RVLYBM/?ref=hdr-favorite_products) |
| [LCD Display](hhttps://www.waveshare.com/datasheet/LCD_en_PDF/LCD1602.pdf) | Displays song | [16 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html?search_query=LCD+1602+cu+Interfata+I2C+si+Backlight+Albastru&results=2) |
| [Amplifier](https://www.elecrow.com/download/PAM8403-datasheet.pdf?srsltid=AfmBOopAvrZSALh3lz5hqR4VpMRTfoAjwIrQ9JTq8bMPUaTYGRWY__sL) | Boosts audio | [4 RON](https://www.optimusdigital.ro/ro/audio-amplificatoare-audio/165-amplificator-in-miniatura.html) |
| [Speaker](https://ardushop.ro/ro/componente-discrete/1084-difuzor-1w-8ohm-50mm-6427854014900.html) | Plays audio | [4 RON](https://ardushop.ro/ro/componente-discrete/1084-difuzor-1w-8ohm-50mm-6427854014900.html) |
| [Potentiometer](https://projects.raspberrypi.org/en/projects/introduction-to-the-pico/11) | Changes the volume | [2 RON](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/901-rezistor-variabil-10k-wh148.html?search_query=Rezistor+Variabil+10k+WH148+%28Poten%C8%9Biometru%29&results=2) |
| [SD Card Module](https://www.optimusdigital.ro/ro/altele/98-modul-cititor-card-sd.html) | Reads SD card with songs | [6 RON](https://www.optimusdigital.ro/ro/altele/98-modul-cititor-card-sd.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://crates.io/crates/embassy-rp) | Embassy HAL for Raspberry Pi RP2040 | Used for running Rust on Raspberry Pi |
| [embassy-executor](https://crates.io/crates/embassy-executor) | async/await executor  | Used for asynchronous programming |
| [defmt](https://docs.rs/defmt/latest/defmt/) | Highly efficient logging framework | Used for debugging |
| [cortex-m](https://crates.io/crates/cortex-m) | Low-level access to Cortex-M processors | Used for accesing Cortex-M processor |
| [lcd-lcm1602-i2c](https://crates.io/crates/lcd-lcm1602-i2c) | Driver for LCD1602 | Used to control the display |
<!-- | [rodio](https://link) | USAGE | Used for tblablaba |
| [name](https://link) | USAGE | Used for tblablaba |
| [name](https://link) | USAGE | Used for tblablaba | -->

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Project Radio fm](https://www.optimusdigital.ro/ro/content/9-proiect-radio-fm)
