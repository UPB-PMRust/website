# Order Management and Printing System for Flower Shop

A Wi-Fi connected thermal receipt printer that automatically prints order details
when a customer places an order on an external server.

:::info

**Author**: Mangu Adrian Constantin \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-LikeWho54

:::

---

## Description

This project implements an **Order Management and Printing System** built using **Rust**
on a **Raspberry Pi Pico 2W** microcontroller.

Six months ago, I built a website for a local flower shop. The website allows customers
to visit the site, place an order, pay, and the flower shop automatically receives a
WhatsApp message with the order details. I wanted to replace this with something more
efficient and physical, a small thermal printer that automatically prints a receipt
with the order details instead.

The system works as a **"ticketing machine" style terminal**. When a customer places an
order on the external server, the Pico 2W receives the order data via **Wi-Fi**, parses
it, and prints a physical receipt containing the customer name, list of flowers, and
total price.

---

## Motivation

I chose this project because it solves a **real problem** for a real business I have
already worked with. The current WhatsApp-based solution works, but it is not ideal because
messages can be missed, and the workflow is manual.

A dedicated hardware terminal that automatically prints receipts is more reliable,
faster, and more professional.

---

## Architecture

### Main Components

#### Connectivity Layer
- **Raspberry Pi Pico 2W (RP2350 + CYW43439)**  
  Connects to Wi-Fi, receives order data from the external server via HTTP,
  and drives the thermal printer over UART.

#### Output Layer
- **Thermal Printer (CSN-A2)**  
  Receives print commands from the Pico 2W over UART and prints physical
  receipts with order details (customer name, flowers, total price).

#### Power Layer
- **5V 2A Power Adapter**  
  Powers both the Pico 2W and the thermal printer, which requires a stable
  5V supply for reliable printing.

---

## Component Connection

![Architecture Diagram](Diagram.drawio.svg)

### Wi-Fi Link (CYW43439 ↔ RP2350)
The CYW43439 chip is built into the Pico 2W and communicates internally with
the RP2350. It handles connecting to the Wi-Fi network and receiving order
data from the external server.

### Data Link (Pico 2W ↔ Thermal Printer via UART)
The Pico 2W sends ESC/POS print commands to the thermal printer over UART
(TX/RX pins). The printer outputs the formatted receipt.

### Power Link
The 5V 2A adapter powers the thermal printer directly. The Pico 2W is powered
via its VSYS pin from the same supply.

---

## Log

### Week 23 – 29 March
Got the project idea based on the existing flower shop website I had already built.

### Week 30 March – 5 April
Discussed the idea with the flower shop owner and gathered requirements for
what the receipt should contain and how the system should work.

### Week 6 – 12 April
Got the project approved. Planned the architecture, chose the components
and researched thermal printer communication protocols (ESC/POS) and Rust crates.

### Week 13 – 19 April
Ordered all the required components.

### Week 20 – 26 April
(Work in progress)

### Week 27 April – 3 May
(Work in progress)

### Week 4 – 10 May
(Work in progress)

### Week 11 – 17 May
(Work in progress)

### Week 18 – 24 May
(Work in progress)

---

## Hardware

_Schematic to be added (KiCad)._

_Photos to be added when components arrive._

---

## Bill of Materials

| Device | Usage | Price |
|--------|-------|-------|
| Raspberry Pi Pico 2W | Main MCU + Wi-Fi connectivity | 40 lei |
| Thermal Printer CSN-A2 | Prints order receipts via UART | 170 lei |
| 5V 2A Power Adapter | Powers Pico 2W and printer | 10 lei |
| Jumper Wires + misc | Connections | 5 lei |

---

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| embassy-rp | Async embedded framework for RP2350 | GPIO, UART, task management |
| embassy-executor | Async task executor | Managing concurrent tasks |
| cyw43-pio | CYW43439 Wi-Fi driver | Wi-Fi connectivity |
| embassy-net | Network stack | TCP/IP, HTTP requests |
| serde-json-core | JSON parsing (no_std) | Parsing incoming order data |
| thermal-print | Thermal printer ESC/POS commands | Controlling the printer |


## Links

1. [Embassy-rs Framework](https://embassy.dev)
2. [ESC/POS Command Reference](https://www.epson-biz.com/modules/ref_escpos/index.php)
3. [Raspberry Pi Pico 2W Datasheet](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)
4. [Flowershop](https://www.ducksflower.ro/)