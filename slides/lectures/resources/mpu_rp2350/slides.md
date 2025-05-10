---
layout: section
---

# MPU for RP2350
Protected Memory System Architecture v8 (PMSAv8)


---
---
# Bibliography
for this section

**Joseph Yiu**, *The Definitive Guide to ARM® Cortex®-M23 and Cortex-M33 Processors* 
   - Chapter 6 - *Memory System*
     - Subchapter 6.4 - *Access Permission Management*  
   - Chapter 12 - *Memory Protection Unit (MPU)*

---
layout: two-cols
---

# MPU for RP2350
Cortex-M33 works in three modes

- **handler** mode - *no restrictions* - used while executing ISRs and Exception Handlers
- **thread** mode
  - **privileged** *no restrictions* - usually used for the operating system
  - **unprivileged** mode - *used for applications*, allows only ALU and memory access through:
     - Default Access Permission - *restricts unprivileged access to the Cortex-M Peripherals*
     - Memory Protection

MPU allows 8 regions
- permissions R W X

:: right ::

<img src="./mpu_rp2350.svg" class="w-120 rounded">

---
layout: two-cols
---

# Memory Protection Unit
Cortex-M MPU (PMSAv8)

<style>
.two-columns {
    grid-template-columns: 3fr 2fr;
}
</style>

<img src="./mpu.svg" class="w-120 rounded">

- allows the definition of *memory regions*
- regions <span v-mark.underline.red>cannot overlap</span>
- regions have access permissions (similar to rwx)

$$ region\_size = 32 \times N $$
$$ base\_address = 32 \times N $$

:: right ::

<img src="./mpu_regions.svg" class="w-70 m-5 rounded">

---

# Memory Protection Unit
Access Protection

<div grid="~ cols-2 gap-20">
<div>

<img src="./mpu.svg" class="w-120 rounded">

**AP** Access Protection

**XN** eXecute Never 
  - faults if MCU has to read the next instruction from an *XN* region

**SH** Shared between cores and peripherals

**Attributes** *used for cache*

</div>
<div>

| **AP** | Privileged Mode | Unprivileged Mode |
|-------|------------|--------------|
| `00` | Read/Write | No Access |
| `01` | Read/Write | Read/Write |
| `10` | Read only | No Access |
| `11` | Read only | Read only |

Better granularity -> there is no need for *No Access* in privileged mode.

> There is no need to overlap regions to obtain the required protected memory space.

</div>
</div>
