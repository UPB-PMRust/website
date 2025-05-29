# SafePi
A smart and secure mini-safe that unlocks with RFID and fights back against intruders.

:::info 

**Author**: Nămoianu Petre \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-PetreNamo

:::

## Description

SafePi is a compact, smart security safe controlled via RFID authentication.
It is designed to be unlocked using authorized RFID cards or a smartphone.
When an unauthorized access attempt is detected, the system triggers a loud audible alarm,
sends a notification to the user's smartphone, and activates a physical deterrent mechanism
that launches small projectiles to discourage tampering.

## Motivation

The motivation behind SafePi is to enhance personal security with an accessible and affordable system.
It not only alerts you instantly via phone notification, but also discourages and delays access through physical deterrents.

## Architecture

- **Main Controller (Raspberry Pi Pico 2W):**  
  Acts as the central unit managing all inputs and outputs
  and sends notifications to your phone.

- **RFID Module:**  
  Reads the tag of the person attempting to open the safe.

- **Servo Motors:**  
  One opens the door when told by the controller,
  the others launch small objects if an intruder is trying to open the safe.

- **LEDs, Buzzer:**  
  Used as a visible and audible alarm.

```mermaid
flowchart TD
    RPI[Raspberry Pi Pico 2 W]

    RFID[RFID Module]
    UIDV[Valid UID?]
    UIDR{Registered UID?}

    ALERT[Send alert]
    BUZZ[Buzzer, LEDs, Deterrence measure]
    SAFE[Open safe]
    SERVO[Servo Motor]

    RPI -->|SPI/I2C| RFID
    RFID --> UIDV
    UIDV --> UIDR

    UIDR -- No --> ALERT
    UIDR -- No --> BUZZ
    UIDR -- Yes --> SAFE
    SAFE --> SERVO

    RPI -->|GPIO| BUZZ
    RPI -->|WiFi| ALERT
    RPI -->|PWM| SAFE
```

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
  Started gathering the required components, checked to see what voltage some parts work at. Started writing the documentation and researching how to make a safe shoot "projectiles".
### Week 12 - 18 May
  During this week I acquired all the remaining parts, decided to switch from 4x1.5V batteries to a single 9V battery in case my DC motor needed more voltage to spin faster. I put all my hardware in the safe for the first time and came up with a rough idea for the launching mechanism for the "projectiles". 
### Week 19 - 25 May
  Started writing the code for the safe. First made the LEDs blink and the buzzer sound, then I made the servomotor twist. After a brief pause, I continued with the PN532. I initially forgot to connect the RESET pin and did not know why I had to disconnect the PN532 every time i wanted to run the code. After that I struggled a little with an issue that would make the module not read keycards after about 1 minute.

## Hardware
  For my safe, I used a Raspberry Pi Pico 2W as my MCU. With the help of a PN532 module I am able to scan any access keys with NFC/RFID. My locking mechanism is a SG90 servomotor. I have also installed two LEDs, one blue and one red, and an active buzzer, meant to resemble an alarm. The "projectiles" are fired with the help of a DC motor that is spinning a wheel. Below is a photo with the hardware outside the box:

![Hardware](hardware.webp)


### Schematics

![Kicad](kicad.svg)

### YouTube Demo
[SafePi Demo](https://youtube.com/shorts/JDA9XmK4RdE?feature=share)


### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico 2W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [40 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/13327-raspberry-pi-pico-2-w.html) |
| Red LED | Blinking when the alarm goes off | [0.39 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/696-led-rou-de-3-mm-cu-lentile-difuze.html) |
| Blue LED | Blinking when the alarm goes off | [0.29 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/12237-led-albastru-de-5-mm.html) |
| Buzzer | Used when the alarm goes off | [1 RON](https://www.optimusdigital.ro/ro/audio-buzzere/635-buzzer-activ-de-3-v.html) |
| [PN532 RFID Module](https://www.nxp.com/docs/en/nxp/data-sheets/PN532_C1.pdf) | Used for  opening the safe | [50 RON](https://www.optimusdigital.ro/ro/altele/1590-modul-nfc-pn532.html) |
| 9V Battery | Used for powering the motors and the RFID Module | 10 RON |
| Battery Support | Connecting the battery to the breadboard | [1.29 RON](https://www.optimusdigital.ro/ro/suporturi-de-baterii/20-conector-pentru-baterie-de-9-v.html) |
| Breadboard | Connecting all the components | [4.56 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/44-breadboard-400-points.html) |
| [Step-Down module](https://www.monolithicpower.com/en/documentview/productdocument/index/version/2/document_type/Datasheet/lang/en/sku/MP1584EN-LF-Z/document_id/204/?srsltid=AfmBOorbWpoH9ILvxrVysau20kQVKT-H9PYR5KDHIYSkhpZNmCMDHyRe) | Used to limit the voltage to 5V | [6.49 RON](https://www.optimusdigital.ro/ro/surse-coboratoare-reglabile/166-modul-dc-dc-step-down-mp1584en.html) |
| [Servomotor](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf) | Safe locking mechanism | [11.99 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) |
| [DC Motor](https://www.handsontec.com/dataspecs/GA12-N20.pdf) | Used for throwing small balls | 33 RON | 
| Transistor | Used for commanding the DC motor | 2 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy](https://github.com/embassy-rs/embassy) | Async embedded framework for Rust | Main framework for handling async tasks |
| [embassy-rp](https://github.com/embassy-rs/embassy) | RP2350 support for Embassy | Provides access to GPIO, I2C, and timers for Raspberry Pi Pico 2 |
| [defmt](https://github.com/knurling-rs/defmt) | Logging framework for embedded Rust | Debugging tool |
| [embedded-hal-async](https://crates.io/crates/embedded-hal-async) | asynchronous traits for embedded hardware access using async | asynchronous I2C |
| Own PN532 crate | a lightweight pn532 implementation | wrote it to be able to use with async with better logging |



## Links
[1. PWM](https://pmrust.pages.upb.ro/docs/acs_cc/lab/03)

[2. I2C](https://pmrust.pages.upb.ro/docs/acs_cc/lab/06)
<!-- Add a few links that inspired you and that you think you will use for your project -->

