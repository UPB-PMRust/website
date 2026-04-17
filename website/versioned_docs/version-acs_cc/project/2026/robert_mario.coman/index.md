# Endless Motion: Reactive Robotic System for Liquid Cleaning

### Project Description
**Endless Motion** is a robotic arm system that receives input from sensors placed in specific locations to detect the presence of a liquid. It processes these signals to determine the target area that requires intervention. Based on this data, it calculates control commands, moves to the detected zone, and executes a wiping action. The system continuously updates its movement, repeating the process based on newly received signals.

### Motivation
This project explores the control of a robotic arm within a real-time reactive system. The main idea is to adapt an automated industrial behavior (responding to external stimuli) into a small-scale application. The project combines data acquisition electronics, asynchronous embedded programming in Rust, and mechanical modeling, demonstrating the capability to read real-time sensory data and translate it into precise mechanical actions.
