---
layout: section
---

# The good and the bad on modern microcontrollers

---

# What a simple controler's pins can do

Atmega324P *Note: 2 - max 3 functions per output pin*

<p align="center">
  <img src="img/pins_atmega324.png" style="width:60%; display:block; margin:0 auto;" alt="pins_st">
</p>

---

# What this looks like for a modern one

*Note: up to 20 functions per output-pin*

<p align="center">
  <img src="img/pins_st.png" style="width:70%; display:block; margin:0 auto;" alt="pins_st">
</p>

---

# "It can do anything"

The “many registers” are just the control surface for a very capable on-chip system.

**Run lots of interfaces:** UART/USART, SPI, I²C, I²S, CAN/FDCAN (device-dependent), USB, etc.

**Talk to the real world with rich GPIO:** alternate functions, open-drain, pull-ups, fast/slow edges, input filtering

**Do precision timing:** advanced timers for PWM, motor control, capture/compare, encoder, synchronized outputs

**Measure analog signals:** ADC (multi-channel, triggers), comparators/op-amps (device-dependent), internal references

**Move data at high speed without CPU involvement:** DMA + FIFO paths between peripherals and memory

**Tune performance and power:** multiple clock sources/PLLs/dividers + per-peripheral clock selection

**Manage low-power operation:** sleep/stop/standby modes, wakeup sources, retention choices

**Built-in security & integrity:** hardware crypto accelerators, key storage/options, access control/locking (device-dependent)

---

# How can it?

*Note: this is 40% of the diagram*

<p align="center">
  <img src="img/lots_of_hardware.png" style="width:100%; display:block; margin:0 auto;" alt="pins_st">
</p>

---

# How can it?

Lots of hardware blocks (e.g., DMA, WDT, SPI, etc) that must be **(1) configured**, **(2) clocked and interconnected/routed correctly** (bus, interrupts, req), and **(3) mapped to the outside world via properly configured pins** (AF, electrical characteristics such as mode, speed, pull-ups, drive strength, type).

<p align="center">
  <img src="img/lots_of_hardware_2.png" style="width:100%; display:block; margin:0 auto;" alt="pins_st">
</p>

---

# All these need lots of parameters to be set (1)

There is always a tradeoff - between simple, chip andlimited and powerfull, expensive and with lots of options - and complex set-up

**Run lots of interfaces**: enable peripheral clock, select pins/AF, set protocol mode + timing/baud, enable IRQ/DMA, handle status/error flags

**Talk to the real world with rich GPIO:** mode (in/out/AF/analog), pull-up/down, output type (push-pull/open-drain), speed/slew, input filters, alternate function mapping

**Do precision timing:** timer clock source/prescaler, counter mode, PWM/capture settings, channel mapping, triggers/synchronization, interrupts/DMA

**Measure analog signals:** analog pin mode, channel selection, sampling time, reference/calibration, trigger source, DMA/interrupts, conversion sequence

**Move data at high speed without CPU involvement:** DMA channel/stream, request routing (DMAMUX if present), buffer addresses/length, transfer width/increment, priorities, completion/error IRQs

---

# All these need lots of parameters to be set (2)

Also:

**Tune performance and power:** clock tree (source/PLL/dividers), per-peripheral clock selection, voltage scaling, flash wait states/cache settings

**Manage low-power operation:** low-power mode selection, wakeup sources, peripheral retention/stop behavior, clock re-init on wake, interrupt configuration

**Built-in security & integrity:** enable/route security peripherals, key/option configuration, access permissions/locking, secure boot/attestation options (device-dependent)

---

# Where do you "set them" ?

[Datasheet](https://www.st.com/resource/en/datasheet/stm32u545ce.pdf)

[Referance manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf)

[STM32 Cortex®-M33 MCUs and MPUs programming manual](https://www.st.com/resource/en/programming_manual/pm0264-stm32-cortexm33-mcus-and-mpus-programming-manual-stmicroelectronics.pdf)

<p align="center">
  <img src="img/reg_intro.png" style="width:80%; display:block; margin:0 auto;" alt="eg_intro">
</p>

---

# Example - CPUID

Zoom in on the CPUID

<p align="center">
  <img src="img/cpuid_reg.png" style="width:100%; display:block; margin:0 auto;" alt="cpuid_reg">
</p>

---

# Super summary on how to set "a pin"

| Step / Layer | What it is (simple) | Typical things you configure | Quick sanity check |
|---|---|---|---|
| **1) Pad / Pin layer** | The *physical pin* and how it behaves electrically | GPIO mode, Alternate Function, pull-up/down, OD vs PP, speed/slew | Is the signal on the right pin **and** with the right electrical behavior? |
| **2) Peripheral engine layer** | The *hardware block* that implements the function (I2C/SPI/UART/TIM/ADC…) | Enable/start, mode, timing/baud, frame format, channels, status/error flgs etc | Does the peripheral generate/interpret the right waveform/data? |
| **3) Infrastructure layer (clock, reset, data paths)** | The chip’s *internal “plumbing”* that makes the peripheral run and move data | RCC: clock source/ PLL/ dividers, E, R; Bus access; NVIC; DMA; routing + buffers | Is the clock ok, and is the cdata path enabled end-to-end? |

---

# General "formula" for setting up "anything"
in 6 easy steps :)

1. **Define goal:** peripheral instance, pins, speed, and transfer method (polling, interrupt, DMA). This prevents wrong assumptions later.

2. **Enable clocks and reset:** configure system clock tree if needed, then RCC enable peripheral and GPIO clocks.

3. **Configure pins:** set GPIO mode to alternate function, choose AF number, and apply electrical settings for the protocol.

4. **Configure the peripheral engine:** mode, timing or baud, frame format, filters, FIFO thresholds, and clear status flags.

5. **Choose the data path:** polling, interrupts via NVIC, or DMA via GPDMA and DMAMUX; configure priorities and buffers.

6. **Start operation:** enable peripheral, start transfer, verify with flags or callbacks, handle errors, and add recovery or timeout.

---

# Step 1

Define the goal

Choose a peripheral instance and pins from the board schematic or datasheet.

Decide required speed, accuracy, and latency.

Pick transfer style: polling, interrupt, or DMA.

List external requirements: pullups, termination, voltage levels.

Write one sentence success criteria for debugging.

---

# Step 2

Clocks and reset (RCC - Reset and Clock Control)

Configure the system clock tree if your peripheral timing depends on it.

Enable GPIO port clock(s) for used pins.

Enable the peripheral clock in the RCC.

Optionally force reset and release reset for a clean start.

Confirm clock source/divider matches desired peripheral frequency.

---

# Step 3

Pins and electrical behavior

Set GPIO mode: AF (Alternate Function) for most peripherals.

Select the correct AF mapping for each pin.

Set output type: push-pull or open-drain (I2C).

Configure pull-up/down and speed/ slew rate.

Verify pin conflicts: the same pin is used by another function / shared timers etc.

---

# Step 4

Peripheral engine configuration

Set operating mode: master/slave, TX/RX, PWM, sampling, etc.

Program timing: baudrate, prescalers, sampling time, or protocol timing.

Configure frame format, word length, parity, stop bits, addressing.

Configure FIFO thresholds, filters, and error behavior.

Clear status flags before starting.

---

# Step 5

Data path (uC busy vs IRQ vs DMA)

Polling: simplest; CPU waits on flags.

Interrupts: enable NVIC line and set priority; handle events in ISR.

DMA: configure GPDMA/LPDMA channel and request routing if present.

Set buffer addresses/length, transfer width, increment, priorities.

Enable completion/error interrupts for IRQ or DMA.

---

# Step 6

Start and verify

Enable the peripheral and start the operation.

*Optionals but good practice:*

Confirm with flags, callbacks, or a scope/logic analyzer.

Handle error flags: clear, retry, or reset peripheral.

Add timeouts to avoid permanent blocking.

Log a minimal status snapshot for debugging.

---

# Now let's look at an example

## Proposed goal
fast update for a TFT display (e.g., 320×240 RGB) 

that means: 320×240 RGB565 frame > 153,600 bytes

at 30 FPS needs => ~4.6 MB/s sustained.

## Tech details
Typically, over a SPI interface

let's do it without CPU busy-waiting => DMA

---

# Settings (1)

The steps:

(Step 1) Define goal: Update TFT region (e.g., 320×240 RGB565) without CPU busy-waiting.

(Step 2) Clocks/reset: 
+ Enable GPIO 

+ SPI kernel clock 

+ DMA clocks 

+ reset SPI for a clean start

(Step 3) Pins: 

+ SCK/MOSI as AF (Alternate Function) push-pull 

+ CS + D/C as GPIO outputs 

+ optional RESET pin

---

# Settings (2)

(more...)

(Step 4) SPI engine set-up: 
+ Master (typically TFT display is slave)
+ correct mode (CPOL/CPHA)
+ 8-bit frames for commands/data
+ high SPI clock
+ other (bit-order, direction etc - usualy left at defaults)

(Step 5) Data path: 
+ Use DMA for pixel payload (set controller, channel, direction, data width, priority, mode, **IRQ**, etc)
+ uc time for command headers.

(Step 6) Init + run (CS, command, DMA, wait, CS, "check image")

---


---
layout: section
---

# GPIO
General Purpose Input Output - Why

---
layout: two-cols
---

# Why GPIO can be complex (phisical output)

### This is a minimalistic representation of a typical GPIO pin (the "pad")

<br>

> the diodes are topical over and under voltage protections

<br>

> C represents the topical "parasitic" capacity 

<br>

> Rpu = Pull-up Resistor with a transistor for on/ off towards the positive supply rail 

:: right ::

<br>
<br>
<br>
<br>


 <img src="./gpio_simple.png" class="h-80 rounded">

---

# Why GPIO can be complex (phisical output)

### This is a minimalistic representation of a typical GPIO pin

<br>

> the diodes are topical over and under voltage protections

<br>

> C represents the topical "parasitic" capacity 

<br>

> Rpu = Pull-up Resistor with a transistor for on/ off towards the positive supply rail 

:: right ::

<br>
<br>
<br>
<br>


 <img src="./gpio_simple.png" class="h-80 rounded">

---
layout: two-cols
---

# Why GPIO can be complex 
### This is a minimalistic representation of a typical GPIO pin

<br>

> the diodes are topical over and under voltage protections

<br>

> C represents the topical "parasitic" capacity 

<br>

> Rpu = Pull-up Resistor with a transistor for on/ off towards the positive supply rail 

:: right ::

<br>
<br>
<br>
<br>


 <img src="./gpio_simple.png" class="h-80 rounded">

---
layout: two-cols
---

# AVR 328P GPIO

- PUD: PULLUP DISABLE
- SLEEP: SLEEP CONTROL
- CLKI/O: I/O CLOCK

- WDx: WRITE DDRx
- RDx: READ DDRx
- WRx: WRITE PORTx
- RRx: READ PORTx REGISTER
- RPx: READ PORTx PIN
- WPX: WRITE PINx REGISTER

:: right ::

<img src="./gpio_328p.png">

---

# Pin on STM32

*Note: this is a simplified block diagram - it looks simpler but it is much more complex as it allows for many advanced functionalities to be mutiplexed on this pin*

<p align="center">
  <img src="img/pin_stm32.png" style="width:80%; display:block; margin:0 auto;" alt="pins_st">
</p>
