# Mini air defense system
A robotic pan-tilt turret capable of detecting targets and launching projectiles using a flywheel mechanism.

:::info 

**Author**: Condrea Vlad-Stefan \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Ionidis


:::

## Description

A smart robotic turret made using a STM32 NUCLEO board that acts as the main controller. The system uses a pan-tilt mechanical bracket driven by two MG996R servo motors to aim. An HC-SR04 ultrasonic sensor is mounted on the moving arm to detect the distance to a target. Once the target is within range, the Nucleo board triggers the flywheel launcher, which consists of two high-speed DC motors with rubber wheels that shoot the projectile. 
The entire system is powered by a LiPo battery, using an LM2596 step-down module to safely provide 5V to the servos and sensor.

## Motivation

I believe that it would be a fun and challenging experience. I find robotics fascinating, and this project perfectly combines mechanical assembly, sensor data acquisition, and motor control using Rust. It acts as a great introduction to tracking systems and automated defense mechanisms.

## Architecture 

The system starts with the HC-SR04 ultrasonic sensor, which constantly sends distance data to the STM32 Nucleo controller. 

The Nucleo board processes the sensor data to determine if a target is present. Based on the logic, it generates PWM signals sent directly to the two MG996R servo motors to adjust the Pan (horizontal) and Tilt (vertical) angles. When the target is locked, the Nucleo sends digital signals to a motor driver, which spins up the two DC motors of the flywheel launcher.

Power management is critical: a 7.4V LiPo battery supplies raw power to the motor driver for the DC motors. In parallel, the battery connects to an LM2596 Step-Down converter, which drops the voltage to a stable 5V to safely power the STM32 board, the HC-SR04 sensor, and the servo motors without frying them.

![Architecture Diagram](images/architecture.svg)

## Log

### Week 20 - 26 April
Got approval and researched the components. 
Ordered all the components.

### Week 5 - 11 May
Assembled the mechanical pan-tilt bracket. Fixed alignment issues with the tilt servo and bearing. 
Attached the HC-SR04 sensor.
### Week 12 - 18 May

### Week 19 - 25 May

## Hardware

The project uses the Nucleo board as the brain. It receives echo pulses from the HC-SR04 sensor. After processing the distance, it sends PWM signals to the Pan and Tilt MG996R servos. It also controls a motor driver (L298N/L293D) to activate the dual DC motors for the launcher. The LM2596 acts as a power regulator.

### Schematics

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32 Nucleo-U545RE-Q](https://www.st.com/) | Main Controller | Lab Provided |
| [Metal Pan-Tilt Bracket](https://www.optimusdigital.ro/) | Mechanical structure for the arm | 45 RON |
| [2 x MG996R Servo Motor](https://www.optimusdigital.ro/) | Aiming (Pan and Tilt axis) | 60 RON | 
| [2 x DC Motor with Rubber Wheel](https://www.optimusdigital.ro/) | Flywheel projectile launcher | 30 RON |
| [HC-SR04 Ultrasonic Sensor](https://www.optimusdigital.ro/) | Target detection and distance measuring | 10 RON |
| [LM2596 Step-Down Module](https://www.optimusdigital.ro/) | Voltage regulator (Drops LiPo 7.4V to 5V) | 15 RON |
| [L298N Motor Driver Module](https://www.optimusdigital.ro/) | Controls the DC motors for the launcher | 20 RON |
| [LiPo Battery 7.4V](https://www.optimusdigital.ro/) | Main power supply | 60 RON |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-stm32](https://github.com/embassy-rs/embassy) | Hardware Abstraction Layer | Handling GPIOs, EXTI (for sensor echo), and PWM peripherals (for servos) |    
| [embassy-executor](https://github.com/embassy-rs/embassy) | Async task executor | Managing concurrent tasks (sensor reading, aiming, shooting) |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction traits | Standard interface for interacting with peripherals |
| [defmt](https://github.com/knurling-rs/defmt) | Logging framework | Used for debugging distance readings and states |

## Links

[How a Flywheel Blaster Works](https://www.youtube.com/watch?v=example1)
[STM32 Rust Embassy Tutorial](https://github.com/embassy-rs/embassy)