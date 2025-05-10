# Radar
An ultrasonic **radar system** that detects objects' positions using sound waves instead of radio signals.

:::info 

**Author**: Zaluceanu Mihaela \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-mihaelazaluceanu

:::

## Description

This project is a small-scale ultrasonic RADAR system that detects the presence and relative position of objects in the horizontal plane. It uses the **HC-SR04 ultrasonic sensor** for distance measurements and the **SG90 micro servo motor** to rotate the sensor across a 180° arc, enabling area scanning. The system is controlled by a **Raspberry Pi Pico 2W**.
Designed as a modular component, this project can be integrated into larger systems where echolocation improves spatial awareness and obstacle detection.

## Motivation

This project is modular and can be integrated into larger robotics or IoT projects later.

## Architecture 

![Radar](arhitectura.svg)

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May

### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

- x2 Raspberry Pi Pico 2W;
- SG90 micro-servomotor with 180° rotation range;
- HC-SR04 ultrasonic distance sensor;
- LM7805 voltage regulator/stabilizer;

### Schematics

TODO: ...

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.tme.eu/ro/details/sc1632/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2-with-header/) |
| [Raspberry Pi Pico 2](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The debugger | [35 RON](https://www.tme.eu/ro/details/sc1632/raspberry-pi-sisteme-incorporate/raspberry-pi/raspberry-pi-pico-2-with-header/) |
| Breadboard | Breadboard | [7 RON](https://www.emag.ro/breadboard-placa-test-400-puncte-oky0005/pd/DSBXR1MBM/?ref=history-shopping_422378681_2895_1) |
| [LM7805](https://www.ti.com/lit/ds/symlink/lm7800.pdf) | The stabilizer | [7.33 RON](https://www.emag.ro/circuit-integrat-stabilizator-de-tensiune-to220-3-cdil-lm7805-t157585/pd/DH365KYBM/?ref=history-shopping_422378681_7083_1) |
| Wires | Wires | [6 RON](https://www.emag.ro/10-x-fire-dupont-tata-tata-20cm-cl55/pd/DV8M9WBBM/?ref=history-shopping_422378681_12161_1) |
| [USB to Micro-USB Cable](https://www.mouser.com/pdfdocs/HiroseZX62Datasheet24200011.pdf) | USB | [8.79 RON](https://www.emag.ro/cablu-alimentare-si-date-ugreen-fast-charging-usb-la-micro-usb-nickel-plating-pvc-1m-negru-6957303861361/pd/DC7CCBYBM/?ref=history-shopping_422378681_64468_1) |
| [HC-SR04 Sensor](https://cdn.sparkfun.com/datasheets/Sensors/Proximity/HCSR04.pdf) | The sensor | [12.54 RON](https://www.emag.ro/senzor-ultrasonic-hc-sr04-3/pd/D3NL48YBM/) |
| [SG90 Servo-Motor](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | The servomotor | [19.64 RON](https://www.emag.ro/set-servomotor-sg90-unghi-de-lucru-180-grade-23-mm-9-g-3874783591829/pd/DXVZ3MYBM/) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy](https://embassy.dev/) | Asynchronous embedded framework for Rust | Core framework for running async tasks on the Raspberry Pi Pico 2 |
| [embassy-rp](https://embassy.dev/) | For the Raspberry Pi RP2040 microcontroller | Hardware abstraction layer for Raspberry Pi Pico 2 |
| [embassy-hal](https://embassy.dev/) | Hardware abstraction layer for embedded devices | Standard interface for GPIO, SPI, I2C, PWM, etc. |
| [panic-halt](https://docs.rs/panic-halt/latest/panic_halt/) | Minimal panic handler | 	Used for debugging in embedded Rust |
| [ultrasonic-sensor](https://crates.io/crates/ultrasonic-sensor) | Driver for HC-SR04 ultrasonic sensor | Measures distance via trigger/echo GPIO |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Lab03 - PWM & ADC](https://pmrust.pages.upb.ro/docs/acs_cc/lab/03)
2. [Lab05 - SPI](https://pmrust.pages.upb.ro/docs/acs_cc/lab/05)
3. [Lab06 - I2C](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
4. [Radar 2016](https://ocw.cs.pub.ro/courses/pm/prj2016/anitu/bogdan.folea)
