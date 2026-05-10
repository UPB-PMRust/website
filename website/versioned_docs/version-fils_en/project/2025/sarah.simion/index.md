# Lil-console
My project is named Lil-Console and it's a little portable retro gaming device.

:::info

**Author**: Simion Sarah Stefania \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/project-Dydys12)

:::

## Description

The goals of this project is to use the Raspberry Pi Pico 2W microcontroller to build a basic retro portable console. A joystick and a few buttons are used for movement, navigation and selection. The games will have a classic gaming vibe, for this I choose the games "Pong" and "Snake", along with a simple menu system.

## Motivation

I choose this idea because I am currently very passionate about games and I enjoy playing games on both computer and consoles, but I haven't experienced the retro gaming consoles world and thought that if it is possible to make one myself now I would really enjoy the making of it and definitely the using of it.  

## Architecture 

![Alt text](pic1.webp)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

I attached all the components and made a suitable kicad schmatic.

### Week 12 - 18 May

I added a debugger and I tried testing the wiring to the display with a simple code of the spi connection.

### Week 19 - 25 May

I added all components and I started working on the games. After I finished the snake game, the most problematic error I encountered was setting the thresholds of the joystick, just to realize the x axys wasnt working so I had to change the joystick.

## Hardware

This are the hardware parts that I would like to use:

- Raspberry Pi Pico 2W (main microcontroller) + one used as a debugger
- LCD SPI 1.8"(128x160) Display
- PS2 Joystick Module and a button


![Alt text](pic3.webp)

![Alt text](pic4.webp)


### Schematics

![Alt text](schematic.svg)

### Bill of Materials


| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2W](https://datasheets.raspberrypi.com/picow/pico-2-w-pinout.pdf) | Main microcontroller board | [40 lei](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html?search_query=raspberry+pi+pico&results=36) |
| [LCD SPI 1.8" (128x160) Display](https://www.openimpulse.com/blog/wp-content/uploads/wpsc/downloadables/1.8-SPI-LCD-Module-datasheet.pdf) | Used to visually display information or interface elements. | [29 lei](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/1311-modul-lcd-spi-de-18-128x160.html?search_query=Modul+LCD+SPI+de+1.8%27%27+%28128x160%29&results=3) |
| [PS2 Joystick Module](https://naylampmechatronics.com/img/cms/Datasheets/000036%20-%20datasheet%20KY-023-Joy-IT.pdf) | Provides directional input and control options. | [4 lei](https://www.robofun.ro/componente/modul-joystick-ps2.html) |
| [Simple PCB Boards] | Base for soldering and connecting components. | [had at home] |
| [Buttons](https://components101.com/switches/push-button) | For control options | [2x 2 lei](https://www.optimusdigital.ro/ro/butoane-i-comutatoare/1114-buton-cu-capac-rotund-rou.html?search_query=%09Buton+cu+Capac+Rotund+Ro%C8%99u&results=1) |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [rp-hal](https://github.com/rp-rs/rp-hal) | Hardware Abstraction Layer for the RP2040 microcontroller | Provides access to RP2040 peripherals |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library for embedded systems without a display server | Renders shapes, text, and images on the display |
| [rand](https://github.com/rust-random/rand) | Random generator | Used to make random values for games |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | A set of traits for embedded hardware abstraction in Rust | Provides generic interfaces for timers, digital I/O, SPI, etc |
| [st7735](https://github.com/almindor/st7789) | Driver crate for ST7735-based LCD displays | Drives the display with embedded-graphics support |
| [heapless](https://github.com/rust-embedded/heapless) | Fixed-capacity data structures without dynamic memory allocation | Enables usage of vectors, strings, and other collections without a heap |
| [mipisdi](https://github.com/almindor/mipidsi) | Driver for MIPI DSI compatible displays like ST7735 | Interfaces MIPI DSI displays with embedded-graphics support |
| [ufmt](https://github.com/knurling-rs/ufmt) | Lightweight, no_std-friendly formatting crate | Enables efficient string formatting with minimal binary size |

## Links

1. [link](https://github.com/Gameboypi/SPW)
2. [link](https://www.youtube.com/watch?v=yauNQSS6nC4&t=16s)
