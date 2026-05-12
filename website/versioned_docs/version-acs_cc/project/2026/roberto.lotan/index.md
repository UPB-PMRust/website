# Shot Maker
A smart system that automatically prepares drink shots based on user selection.

:::info

**Author**: Lotan Roberto-Gabriel  
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-Adamura1

:::

## Description

Shot Maker is an automated system that prepares drink shots based on user input. The user selects a drink type from a display interface. Based on this selection, the system calculates the required quantity for each ingredient.

Liquids are dispensed into a glass using pumps or valves. A weight sensor continuously measures the amount poured, and the system stops dispensing when the target weight is reached. For multi-ingredient drinks, each component is added proportionally.

## Motivation

The project aims to automate drink preparation while ensuring precision and repeatability. It explores embedded control systems applied to real-world tasks.

## Architecture

- **Input Layer**: touchscreen interface for selecting drink type  
- **Processing Layer**: computes target weight and ingredient ratios  
- **Actuation Layer**: controls pumps or valves  
- **Measurement Layer**: load cell monitors real-time weight  
- **Output Layer**: final drink in the glass  

### Flow Summary:
- User selects drink  
- System computes ingredient weights  
- Pumps dispense liquids  
- Load cell measures continuously  
- System stops at target weight  

## Log

### Week 20-26 Apr
- Defined functionality  
- Identified main components  

### Week 27-3 Apr
- [To be completed]

### Week 4-10 Apr
- [To be completed]

## Hardware

- Microcontroller  
- Load cell + HX711  
- Pumps / valves  
- Tubing  
- Display
- Power Source
- Load cell

### Schematics
![alt text](<schematic.svg>)
### Photos

### Bill of Materials

| Device | Usage | Price |	
|--------|--------|-------|
| [STM32 NUCLEO-U545RE-Q](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html) | Microcontroller (main control unit) | ~120 RON |
| [HX711 Load Cell Amplifier](https://www.optimusdigital.ro/ro/senzori-senzori-de-greutate/471-modul-hx711.html) | Converts load cell signal to digital | ~10 RON |
| [Load Cell 1kg](https://www.optimusdigital.ro/ro/senzori-senzori-de-greutate/468-celula-de-sarcina-1kg.html) | Measures weight of the liquid | ~35 RON |
| [Peristaltic Pump 12V](https://www.optimusdigital.ro/ro/cautare?controller=search&s=pompa+peristaltica) ×4 | Dispenses liquids | ~50 RON × 4 |
| [MOSFET Module IRF520](https://www.optimusdigital.ro/ro/cautare?controller=search&s=mosfet+irf520) ×4 | Controls pumps from microcontroller | ~5 RON × 4 |
| [LCD 16x2 I2C](https://www.optimusdigital.ro/ro/cautare?controller=search&s=lcd+1602+i2c) | User interface display | ~20 RON |
| [Power Supply 12V 3A](https://www.optimusdigital.ro/ro/cautare?controller=search&s=sursa+12v) | Powers pumps | ~40 RON |
| [Breadboard](https://www.optimusdigital.ro/ro/cautare?controller=search&s=breadboard) | Prototyping | ~15 RON |
| [Jumper Wires](https://www.optimusdigital.ro/ro/cautare?controller=search&s=fire+jumper) | Connections | ~10 RON |
| [Silicone Tubing](https://www.optimusdigital.ro/ro/cautare?controller=search&s=tub+silicon) | Liquid transport | ~15 RON |

TOTAL: ~450 RON

## Links

* [Lab Materials](https://pmi.acs.pub.ro/)
* [About Rust](https://docs.rust-embedded.org/book/)
* [Youtube](https://youtu.be/Z7GkGeZrb2Y?is=evn5AIzjkZRhCvdm)
* [Youtube2](https://youtu.be/2DopvpNF7J4?is=vVKccLsi-WARQd98)