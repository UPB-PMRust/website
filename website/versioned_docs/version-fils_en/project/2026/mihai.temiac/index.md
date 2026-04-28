# Prosthetic arm with digit control
3D-printed prosthetic hand with EMG-based digit control and tendon-driven mechanical actuation

:::info 

**Author**: Temiac Mihai-Gabriel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-mihailinux

:::

<!-- do not delete the \ after your name -->

## Description

The project focuses on building a prosthetic hand with electromyography, the most used technique for driving prosthetics. Muscle signals are preferred due to their higher amplitude and ease of non-invasive surface acquisition.

By sampling multiple areas with electrodes, via specialized EMG sensors or repurposed EKG sensors, a potential difference is linked to muscle activity. Using multiple sensors enables the mapping of groups of digits/individual digits, allowing the prosthetic to become usable in multiple scenarios. 

The prosthetic will be 3D printed, with channels for the hand pulleys, plus string materials for the flexor tendons and elastic materials for the extendor tendons, achieving natural momevent, aiming for a mechanical system as close to reality.

## Motivation

I've always been passionate about the inner workings of life, with physiology being at the top of the list. Building a prosthetic arm would allow me not only to practice my mechanical skills, but also delve into how electrical signals, something which we study so extensively, are produced on such a small scale. 

Processing muscle signals is a difficult task due to their irregular nature, as they do not contract in a completely synchronized manner. EKG signals are periodic, synchronized and have a consistent pattern, but they lack the challenge of analysing such complex signals (not to deny their difficulty, but looking at it in an objective manner). 

As such, I chose this project due to its nature and complexity, as I strongly believe it will help me develop skills which I may otherwise not have the chance to. 

## Architecture 

<center>
![Diagram](diagram.svg)
</center>

## Log

<!-- write your progress here every week -->

### Week 4-5
- Research on project ideas
- Found many open-source projects, there seems to be an even bigger community for this than I thought

### Week 6-7
- Started looking into tutorial regarding 3D modelling
- Found multiple possible issues with the tendon channel, prototyping is necessary

### Week 8
- First 3D renders, there is plenty of room for improvement, but have been getting more comfortable with Fusion

### Week 9
Working on the documentation.

## Hardware

The system uses **AD8232** EKG sensors, repurposed for EMG due to the high cost of specialised sensors. 

**Gel electrodes** are used for stabilising the signal, as the AD8232 is already noisy by default, with muscle signals introducing yet another degree of complexity. 

Everything is processed by an **STM32U545 Nucleo** microcontroller. **MG90S** servos will be used in the finger pulley system, as I need enough torque to counteract the force pulling the finger back in its neutral position.

Generic string and elastic materials will be used for the tendons, such as fishing line for the flexors and springs for the extensors.

### Schematics

TODO

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|------|------|
| [STM32U545 Nucleo](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Microcontroller | [borrowed] |
| [AD8232 EKG sensor](https://www.analog.com/media/en/technical-documentation/data-sheets/ad8232.pdf) | EMG signal acquisition (repurposed EKG module) | [35 RON x 3](https://www.optimusdigital.ro/ro/senzori-altele/1347-modul-senzor-ecg-ad8232.html?srsltid=AfmBOooAX5b3QDFkBnnuSQi5Ejg6U0BX_VEz3xrKOzaeQP8Z8HV6hnwI)|
| [MG90S Servo Motor](https://www.tinytronics.nl/product_files/000263_Data%20Sheet%20of%20MG90S%20Analog%20Servo%20Motor.pdf) | Finger actuation | [20RON x 3](https://sigmanortec.ro/en/servo-motor-mg90s-metal-gears) |
| Gel electrodes | Skin-signal interface | [30 RON](https://www.aviafarm.ro/cumpara/set-100-electrozi-adezivi-ecg-ovali-f9089-cu-gel-solid-si-senzor-ag-953?utm_source=portal&utm_medium=web&utm_campaign=google_xml&gad_source=1&gad_campaignid=22826372328&gbraid=0AAAAADQMw-p4DXuVPfSpT3SlDTUKEi_cz&gclid=CjwKCAjwqazPBhALEiwAOuXqdCRNGu_i0Rv1CSS49kxbpT8zRxvhcMcm2q3GkeL3nzO_ce6pKp12-xoCPNoQAvD_BwE) |
| Elastic bands / springs | Extensor tendon system | owned |
| Fishing line | Flexor tendon system | owned |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | STM32 async runtime | Hardware control, ADC, PWM |
| [embassy-executor](https://github.com/embassy-rs/embassy) | Task scheduler | Real-time control loop execution |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction | GPIO, ADC, PWM interfacing |
| [defmt](https://github.com/knurling-rs/defmt) | Embedded logging | Debugging and signal monitoring |
| [micromath](https://github.com/NeoBirth/micromath) | Lightweight math | Signal filtering and smoothing |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Advanced version with high-end EMG sensors](https://www.youtube.com/watch?v=KSP4o_WCqVs)
2. [OpenBionics](https://openbionics.com/en/)
3. [OpenBionics open-source designs](https://github.com/OpenBionics/Prosthetic-Hands)
