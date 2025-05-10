# Focalizare Unificata cu Tintire Autonomata Inteligenta  
A turret that tracks and targets people using computer vision and a laser.

:::info 

**Author**: Andrei-Valerian Andreescu \
**GitHub Project Link**: [Github](https://github.com/UPB-PMRust-Students/proiect-DrescoAV.git)

:::

## Description

This project is about building a simple automated turret that can detect and follow people with a camera and aim a laser at them in real-time. The user will be able to set a target (for example a red circle or a yellow t-shirt), and the turret will lock onto it and track it.

## Motivation

I got the idea from the game *Rust*, where you can build autoturrets that shoot at players who get too close to your base. I thought it would be cool to try and make something similar in real life, but just using a laser for now.

## Architecture 

The system has two main parts:

- **Vision + Targeting (Python)**: A Python script runs object detection (YOLO) on a laptop and identifies the target based on color or shape. It sends coordinates to the turret controller.
- **Control (Rust)**: A Raspberry Pi 2W runs a Rust program that controls two servo motors — one for horizontal rotation, one for vertical — and points the camera and laser to track the target. It also reads a distance sensor.
- The camera and laser are mounted on the same platform, so they move together.

## Log

<!-- write your progress here every week -->

### Week 5 - 11 May
TBD

### Week 12 - 18 May
TBD

### Week 19 - 25 May
TBD

## Hardware

Detail in a few words the hardware used. TBD

### Schematics

Place your KiCAD schematics here. TBD

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
TBD


## Software

| Library | Description | Usage |
|---------|-------------|-------|
TBD


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->
TBD
