
# Automatic Music Player and Advertising System
An automated audio solution that blends background music with scheduled announcements using the RP2350 and Rust.

:::info 

**Author**: Nedelcu Diana Ioana \
**GitHub Project Link**: [https://github.com/UPB-PMRust-Students/fils-project-2026-dianaioana05](https://github.com/UPB-PMRust-Students/fils-project-2026-dianaioana05)

:::

## Description

The project consists of an automated audio playback system. It plays a continuous playlist of MP3 files from a microSD card. At predefined intervals (e.g., every 10–15 minutes), the system finishes the current song, switches to a folder for advertisements/announcements, plays one file, and then returns to the music playlist.


## Motivation

I chose this project because I wanted to build something practical that you see every day, like the audio systems in **supermarkets or gyms** that play background music and periodically interrupt it for announcements or ads. 

Usually, these systems are either overpriced proprietary hardware or just a PC that’s overkill for the task. Using the **RP2350** and **Rust** is a great way to try and build a "professional" version that is:
* **Reliable:** It won't crash or stutter like a basic media player might.
* **Automatic:** It handles the timing and switching between folders by itself, which is exactly how these systems work in the real world.


## Architecture 

The system follows a producer-consumer architecture managed by the Embassy executor:
 - **Main Task (Producer):** Reads raw WAV data from the MicroSD card via SPI.
 - **I2S Driver (Consumer):** Uses the Pico's **PIO** to stream the PCM data to the PCM5102A DAC. It uses DMA (Direct Memory Access) to ensure playback doesn't stutter while the CPU is busy reading from the SD card.
 - **Timer Task:** Tracks the 10-15 minute intervals and signals the main task to switch directories.
 - **User Interface:** Handles GPIO interrupts from push buttons for volume adjustment and track skipping.
![Design](diagram.drawio.svg)

## Log

### Week 5 - 11 May
- Established project topic and finalized the hardware list.

### Week 12 - 18 May
- Decided to switch from MP3 to WAV to focus on high-throughput data handling.

### Week 19 - 25 May
- Ordered components

## Hardware

The project uses the **Raspberry Pi Pico 2W** (RP2350), which provides more processing power for audio handling. The **PCM5102A** DAC provides high-quality stereo output via an I2S interface.


### Software Data Flow
1. **File System Task:** Reads raw WAV data chunks from the SD card into a shared memory buffer.
2. **Audio Task:** Consumes the buffer and uses **DMA** to stream data to the PIO state machine, ensuring the I2S timing remains perfect.
3. **Manager Task:** An async loop that tracks the 10-15 minute timer for advertisements and handles button interrupts to change the playback state.
### Schematics

*(To be added)*

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| Raspberry Pi Pico 2W | Main Microcontroller (RP2350) | 39.66 lei |
| PCM5102A Audio DAC | I2S Audio Output | 94.99 lei |
| MicroSD Card Module| Storage Interface | 24.39 lei |
| Push Buttons | Volume/Skip Controls | 3.60 lei (10pcs) |
| Resistor set | Pull-ups and protection | 7.00 lei |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy) | HAL for RP2350 | Peripheral control and PIO/DMA management |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async Executor | Managing concurrent audio streaming tasks |
| [embedded-sdmmc](https://github.com/rust-embedded-community/embedded-sdmmc-rs) | FAT File System | Reading WAV files from the SD Card |
| [pio](https://crates.io/crates/pio) | PIO Assembler | Creating the I2S protocol state machine |

## Links

1. [PCM5102A Datasheet](https://www.ti.com/product/PCM5102A)
2. [Pico PIO I2S Example](https://github.com/raspberrypi/pico-examples/tree/master/pio/i2s)
3. [Embassy-rs PIO Documentation](https://embassy.dev/book/dev/pio.html)