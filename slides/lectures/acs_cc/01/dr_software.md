---
layout: section
---

# Embedded Software 

---

# From bare metal to Linux

<style>
  table {
    font-size: 0.9em;
    border-collapse: collapse;
  }
  th {
    font-weight: bold;
  }
  td, th {
    padding: 4px; /* Reduce padding */
  }
</style>

| Controller | RAM | Flash / storage | Runs | Example OS / framework |
|---|---:|---:|---|---|
| ATmega324 | 2 KB | 32 KB Flash | Bare metal | Superloop + ISR |
| ATmega128 | 4 KB | 128 KB Flash | Bare metal | Superloop + timer tick |
| Microchip SAMD21 | 32 KB | 256 KB Flash | Bare metal, RTOS | FreeRTOS |
| STM32F103C8 | 20 KB | 64 KB Flash | Bare metal, RTOS | FreeRTOS |
| Nordic nRF52832 | 64 KB | 512 KB Flash | Bare metal, RTOS | FreeRTOS + BLE stack |
| ESP32-C3 | 400 KB | 4 MB | RTOS | FreeRTOS (ESP-IDF) |
| RP2350 | 520 KB | 4 MB | BM, RTOS, async | Embassy, FreeRTOS |
| STM32U545RE | 274k | 2 MB Flash | Bare metal, RTOS, async | Embassy, FreeRTOS, Zephyr |
| STM32MP157 | 512 MB | eMMC / SD / NAND | Full OS | Linux (minimal/rootfs) |
| Raspberry Pi | 1–8 GB | SD / eMMC / SSD | Full OS | Linux (Raspberry Pi OS, Ubuntu) |

---

# Why Embedded Software is Different

<v-click>

## It tends to be very application-specific
*  It comes in the form of a blob, which contains data, configuration, application and drivers
* While some operating systems exist for embedded devices, they are very rare

</v-click>

<v-click>


## It uses specialized hardware to achieve its goal
* DSPs for audio/video processing
* On-chip/off-chip peripherals (ADCs/DACs for data acquisition, audio playback, capacitive touch)
* Displays, buttons for user interfaces

</v-click>

<v-click>


## It is much more tightly coupled to hardware than PC/server software
* This allows for smaller binaries but the trade-off is less portable code 
* It must be designed in parallel with the hardware

</v-click>

---

# Why Bare Metal / Async

## Bare metal / async is often the better default
* You control the whole exec model (superloop/interrupts or async tasks) => predictable latency and low jitter
* Lower RAM, flash, & power, because there’s no kernel, scheduler overhead, or background services

## An OS adds overhead and complexity you must “pay for”
* Context switches, tick interrupts, stacks per task, drivers, IPC => CPU/RAM cost and extra failure modes
* More moving parts (scheduler, priorities, locks) => deadlocks, priority inversion, starvation, timing surprises

## Verification, robustness, and debugging are harder with an OS
* Worst-case timing and system state become harder to reason about (more interleavings, shared state, nondeterminism)
* When it fails, it can fail “system-wide” (scheduler stuck, heap fragmentation, task crash) and root cause is harder to reproduce

---

# Hardware Programming & Debugging Devices

<div grid="~ cols-2 gap-20">
<div>

Software tools + hardware tools:
- IDE
- compiler
- programming device/ debugger
- hardware device

Extras:
- oscilloscope
- waveform analyzer
- power analyzer 


</div>

<img src="/img/lauterbach.png" class="w-2500">[^1]

</div>

[^1]: https://wiki.dave.eu/index.php/MITO8M-AN-001:_Advanced_multicore_debugging,_tracing,_and_energy_profiling_with_Lauterbach_TRACE32

---

# Program Flow - ARM vs AVR

<style>
    table {
        width: 100%;
        border-collapse: collapse;
        background-color: #ccd8e6;
    }
    th, td {
        border: 1px solid #000;
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: #add8e6;
        font-weight: bold;
    }
    td {
        vertical-align: top;
    }
    em {
        font-weight: bold;
    }
</style>

| What              | ARM                                                         | AVR                                |
|-------------------|-------------------------------------------------------------|------------------------------------|
| Program Load      | Using an external programmer or bootloader                  | (same)                             |
| Execution launch  | When the microcontroller is reset, execution starts from a preset address | (same)                             |
| Execution threads | Supports multiple threads, multiple values for the Program Counter PC (R15) | Single thread, controlled by PC (Program Counter) |
| In/ Out interaction | Memory mapped I/O                                         | Port-mapped I/O                    |

---

# The code

## How do we program a microcontroller?

<v-click>

1. The code is compiled and a binary file containing the machine code instructions is produced.
- .UF2 / .BIN / .HEX on ARM
- .HEX on AVR

</v-click>

<v-click>

2. The binary must end up in the microcontroller's program memory (Flash) [^1]
- Using an external programmer (In-System Programmer or JTAG)
- using a bootloader 

> The bootloader takes up space in the program memory for AVR (for RPI it resides in ROM).

</v-click>

<v-click>

3. After programming, a RESET is automatically applied to the processor, and it starts execution from the start address. 
> Depending on the configuration (eg where the bootloader is written), it may not be 0.

</v-click>


[^1]: ARM microcontrollers are able to execute code from RAM

---

# In / Out

## No 
- screen :) 
- console :)

## Yes
- LEDs
- LCD
- Serial interface
- Hardware Debugger

---

# Variables

## Allocation


<div grid="~ cols-2 gap-20">
<div>

- Local variables > stack
> Be careful when using recursive functions
- Global variables > data
- Dynamic variables > heap 
> Dynamic variables require an allocator - might not be ideal on an AVR / when you are low on memory 
- Const > flash memory (program memory - written at compile time) 
> Const on AVR can also be stored on EEPROM (slow)


</div>

<img src="/img/stack.png">

</div>

---

# Memory on AVR - 328P example

<style>
    table {
        width: 100%;
        border-collapse: collapse;
        background-color: #ccd8e6;
    }
    th, td {
        border: 1px solid #000;
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: #add8e6;
        font-weight: bold;
    }
    td {
        vertical-align: top;
    }
    em {
        font-weight: bold;
    }
</style>

## ATmega328P Memory Details
| Memory Type             | Size    | Purpose                                      |
|-------------------------|---------|----------------------------------------------|
| Flash (ROM)             | 32 KB   | Stores program instructions (non-volatile).  |
| SRAM (RAM)              | 2 KB    | Stores variables, stack, heap, and registers.|
| EEPROM                  | 1 KB    | Stores persistent data (non-volatile, writable).|
| General Purpose Registers | 32 Bytes | Fast-access CPU registers.                  |
| I/O Registers           | 64 Bytes| Port-mapped peripheral control registers.    |
| Extended I/O Registers  | 160 Bytes | Memory mapped peripheral control registers. |

---

# Memory on ARM - RP2350 example - M33 based

<style>
    table {
        width: 100%;
        border-collapse: collapse;
        background-color: #ccd8e6;
    }
    th, td {
        border: 1px solid #000;
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: #add8e6;
        font-weight: bold;
    }
    td {
        vertical-align: top;
    }
    em {
        font-weight: bold;
    }
</style>
## RP2350 Memory Breakdown
| Memory Type       | Size       | Purpose                                      |
|-------------------|------------|----------------------------------------------|
| XIP [^1] Flash         | Up to 16 MB| Stores program code (external QSPI Flash).   |
| SRAM (On-chip)    | 520 KB     | Stores stack, heap, variables, and data.     |
| Boot ROM          | 32 KB      | Stores bootloader, factory firmware.         |
| OTP               | 8 KB       | One-time-programmable (Product id, cryptographic keys). |
| Peripheral Space  | Varies     | Memory-mapped I/O for GPIO, UART, SPI, DMA.  |
| Registers         | 16 + control registers | General purpose + program flow + special purpose |

<br> 

[^1]: XIP = Execute in Place (without this, the code would need to be copied in RAM first)


---

# Memory on ARM - STM example - M33 based

<style>
    table {
        width: 100%;
        border-collapse: collapse;
        background-color: #ccd8e6;
    }
    th, td {
        border: 1px solid #000;
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: #add8e6;
        font-weight: bold;
    }
    td {
        vertical-align: top;
    }
    em {
        font-weight: bold;
    }
</style>

## STM32U545RE-Q Memory Breakdown
| Memory Type | Size | Purpose |
|---|---:|---|
| Embedded Flash | **512 KB** | Stores program code and non-volatile data in internal Flash. |
| SRAM (on-chip) | **274 KB** | Stores stack, heap, variables, and runtime data. |
| System Memory  | **64 KB** | Factory-programmed ST bootloader (“system memory”). |
| OTP | **512 bytes** | One-time-programmable user data (e.g., IDs/keys). |
| Peripheral Space | Varies | Memory-mapped I/O regions for GPIO, UART, SPI, DMA, etc. |
| Registers | 16 + control registers | General purpose + program flow + special purpose |

---
layout: section
---

# Why Rust-lang

---

# Let's see some code


<v-click>


```c

#include <stdio.h>
#include <stdint.h>

void printBinary(uint32_t num) {
    for (int i = 31; i >= 0; i--) {
        printf("%d", (num >> i) & 1);
        if (i % 8 == 0) printf(" ");  
    }
    printf("\n");
}

int main()
{
    uint8_t a;
    uint32_t b;

    a = 0x01;
    b = a << 24;

    printBinary(a);
    printBinary(b);

    return 0;
}

```
<br> 

</v-click>

:: right ::

<v-click>

<br> <br> 

#### &nbsp;&nbsp;What is the resulting value? 

</v-click>

<v-click>


> &nbsp;&nbsp; it depends on the compiler and on the architecture


</v-click>


<br> 

<v-click>

#### &nbsp;&nbsp; Solution

```c
b = (uint32_t) a << 24;
//b will be 00000001 00000000 00000000 00000000  
//same result on any architecture and compiler;

```

</v-click>

---

# Variables in C

```c

#include <stdio.h>

int8_t, uint8_t
int16_t, uint16_t
int32_t, uint32_t


```

<br> 

# Variables in Rust

```rust
u8, u16, u32, u64, u128
i8, i16, i32, i64, i128
usize //word size (eg - 32b for 32b processor)
isize //word size (eg - 32b for 32b processor)

//NOTES:
char // 4 bytes != u8 //UTF-8 not ASCII like in C
b"str" //ASCII string
"str" UTF-8 string

's' // char
b's' // u8
```

---

# Lets see how C++ is doing

| Link | Memory-safety issue | Why Rust prevents it |
|---|---|---|
| [ZDI-24-854](https://www.zerodayinitiative.com/advisories/ZDI-24-854) | Unchecked AES-key length copied into fixed stack buffer => overflow enables network-adjacent remote code execution. | Safe Rust enforces bounds checks; unchecked stack copies aren’t possible.| 
| [Toyota: single bit flip](https://www.eetimes.com/toyota-case-single-bit-flip-that-killed/) | Stack overflow/memory corruption can kill RTOS task, bypass fail-safes => loss of throttle control (unintended acceleration). | Rust prevents overflows/races; hardware bit-flips and logic bugs remain possible. |
| [CrowdStrike RCA (Channel File 291)](https://www.crowdstrike.com/wp-content/uploads/2024/08/Channel-File-291-Incident-Root-Cause-Analysis-08.06.2024.pdf) | Template field-count mismatch caused out-of-bounds input-array read in sensor, triggering Windows system crash/BSOD.|Rust bounds-checked indexing prevents OOB reads; you get an error instead. |

---

# How do we keep C++ code functional in safety-critical applications?

Lots of tooling and processes

Coding standards / restricted subsets: MISRA C/C++, AUTOSAR C++14, SEI CERT C/C++, JSF AV C++ (plus documented deviations/waivers).

Static analysis & compliance checking: rule checkers + dataflow analyzers (e.g., Polyspace, Coverity/CodeSonar/Parasoft, clang-tidy) integrated in CI.

Compiler hardening & build gates: warnings-as-errors, stack protection, fortified libc checks / hardening bundles (e.g., _FORTIFY_SOURCE, -fstack-protector-strong, -fhardened).

Dynamic bug finding (test builds): sanitizers for memory/UB/races (ASan/UBSan/TSan), plus coverage-guided fuzzing (libFuzzer).

Safety evidence overhead: mandated reviews + traceability + V&V activities (ISO 26262 / DO-178C / IEC 61508-style workflows).

---

# Why Rust-lang - some tech insights

### The tagline of Rust is No Undefined Behavior. 

- no null reference; the Rust compiler explicitly asks developers to check
this;
- no implicit cast, even adding a u32 to a u8 must be casted;
-  safe access to shared data across threads verified at compile time;
- uses type states to move runtime checks to compile time and force
developers to check;
- clearly defined data types, unlike i8 or u128;
- safe unions, that provide a safeguard to prevent wrong interpretation
of data;
- clear code organization into crates and modules;
- backward compatibility at crate level.

---

# Does Rust remove the need for tooling?

No, but it sure makes code safer and dev faster 

Rust’s advantage is biggest in safe Rust.

As unsafe/FFI grows, assurance shifts back to: unsafe policy & reviews, FFI boundary correctness, sanitizers/fuzzing on risky edges, dependency/toolchain governance; plus the same ISO/DO-178 traceability & V&V.

[Rust in Android: move fast and fix things](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html) 

- A 1000x reduction in memory safety vulnerability density compared to Android’s C and C++ code
- With Rust changes having a 4x lower rollback rate and spending 25% less time in code review, the safer path is now also the faster one

---

# Who supports Rust-lang

## Some links to read

[the NSA: U.S. and International Partners Issue Recommendations to Secure Software Products Through Memory Safety](https://www.nsa.gov/Press-Room/Press-Releases-Statements/Press-Release-View/Article/3608324/us-and-international-partners-issue-recommendations-to-secure-software-products/)

[The White House: Back to the building blocks: A path toward secure and measurable software](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)


[How Rust went from a side project to the world’s most-loved programming language](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)

[On Rust-lang adoption based on git-hub adoption](https://innovationgraph.github.com/global-metrics/programming-languages)


[Rust developers at Google are twice as productive as C++ teams](https://www.theregister.com/2024/03/31/rust_google_c/)

## Some companies that are building up Rust teams in embedded:
- Airbus, Ampere, Bae Systems, Boeing, Ford, General Dynamics, Hyundai, Northrop Grumman, NXP, Thales, Toyota, Volvo
