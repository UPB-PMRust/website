# Spicy the mouse
Interactive chase toy for cats!

:::info 

**Author**: IACOB Ioana-Delia \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/proiect-iacobDelia

:::

## Description

Spicy the mouse is an interactive cat toy for cats. It wags its tail as it moves around the room and stops once it encounters any obstacles or the cat topples it.

## Motivation

Pets are a significant part of our lives and important members of our families. The interactive cat toys I saw in stores were my inspiration for this project. This way I could learn about microcontroller programming and hardware interaction while building something I enjoy.

## Architecture 

![diagram](Spicy_the_mouse_diagram.webp "Diagram")

* The microcontroller controls the behaviour of the components
* The DC motors will be controlled to move randomly
* The servomotor will have a continious back and forth movement
* Using the accelerometer or the infrared sensor the robot will stop when the cat topples it or when the robot encounters a wall respectively

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
This week I connected all the components to the microcontroller. I started with the sensors and successfully read values from the infrared sensor using ADC, and from the accelerometer using I2C.

Connecting the motors was a bit trickier though. For the DC motors, although controlling them through PWM works, the debugger crashes shortly after the program is ran. Besides them, I also tried connecting the servomotor, but it couldn't draw enough current from the pico. As such, I also bought a 4 AA batteries case and a LM2596S Step Down module.
### Week 12 - 18 May
I successfully connected all the components to the microcontroller and wrote code for randomly starting and stopping the motors. For this, I used the feature small_rng from the rand crate to randomly generate numbers in a non std environment. The motors also stop when the infrared sensor detects an obstacle.

![3d](3d.webp "3d")

I also designed the support on which I'll be putting all the components. To save space, I decided to put the breadboard on top of the battery pack. I also added plenty of holes that will be used for wire management. To make sure everything fits correctly I created placeholder boxes with the exact dimensions of the components.
### Week 19 - 25 May

## Hardware

![hardware](hardware.webp "Hardware")

Since the pico is, for now, drawing power from the debugger, I'm currently using another breadboard to supply power to the other components. The plan is that, by the end, everything will be connected to the batteries using the smaller breadboard.

* Raspberry pi pico 2w: the microcontroller
* The accelerometer: used for detecting changes in the movement of the robot
* The infrared sensor: used for wall detection
* The servomotor: mimics the movements of a tail
* DC motors: movement
* Dual motor driver: helps control the DC motors

### Schematics

![diagram](schematic.webp "schematic")

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device    | Usage     | Price |
|-----------|-----------|-------|
| Raspberry Pi Pico 2W | Microcontroller | [39.66 Lei](https://www.optimusdigital.ro/ro/placi-raspberry-pi/13327-raspberry-pi-pico-2-w.html) |
| Raspberry Pi Pico RP2040 | Debugger | [30.00 Lei](https://www.emag.ro/microcontroller-raspberry-pi-rp2040-pico/pd/DKQQWNMBM/) |
| Breadboard kit | Electrical connections | [22 Lei](https://www.optimusdigital.ro/ro/kituri/2222-kit-breadboard-hq-830-p.html?search_query=Kit+Breadboard+HQ830+cu+Fire+%C8%99i+Sursa)|
| Infrared sensor | Obstacle avoidance | [3.49 Lei](https://www.optimusdigital.ro/ro/senzori-senzori-optici/4514-senzor-infrarosu-de-obstacole.html?search_query=+Modul+Senzor+Infrarosu+de+Obstacole+&results=6)|
| Motor 130-13200 | DC motors | [4.99 Lei](https://www.optimusdigital.ro/ro/motoare-motoare-fara-reductor/361-motor-130-13200.html?search_query=motor+dc&results=612) x 2|
| Dual Motor driver | Motor control | [3.73 Lei](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/1514-modul-driver-de-motoare-dual-in-miniatura-10-v-15-a.html?search_query=Modul+Driver+de+Motoare+Dual+in+Miniatura&results=1)|
| Ball support | Ease of movement | [3.95 Lei](https://www.optimusdigital.ro/ro/mecanica-suporturi-cu-bila/74-ball-caster.html?search_query=Suport+cu+Bila+&results=118)|
| Accelerometer | Movement sensor | [12.99 Lei](https://www.optimusdigital.ro/ro/senzori-senzori-inertiali/97-modul-accelerometru-cu-3-axe-adxl345.html?search_query=Modul+Accelerometru+cu+3+axe+ADXL345&results=2)|
| Micro Servomotor | Tail movement mechanism | [11.99](https://www.optimusdigital.ro/ro/motoare-servomotoare/2261-micro-servo-motor-sg90-180.html) |
| Step Down module LM2596S | Converting voltage | [12.99](https://www.optimusdigital.ro/ro/surse-coboratoare-reglabile/1108-modul-dc-dc-step-down-lm2596hv.html)|
| Battery support | Power supply | [9.43 Lei](https://www.optimusdigital.ro/ro/suporturi-de-baterii/2806-suport-baterii-4-x-r6.html)|



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy_rp](https://docs.embassy.dev/embassy-rp/git/rp2040/index.html) | HAL for the Raspberry Pi PICO 2W microcontroller | Used to program the Pico |
| [embassy-executor](https://crates.io/crates/embassy-executor) | An async/await executor designed for embedded usage | Used for task execution |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Inspiration for this project](https://www.youtube.com/watch?v=CJwI9GmFXds)
2. [Two wheeled line follower](https://www.ijert.org/research/pid-controller-based-line-following-and-obstacle-avoidance-two-wheeled-robot-IJERTCONV7IS02026.pdf) - not the same idea but similar execution
