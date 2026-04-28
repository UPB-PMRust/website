# Mini Solar System Simulator
A model depicting the Earth's rotation and revolution movements around the Sun, surrounded by constellations


:::info 

**Author**: Daria-Antonia Niculescu \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-ifrit18

:::

## Description
This project is a mechanical and electronic simulation of orbital motion, designed as an interactive educational tool. The device uses an STM32 development board to control the rotation of a mechanical arm (representing Earth's orbit) around a light center (the Sun). The project provides real-time information on an OLED screen and envisions constellations through LED light points.

## Motivation
I chose this project because it allows me to bring together two of my biggest passions: astronomy and hardware. It presents a unique challenge to bridge the gap between abstract celestial mechanics and tangible physical implementation and also give me the opportunity to present a visually stunning scenary.

## Architecture 

The project is divided into a few main parts that work together.

**TM32 Control Unit**: The brain that processes encoder inputs, manages orbital timing, and controls the OLED and motor driver.

**Precision Drive System**: A NEMA 17 motor with an A4988 driver provides micro-stepping for smooth revolution, using a flexible coupling to dampen vibrations.

**Visual Interface (HMI)**: A 0.96" OLED displays real-time telemetry, while a rotary encoder allows for intuitive speed and menu adjustments.

**Celestial Lighting**: A central WS2812B RGB Ring represents the Sun, complemented by a 16-LED matrix to illuminate specific star patterns.

**Power Management**: A 12V supply powers the motor, while an LM2596 Step-Down converter provides stable 5V logic power for the STM32 and LEDs.

**Rapid-Prototyping Framework**: Uses an MB-102 Breadboard and Kapton tape for a screwless, modular assembly that is easy to modify and repair.



## Log

### Week 20 - 24 April
Bought the hardware components and started the documentation process.


## Hardware
**Fastening**: Structural screws were eliminated in favor of Kapton tape for securing modules onto the aluminum arm, ensuring a lightweight and non-conductive mounting solution.

**Constellations**: The star map is implemented via a manually perforated cardstock panel, with rear-mounted LEDs aligned to the apertures to create a precise point-source light effect.

### Bill of Materials

| Device| Usage 
| :--- | :--- | 
| **STM32 Blue Pill** | Central Processing Unit | 
| **Motor NEMA 17** | Stepper motor for the orbital motion of the arm | 
| **Driver A4988** | Current and step control for the motor | 
| **OLED 0.96" I2C** | Real-time telemetry display and menu | 
| **Encoder Rotativ** | User input device (speed, navigation) | 
| **Inel 12 LED RGB** | Visual representation of the Sun (light center) |
| **Sursă 12V 20A** | Main power supply for the entire system | 
| **LM2596 Step-Down** | Efficient 12V to 5V conversion for logic |
| **Cuplaj 5x8mm** | Mechanical motor-shaft connection | 
| **Banda Kapton** | Insulating mechanical fastening | 
| **Adaptor DC Jack** | Simple power interface |
| **LED-uri 5mm** | Point-source lighting for the constellation map | 

## Software
TBD 

## Links 
TBD
