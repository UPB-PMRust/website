---
sidebar_label: 'Joystick-Controlled Mini Excavator'
sidebar_position: 1
---

# Joystick-Controlled Mini Excavator

## Description
The project aims to build a mechanical excavator arm controlled via a joystick interface. The system processes analog signals from a 2-axis joystick and digital inputs from command buttons to determine the arm's direction and movement type. A core feature of the control logic is the implementation of **software end-stops** to prevent mechanical strain on the joints.

The system drives four servomotors to coordinate:
* **Base Rotation** (swing)
* **Main Boom Lift**
* **Stick Movement** (dipper)
* **Bucket Operation** (open/close)

The result is a fluid, responsive robotic arm capable of manipulating light objects, accurately replicating the user's input from the joystick.

## Technical Summary (Peripherals)
To implement this in Rust, the following hardware peripherals and concepts will be used:
* **ADC (Analog-to-Digital Converter):** To read the voltage values from the joystick's X and Y axes.
* **PWM (Pulse Width Modulation):** To control the precise position of the 4 servomotors.
* **GPIO (General Purpose Input/Output):** For the joystick button and additional control switches, using **interrupts** for immediate response.
* **RTIC / Embassy:** A concurrency framework to manage the real-time execution of the control loop.