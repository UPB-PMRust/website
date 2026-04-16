---
sidebar_label: 'Mara-Andreea Damoc'
title: 'RustArcade: High-Performance Embedded Gaming Console'
---

# RustArcade: High-Performance Embedded Gaming Console

## Description
This project implements a standalone handheld gaming console that executes real-time game logic on an STM32 microcontroller using a bare-metal Rust environment. The system operates by continuously polling four digital input lines connected to tactile switches, translating physical presses into directional movement vectors. These inputs are processed through a deterministic game loop that manages coordinate-based state updates, monitors collision detection between active entities, and enforces game rules such as health depletion and scoring. Based on this processing, the system produces synchronized outputs: it renders a dynamic 128x160 graphical interface for gameplay and UI feedback, manages a hardware-level life counter by toggling three discrete LEDs, and synthesizes specific auditory alerts through pulse-width modulation in response to game events. Additionally, the system ensures data persistence by verifying and writing high-score values to non-volatile memory upon the conclusion of each session.

## Technical Risks and Difficulties
Implementing the project presents technical risks primarily related to the strict resource management enforced by the Rust ownership model, which can complicate concurrent access to peripherals like timers or GPIO pins. A major difficulty is optimizing data transmission via the SPI protocol to the TFT LCD screen to maintain a fluid frame rate, as well as rigorously managing the limited Flash memory for storing graphics and scores. Furthermore, implementing a precise debouncing algorithm for the buttons in a non-OS environment represents a challenge for control stability. In this context, I will require specific guidance for the correct configuration of clock registers and interrupt vectors, the efficient abstraction of peripherals using `embedded-hal` traits, and the safe handling of memory addresses for persistent data saving in the internal flash.
