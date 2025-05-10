# MP3 Player
An MP3 player with button controls and a display that shows the current track, play/pause status, and volume.

:::info 

**Author**: Nistor Ioan-Mario \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-marionistor

:::

## Description

This MP3 player, built around the Raspberry Pi Pico 2 microcontroller, enables playback of MP3 files stored on a microSD card. It features five tactile buttons for intuitive control, supporting play, pause, song navigation (next/previous), and volume adjustment. A 1.3" I2C OLED display delivers real-time updates, showing the current track title and playback status. The system employs a DFPlayer Mini module for MP3 playback, interfaced with the Pico via UART, complemented by a PAM8403 amplifier for improved audio output. Power is supplied by a 5V power bank, ensuring dependable and portable performance.

## Motivation

This project draws inspiration from the nostalgia of early portable MP3 players, known for their compact screens and tactile buttons, aiming to revive their charm with modern technology. Using the Raspberry Pi Pico 2, DFPlayer Mini, and OLED display, the goal is to create a functional player that captures the simplicity and joy of those classic devices.

## Architecture 

![Diagram](mp3_player.svg)

### Connections
- **Microcontroller ↔ DFPlayer Mini**: UART (GPIO0 TX → DFPlayer RX, GPIO1 RX → DFPlayer TX) for playback commands.
- **Microcontroller ↔ Display**: I2C (GPIO4 SDA, GPIO5 SCL) for displaying track info, with OLED VCC connected to 5V and GND to ground.
- **Microcontroller ↔ Buttons**: GPIO6–10 as inputs for the buttons.
- **DFPlayer Mini ↔ PAM8403**: DFPlayer audio output (L/R) to PAM8403D input for amplification.
- **PAM8403 ↔ Speaker**: PAM8403D output connected to the 4Ω/3W speaker.
- **Power**: 5V power bank connected to VBUS (Pin 40) of the Raspberry Pi Pico 2, supplying 5V directly to the OLED VCC, DFPlayer Mini VCC, and PAM8403D VDD.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- **Microcontroller**: Raspberry Pi Pico 2 manages logic, UART communication with DFPlayer, and I2C with OLED.
- **Audio Module**: DFPlayer Mini plays MP3s from a microSD card.
- **Display**: 1.3" I2C OLED (SSD1306) shows track info and playback status, powered at 5V as per specifications (3.3V-5V range).
- **Input**: Five tactile buttons (SW1–SW5) for play/pause, next, previous, and volume control.
- **Audio Amplifier**: PAM8403D boosts the sound output to drive the speaker.
- **Speaker**: 4Ω/3W speaker connected to the PAM8403D for audio playback.

### Schematics

![Schematic](proiect_pm.svg)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Main microcontroller running the firmware | [28 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13266-raspberry-pi-pico-2.html) |
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Debugging | [40 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| [DFPlayer Mini](https://wiki.dfrobot.com/DFPlayer_Mini_SKU_DFR0299) | MP3 decoder and audio playback module | [14 RON](https://www.optimusdigital.ro/ro/audio/1484-modul-mp3-player-in-miniatura-dfplayer-mini.html) |
| [SanDisk 32GB MicroSD Card](https://documents.westerndigital.com/content/dam/doc-library/en_us/assets/public/sandisk/product/memory-cards/ultra-uhs-i-microsd/data-sheet-ultra-uhs-i-microsd.pdf) | Stores the MP3 files for playback | [Already owned](https://www.around.ro/card-de-memorie-microsd-sandisk-32gb-adaptor-sd-class-4-40414) |
| [PAM8403](https://www.elecrow.com/download/PAM8403-datasheet.pdf) | Amplifies audio signal for the speaker | [3 RON](https://www.optimusdigital.ro/ro/audio-amplificatoare-audio/614-modul-mini-amplificator-pam8403-in-clasa-d-stereo-3-w.html) |
| [4Ω/3W Speaker](https://docs.rs-online.com/6abb/A700000011931840.pdf) | Outputs the audio playback | [11 RON](https://tehnoelectric.ro/difuzoare/1307-12159-difuzor-50x18mm-4-3w.html) |
| [OLED 1.3" Display](https://cdn.awsli.com.br/945/945993/arquivos/1.3inch_IIC_OLED_Module_MC130GX&MC130VX_User_Manual_EN.pdf) | Displays song info and player status | [30 RON](https://www.electrospot.ro/display-oled-13-alb-128x64) |
| [HAMA PD10 201719](https://altex.ro/baterie-externa-hama-pd10-201719-20000mah-2xusb-c-1xusb-a-gri/cpd/AIS201719/) | Provides power to the breadboard setup | [Already owned](https://altex.ro/baterie-externa-hama-pd10-201719-20000mah-2xusb-c-1xusb-a-gri/cpd/AIS201719/) |
| [Buttons](https://www.farnell.com/datasheets/2343587.pdf) | User input for controlling playback and volume | [5 x 0.36 RON](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1119-buton-6x6x6.html) |
| [Resistors](https://www.farnell.com/datasheets/3111754.pdf) | Resistors used for buttons | [10 x 0.10 RON](https://www.optimusdigital.ro/ro/componente-electronice-rezistoare/853-rezistor-025w-51k.html) |
| [Breadboard](https://www.busboard.com/documents/datasheets/BPS-DAT-(BB830)-Datasheet.pdf) | For prototyping the entire circuit | [2 x 10 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html) |
| Wires | Connect all components on breadboard | 19 RON |
| Total | - | 168 RON |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://crates.io/crates/embassy-rp) | Embassy framework for RP2350 | Used to control peripherals like UART, I2C, GPIO, and timers on the Pico 2 |
| [ssd1306](https://github.com/rust-embedded-community/ssd1306) | Driver for SSD1306 OLED displays via I2C | Used to control the 1.3" I2C OLED display |
| [embassy-time](https://crates.io/crates/embassy-time) | Async timers and delays | Used for implementing delays and scheduling tasks |
| [dfplayer-async](https://github.com/1-rafael-1/dfplayer-async/) | Async Rust driver for DFPlayer Mini MP3 module | Provides MP3 playback control over UART using async calls |
| [embassy-executor](https://crates.io/crates/embassy-executor) | Async task executor for embedded systems | Manages and schedules async tasks on the microcontroller |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library for embedded systems | Used to draw text, icons, and other UI elements on the OLED display |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware Abstraction Layer for embedded devices | Interface layer used by all driver crates for GPIO, I2C, UART, etc. |
| [defmt](https://github.com/knurling-rs/defmt) | Lightweight logging framework for embedded Rust | Used for debug logging during development |
| [panic-probe](https://crates.io/crates/panic-probe) | Minimal panic handler for embedded Rust | Provides panic messages via logging for embedded debug purposes |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [ MP3 Player made using Arduino ](https://www.electronics-lab.com/project/mp3-player-using-arduino-dfplayer-mini/)
2. [ Connecting Audio Amplifier to DF Player Mini ](https://www.youtube.com/watch?v=eYF1Fa0uTDU&list=WL)
2. [ Ways to power the breadboard ](https://www.zabeducation.com/project/physics/breadboardPowerSupplies.html)