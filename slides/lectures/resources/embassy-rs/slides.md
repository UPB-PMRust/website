---
layout: section
---
# embassy-rs
[Embedded Asynchronous](https://embassy.dev/)

---
---
# embassy-rs

- framework
- uses the rust-embedded-hal
- Features
  - Real-time
  - Low power
  - Networking
  - Bluetooth
  - USB
  - Bootloader and DFU

---

# GPIO Input

<div grid="~ cols-2 gap-2">

<div>

RP2

```rust {none|5|8,9,18|10|6,11-17|all}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Input, Pull};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let pin = Input::new(p.PIN_3, Pull:Up);

    if pin.is_high() {

    } else {

    }
}
```

</div>

<div>

STM32U545RE

```rust {none|5|8,9,18|10|6,11-17|all}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio;
use gpio::{Input, Pull};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let pin = Input::new(p.PC13, Pull::Down);

    if pin.is_high() {

    } else {

    }
}
```

</div>

</div>

The `main` function is called by the embassy-rs framework, so it can exit.

---
---
# GPIO Input for RP2 vs STM32U545RE

````md magic-move

```rust
// RP2

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Input, Pull};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let pin = Input::new(p.PIN_3, Pull:Up);

    if pin.is_high() {

    } else {

    }
}
```

```rust
// STM32U545RE

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio;
use gpio::{Input, Pull};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let pin = Input::new(p.PC13, Pull::Down);

    if pin.is_high() {

    } else {

    }
}
```

````

---
---

# GPIO Output

<div grid="~ cols-2 gap-2">

<div>

RP2

```rust {none|5|8,9,14|10|6,11-13|all}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Level, Output};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pin = Output::new(p.PIN_2, Level::Low);

    pin.set_high();
}
```

</div>

<div>

STM32U545RE

```rust {none|5|8,9,18|10|6,11-17|all}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio;
use gpio::{Level, Output, Speed};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut pin = Output::new(
        p.PA5,
        Level::Low,
        Speed::Medium
    );

    pin.set_high();
}
```

</div>

</div>

The `main` function is called by the embassy-rs framework, so it can exit.

---
---

# GPIO Output for RP2 vs STM32U545RE

````md magic-move

```rust
// RP2

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Level, Output};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pin = Output::new(p.PIN_2, Level::Low);

    pin.set_high();
}
```

```rust
// STM32U545RE

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio;
use gpio::{Level, Output, Speed};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut pin = Output::new(p.PA5, Level::Low, Speed::Medium);

    pin.set_high();
}
```

````
