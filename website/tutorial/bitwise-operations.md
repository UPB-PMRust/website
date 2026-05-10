---
sidebar_position: 1
description: Bitwise operations to interact with hardware registers in Rust
---

# Bitwise Operations in Rust

When developing embedded software for the **STM32 Nucleo-U545RE-Q**, you are frequently required to configure specific hardware peripherals. Whether you are turning on one of the 5 LEDs or reading the state of the 4 buttons  on your lab board, you communicate with the microcontroller by modifying specific bits within its hardware registers.

Because hardware registers group multiple independent configuration settings into a single 32-bit (or 8-bit) word, you cannot simply overwrite the entire register without risking the disruption of other settings. This is where bitwise operations come in.

## The Anatomy of a Hardware Register

Registers in microcontrollers generally consist of bits grouped by their specific roles. Here is what those bits actually do under the hood:

- **Control bits**: These are used to control different operating modes of the microcontroller or to activate specific hardware components. For example, setting a specific control bit might turn on the UART communication peripheral.
- **Status bits**: These are generally read-only bits used to check the state of an action or hardware peripheral. For instance, a status bit might turn to `1` automatically when new data has arrived in a buffer.
- **Data bits**: These bits are used to provide the microcontroller with data to be processed, or to retrieve data from it.
- **Reserved bits**: These bits are not currently used by the microcontroller. As a general rule in embedded programming, you should never modify reserved bits.


## Bit Shifting

Before modifying registers, you need to know how to target a specific bit. We do this by taking the number `1` and "shifting" it to the correct position.

- Right Shift (`>>`): This moves each bit of the operand to the right by a specified number of positions. Zeros are added to the left to maintain the dimension of the number. Mathematically, shifting right is a highly efficient way to divide a binary number by $ 2^n $ in a single operation.

:::info

`1010 >> 1 = 0101` (In decimal: 10 divided by 2 becomes 5).

:::

- Left Shift (`<<`): Moves bits to the left, filling the empty spaces on the right with zeros. Shifting left by n is mathematically equivalent to multiplying by $ 2^n $.

:::info

`0011 << 1 = 0110` (In decimal: 3 multiplied by 2 becomes 6).

:::

## Modifying Registers

Here are the primary operations you will use daily to manipulate bits without corrupting the rest of the register.

### Setting Bits (Making them `1`)

To set a specific bit to `1` while leaving the rest of the register untouched, we use the bitwise **OR** (`|`) operator.

- **Set a single bit**: `register | 1 << bit`
- **Set multiple bits**: `register | bits`

### Clearing Bits (Making them `0`)

To clear a specific bit to `0`, we use a combination of the bitwise **AND** (`&`) and bitwise **NOT** (`!`) operators.

- **Clear a single bit**: `register & !(1 << bit)`
- **Clear multiple bits**: `register & !bits`

### Toggling / Flipping Bits

If you need to invert the current state of a bit (change `1` to `0`, or `0` to `1`), use the bitwise **XOR** (`^`) operator.

- **Flip a single bit**: `register ^ (1 << bit)`
- **Flip multiple bits**: `register ^ bits`

## Value Extraction(Masking)

Often, a hardware register contains a specific multi-bit "field" (like a 4-bit configuration value) packed into a larger 32-bit word. To read just that field, you need to shift the bits down to the zero position and apply a **Mask** to zero-out everything else.

Here is a practical example of extracting a specific portion of a 32-bit ID:

```rust
// We define a mask that isolates the bottom 12 bits
const MASK: u32 = 0b0000_0000_0000_0000_0000_1111_1111_1111;

fn main() {
    // A 32-bit register value
    let large_id: u32 = 0b1100_1010_1111_1100_0000_1111_0110_1101;
    
    // 1. Shift right by 20 to bring the target bits to the bottom
    // 2. Apply the AND mask to clear all upper bits
    let extracted_bits = (large_id >> 20) & MASK; 

    // Result: 00000000_0000_0000_0000_1100_1010_1111 
}
```

## Common Pitfalls

Bitwise operations are prone to logical typos. Watch out for these common errors:

:::warning 

Using the assignment operator (`=`) instead of the compound bitwise operator (`|=` or `&=`).

- **Wrong**: `register = 1 << 4` (This wipes out the entire register and only sets bit 4)
- **Correct**: `register |= 1 << 4` (This preserves the rest of the register while setting bit 4).

:::

:::warning 

Do not confuse **logical** operators with **bitwise** operators!

- `&&` and `||` evaluate truthiness (e.g., `true && false`)
- `&` and `|` perform mathematically accurate bit-by-bit operations (e.g., `0b1100 & 0b0101 = 0b0100`).

:::
