---
sidebar_label: 'Autonomous Catapult Vehicle'
---

# Autonomous Catapult Vehicle

An autonomous vehicle which has a catapult attached to it.

:::info
**Author:** Robert Marian Albu  
**GitHub Project Link:** [https://github.com/UPB-PMRust-Students/acs-project-2026-ARobertM44](https://github.com/UPB-PMRust-Students/acs-project-2026-ARobertM44)

:::

## Description

This project consists of an autonomous car which can navigate to some (x, y) coordinates using magnetic encoders for reliability. Using a camera, it detects a specific target via color tracking, and with a rotating catapult attached, it launches a projectile toward the detected objective.

## Motivation

I chose this project because I wanted to test myself by implementing a machine capable of using real mechanical physics with integrated control, testing technical skills, not just simulations in a software. The idea of ​​building a functional catapult seemed interesting and challenging, but achievable within the time constraints.

## Architecture

![Architecture Diagram](./Architecture.svg)

## Log

### Week 21 - 27 April

Researched and ordered hardware components.

### Week 5 - 11 May
### Week 12 - 18 May
### Week 19 - 25 May

## Hardware

The system is based on an STM32 NUCLEO development board, which manages real-time odometry and Pixy2 camera data. Precision is ensured by magnetic encoders, while a high-discharge 7.4V LiPo battery provides stable power. This setup allows the controller to handle the high current spikes of the MG996R catapult servo without resets.

### Schematics

To be continued..

### Bill of Materials

| Component | Description | Price |
| :--- | :--- | :--- |
| **Nucleo Board** | Main Microcontroller | Free |
| **Pixy2 Camera** | Vision sensor for color-based tracking | Free (competition award) |
| **Gens Ace Soaring LiPo** | 7.4V 2200mAh | 87 RON |
| **L298N Motor Driver** | Dual H-Bridge for DC motor control | 15 RON |
| **AS5600 Magnetic Encoder** | 12-bit resolution rotary sensor (2 units) | 44 RON |
| **MG996R Servo** | High-torque metal gear servo for catapult (2 units) | 60 RON |
| **DC Gear Motors** | Motors for vehicle propulsion (4 units) | 100 RON |

## Software

To be continued..

## Links