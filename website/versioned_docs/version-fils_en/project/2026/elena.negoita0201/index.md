\---

sidebar\_position: 1

\---



\# Color Parking Car



\[cite\_start]A four-wheeled robot using an STM32 microcontroller and Rust to automatically find and park in a spot based on the color of the floor. \[cite: 4, 7, 15]



:::info

\[cite\_start]\*\*Author\*\*: Elena-Daniela NEGOIȚĂ \[cite: 1]



\*\*GitHub Project Link\*\*: \[Color Parking Car Repository]()

:::



\## Description



\[cite\_start]This project consists of an autonomous four-wheel-drive robot based on the STM32U545RE-Q microcontroller, programmed in Rust. \[cite: 7] \[cite\_start]The robot's objective is to traverse a path, identify a specific parking bay from three available options based on a color-coded floor marker, and execute a pre-programmed parking maneuver. \[cite: 8]



\## Motivation



\[cite\_start]I chose this project to explore building an autonomous vehicle using the STM32U545RE-Q and the Rust programming language. \[cite: 14] \[cite\_start]It demonstrates how a 4-wheel drive vehicle can navigate a course and park itself using color cues and timed motor control, showing how logic can solve navigation challenges without expensive distance sensors. \[cite: 15, 17]



\## Architecture



\[cite\_start]The software is organized into a \*\*State Machine\*\* with four specific modes: \[cite: 24, 25]



\* \*\*Mode 1 (Driving):\*\* The robot drives forward and looks for a color. \[cite: 26]

\* \[cite\_start]\*\*Mode 2 (Checking):\*\* Once a color is seen, the robot confirms it is the correct target marker. \[cite: 27]

\* \[cite\_start]\*\*Mode 3 (Parking):\*\* The robot runs a "script" of timed movements (e.g., turning for 1.2s, reversing for 0.8s). \[cite: 28]

\* \*\*Mode 4 (Finished):\*\* The motors stop, and the system waits in a finished state. \[cite: 29]







\## Log



\### Week 5 - 11 May

\* Initial project setup and hardware assembly.



\### Week 12 - 18 May

\* Implementing color sensor logic and motor calibration.



\### Week 19 - 25 May

\* Testing the parking state machine and final adjustments.



\## Hardware



\[cite\_start]The system uses a 4WD chassis driven by four independent DC motors controlled via PWM for speed and calibration. \[cite: 9, 18, 19]



\### Bill of Materials



| Device | Usage |

| --- | --- |

| \*\*STM32 Nucleo-U545RE-Q\*\* | \[cite\_start]Main Microprocessor \[cite: 31] |

| \*\*TCS34725\*\* | \[cite\_start]RGB Color Sensor (I2C) \[cite: 31] |

| \*\*TB6612FNG\*\* | \[cite\_start]Dual H-Bridge Motor Drivers \[cite: 31] |

| \*\*DC Gear Motors\*\* | \[cite\_start]Propulsion (Yellow TT style) \[cite: 31] |

| \*\*LM2596\*\* | \[cite\_start]DC-DC Buck Converter (Step-down) \[cite: 32] |

| \*\*18650 Li-ion Cells\*\* | \[cite\_start]Battery Power Source \[cite: 31] |



\## Software



\[cite\_start]The project is built using the \*\*Embassy\*\* framework for async Rust development. \[cite: 34, 35]



| Library | Usage |

| --- | --- |

| \*\*embassy-stm32\*\* | \[cite\_start]Hardware abstraction for STM32U5 \[cite: 34] |

| \*\*tcs34725\*\* | \[cite\_start]RGB Color Sensor driver \[cite: 38] |

| \*\*embedded-hal\*\* | \[cite\_start]Hardware Abstraction Layer \[cite: 39] |

| \*\*defmt\*\* | \[cite\_start]Efficient logging for debugging \[cite: 43] |



\## Links



1\. \[STM32U545RE-Q Official Page](https://www.st.com/en/microcontrollers-microprocessors/stm32u545re.html)

2\. \[Embassy Rust Framework](https://embassy.dev/)

