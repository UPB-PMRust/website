---
layout: section
---

# Why Rust

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

# Why Rust - some tech insights

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
