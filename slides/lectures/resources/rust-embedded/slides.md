---
layout: section
---
# Rust Embedded HAL
The Rust API for embedded systems

---
---
# The Rust Embedded Stack

<div grid="~ cols-2 gap-4">
<div>

| | |
|-|-|
| Framework | Tasks, Memory Management, Network etc. `embassy-rs`, `rtic` |
| BSC | Board Support Crate `embassy-rp`, `rp-pico`, `embassy-stm32` |
| *HAL Implementation* | Uses the PAC and exports a standard HAL towards the upper levels `embassy-stm32` |
| PAC | Accesses registers, usually created automatically from SVD files - `rp-pac`, `stm32-metapac` |
| μ-architecture | `cortex-m` processor startup |


</div>

<div align="center">
    <img src="./rust_embedded_stack.svg" class="w-120 rounded">
</div>

</div>

---
---
# GPIO HAL
A set of [standard traits](https://docs.rs/embedded-hal/latest/embedded_hal/)

All devices should implement these traits for GPIO.

```rust
pub enum PinState {
    Low,
    High,
}
```

<div grid="~ cols-2 gap-2">

<div>

##### Input

```rust {*}{lines:false}
pub trait InputPin: ErrorType {
  // Required methods
  fn is_high(&mut self) -> Result<bool, Self::Error>;
  fn is_low(&mut self) -> Result<bool, Self::Error>;
}
```

</div>

<div>

##### Output

```rust {*}{lines:false}
pub trait OutputPin: ErrorType {
  // Required methods
  fn set_low(&mut self) -> Result<(), Self::Error>;
  fn set_high(&mut self) -> Result<(), Self::Error>;

  // Provided method
  fn set_state(
    &mut self,
    state: PinState
  ) -> Result<(), Self::Error> { ... }
}
```
</div>

</div>


---

# Bare metal

<!-- ToDoDanut: pp ca de aici doar mici modificari like use-ul de mai jos? -->

This is how a Rust application would look like

<div grid="~ cols-2 gap-4">

```rust{all|1|2|4,5|7|8,12|11|14-17}
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    // your code goes here

    loop { }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop { }
}
```

<div>

### Rules
1. never exit the `main` function
2. add a panic handler that does not exit

</div>

</div>

---

# Bare metal without PAC & HAL
This is how a Rust application would look like

<div grid="~ cols-2 gap-2">

```rust{all}
#![no_std]
#![no_main]

use core::ptr::write_volatile;
use core::ptr::read_volatile;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
  loop { }
}

const PORT_x: u32 = ...;  // A is 0, B is 1, ...
const PIN: u32 = ...;     // Ranging from 0 to 15

const GPIOx_BASE_ADDR: usize =
  0x4202_0000 + 0x400 * PORT_x;
const GPIOx_MODER: *mut u32 = 
  GPIOx_BASE_ADDR as *mut u32;
const GPIOx_ODR: *mut u32 = 
  (GPIOx_BASE_ADDR + 0x14) as *mut u32;
```

```rust{all}{startLine:21}
const RCC_BASE_ADDR: usize = 0x4602_0C00;
const RCC_AHB2ENR1: *mut u32 = 
  (RCC_BASE_ADDR + 0x8C) as *mut u32;

#[cortex_m_rt::entry]
fn main() -> ! {
  unsafe {
    let mut val = read_volatile(RCC_AHB2ENR1);
    let val = val | (1 << PORT_x);
    write_volatile(RCC_AHB2ENR1, val);
  }

  unsafe {
    let val = read_volatile(GPIOx_MODER);
    let mask = !(0b11 << (2 * PIN));
    let val = (val & mask) | (0b01 << (2 * PIN));
    write_volatile(GPIOx_MODER, val);
    write_volatile(GPIOx_ODR, 1 << PIN);
  }

  loop { }
}
```

</div>
