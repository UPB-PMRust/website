# Sistem securizat deschidere seif
Acesta este un sistem de securitate cu 3 nivele pentru deschiderea unui seif.

:::info 

**Author**: Popescu Ionut-Teodor \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-PMRust-Students/acs-project-2026-Waffle20)

:::

<!-- do not delete the \ after your name -->

## Description

Utilizatorul trebuie să treacă prin trei etape succesive de verificare:

Nivelul 1 (Digital): Scanarea unui card/tag NFC autorizat.

Nivelul 2 (Secret): Introducerea unui cod PIN de 4 cifre folosind un keypad matricial.

Nivelul 3 (Biometric): Scanarea amprentei digitale.

Dacă oricare dintre pași eșuează, sistemul revine în starea de așteptare. 
La finalizarea cu succes a tuturor etapelor, un servomotor acționează mecanismul de blocare, iar ecranul LCD afișează confirmarea accesului.

## Motivation

Why did you choose this project?

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

Detail in a few words the hardware used.

### Schematics

Place your KiCAD or similar schematics here in SVG format.

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


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...