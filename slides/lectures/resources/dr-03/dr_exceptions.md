---

---
layout: section
---

# Intro into exceptions

---

# Intro - overview on M33

Exceptions vs device interrupts (NVIC)
- **Exception (umbrella term):** any event that diverts normal instruction flow into a handler.
- Two sources of exceptions:
  1) **System exceptions (core-defined):** fixed architectural meanings (reset, faults, SVC, SysTick, etc.).
  2) **Device interrupts (peripheral IRQs):** MCU-specific sources (TIM, UART, DMA, ADC, GPIO…), routed via **NVIC**.
- Both use **vectored dispatch**: the core reads a handler address from the **vector table** and branches to it.
- Key properties: **pending/active**, **priority**, and **preemption** (higher priority can interrupt lower priority code/ISR).

---

# System exceptions on M33 
A quick summary

**Special**
- **Reset**: entry point after reset (loads initial SP + Reset_Handler from vector table)
- **NMI (Non-Maskable Interrupt):** very high priority, not maskable (e.g., critical external signal)

**Fault exceptions (“Huston, we have a problem")**
- **HardFault**: catch-all when a fault can’t be handled more specifically
- **MemManage / BusFault / UsageFault**: configurable fault types (memory protection, bus errors, illegal instruction/state)
- **SecureFault** (M33 feature when TrustZone is used): security state violation / secure attribution issues

---

# System exceptions on M33 

(more)

**System services / scheduling**
- **SVCall (SVC):** software-triggered request for privileged service
- **PendSV:** software-pendable exception (commonly used for RTOS context switching)
- **SysTick:** system timer exception (periodic tick / timing base)

**Debug**
- **DebugMonitor** (when enabled): debug-related exception (break/watchpoint flows depend on debug config)

---

# Extra (special cases)

Hardware events that may NOT interrupt the CPU

Not every peripheral event becomes an IRQ. Many MCUs let events be routed internally:
- **Timer > ADC trigger:** precise sampling rate without CPU jitter (ADC starts on timer event)
- **Peripheral > DMA request:** data moves in hardware; CPU only gets an IRQ on “transfer complete/error”
- **Peripheral-to-peripheral triggers:** capture/compare, start/stop, sync chains via internal trigger mux/event fabric
- **Sleep/wakeup events:** some sources can wake the core; handling may happen later via normal code or an IRQ
**Idea:** use interrupts for *control decisions*, but use hardware events/DMA for *high-rate data movement*.



---

# NVIC (Nested Vectored Interrupt Controller)
Short intro
- **NVIC is hardware inside the Cortex-M core** that arbitrates **device interrupts (IRQs)** from peripherals
- **Who decides priorities?** "you" set IRQ priorities into **NVIC priority registers** (`NVIC->IPR[]`).
  - **Rule (Cortex-M):** *lower numeric value = higher priority* (e.g., 0 is “highest”).
- NVIC tracks for each IRQ:
  - **Enabled?** (`NVIC->ISER/ICER`) — masked/unmasked at controller level
  - **Pending?** (`NVIC->ISPR/ICPR`) — requested but not yet serviced
  - **Active?** (`NVIC->IABR`) — currently running ISR
- **What NVIC + core do automatically:** if an IRQ is **pending + enabled** and has **higher priority** than the current execution priority, the **core performs exception entry** and jumps via the **vector table** to the ISR.

<br> 

>Note:  *System exceptions are separate: priorities for SysTick/SVC/PendSV/etc. are set via SCB , while Reset/NMI/HardFault are special (not handled like normal device IRQs).*

---

# Example checklist to activate an intrerupt on M33


**Typical enable sequence**
1) Configure the **peripheral event** (e.g., UART RXNE, TIM update/compare, ADC EOC, DMA TC).  
2) Clear any **peripheral pending flag** *before* enabling (many are W1C = write-1-to-clear).  
3) Enable the interrupt **inside the peripheral** (its local “interrupt enable” bit).  
4) Set priority in NVIC: `NVIC_SetPriority(IRQn, p)`  *(0 = highest; only top N bits implemented).*  
5) Clear NVIC pending (optional but common): `NVIC_ClearPendingIRQ(IRQn)`.  
6) Enable in NVIC: `NVIC_EnableIRQ(IRQn)`.  
7) Ensure global masking isn’t blocking: `__enable_irq()` and avoid unintended `BASEPRI/PRIMASK`.

---

# Critical note on IRC
This is valid for all embedded development - no matter the hardware, language, or system design

- An **ISR is a separate execution context** that can **preempt** `main()` at almost any instruction (think: asynchronous, higher-priority code path).
- Communication between an ISR and main - is usually implemented through **shared state** (flags, counters, ring buffers), but you must handle **compiler optimizations** and **race conditions**:
- Strong recommendation: use **`volatile`** - that tells the compiler that a variable may change “outside” the current flow, so each access must be a real **load/store** (no caching in registers, no removing “redundant” reads).

---

# Notes on volatile

- `volatile` is great for memory-mapped I/O registers and simple flags touched by ISR/hardware.
- `volatile` is NOT enough for safety:it does not make operations atomic, does not prevent torn reads/writes, and does not provide mutual exclusion.

``` c
volatile uint64_t time_us;   //Note: on a 32bit arhitecture this is possible, but takes 2 cycles
void TIM2_IRQHandler(void) {
    time_us += 100;         
}
uint64_t read_time_us_unsafe(void) {
    return time_us;
}
```


- For correctness use: **atomics** (flags/counters), or **critical sections** (temporarily disable interrupts) for multi-step updates, or **queues/ring buffers** (often DMA-driven), or RTOS primitives.
> If you omit `volatile`/atomics, code like `while(flag==0){}` may be optimized into an **infinite loop**, because the compiler assumes `flag` never changes.

---

# Embassy


- **Embassy** is an **async/await framework + drivers** for **embedded Rust** (`no_std`) that lets you write firmware as **concurrent tasks** instead of one giant loop.
- It provides a small **executor** (runtime) that schedules **async tasks** cooperatively: a task runs until it hits an `await`, then yields.
- Hardware events (IRQs, DMA completion, timers) **wake** the right task (via wakers), so you don’t busy-wait.

<br> 

> **Why it’s useful:** clean structure for multiple I/O flows, good power (sleep while waiting), less “state-machine spaghetti”.

*Note: It is not preemptive like an RTOS! => CPU-hog task can stall others, so avoid long loops and blocking calls (e.g., a compute-heavy loop with no await can starve an async UART RX task - and result in FIFO overflows / dropped bytes).*

---

# Exemple - input from GPIO trg, output on a GPIO

```rust
// #![no_std] #![no_main] use embassy... stm32 .. sync .. time etc
use {defmt_rtt as _, panic_probe as _}; //NOTE >> BELOW - BAD FORMATING TO COMPRESS IN 1 SCREEN

static SENSOR_EVENT: Signal<NoopRawMutex, ()> = Signal::new();
bind_interrupts!(pub struct Irqs {EXTI15_10 => exti::InterruptHandler<interrupt::typelevel::EXTI15_10>; });

#[embassy_executor::task]
async fn sensor_task(mut sensor: ExtiInput<'static, Async>) {
    loop { sensor.wait_for_rising_edge().await.unwrap();
        SENSOR_EVENT.signal(()); }} //bad formating to save screen space ;) 

#[embassy_executor::task]
async fn command_task(mut cmd: Output<'static>) {loop { SENSOR_EVENT.wait().await;
        cmd.set_high();
        Timer::after_millis(50).await;
        cmd.set_low(); }}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let sensor = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down, Irqs);
    let cmd = Output::new(p.PA5, Level::Low, Speed::Low);
    spawner.spawn(sensor_task(sensor)).unwrap();
    spawner.spawn(command_task(cmd)).unwrap();  }
```
