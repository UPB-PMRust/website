# Fixed Wing RC Drone

A remote controled fixed wing drone

:::info

    **Author:** Vlad Buruleanu \
    **Github Project Link:** https://github.com/UPB-PMRust-Students/fils-project-2026-vladburuleanu

:::

## Description
The aim of this project is to build a reliable UAV system that includes:
- The drone itself
- a flight controller that actuates control surfaces and sends over telemetry
- The ground station that is used to pilot the drone remotely



## Motivation

Commercial RC drones and receiver/transmitter systems are often expensive. The goal is to develop a similar system on a smaller budget with some parts I already have lying around.


## Architecture

![System Architecture](images/architecture.svg)

## Log

# Weeks 5-6

Ordered necessary hardware components and started the development process of the drone, running simulations and making rough calculations to figure out optimal flight configuration.

# Weeks 7-8

Finished 3d design of wing and tail structure.

# Week 9

## Hardware

The flight controller is designed around a Raspberry pi pico 2W microcontroller which controls the servo motors and brushlless motor responsible for actuating the control surfaces and providing thrust. The microcontroller is further responsible with collecting and sending telemetry data to the ground station via the NRF24L01+ RF module. Telemetry data includes battery level and link quality.

The Ground Station is centered around a Raspberry pi Zero 2W, who's main purpose is to take input from an XBOX360 controller and transmit it over to the drone. It also displays some telemetry data on the OLED display.


### Schematics

### Bill of materials

## Software

## Links
