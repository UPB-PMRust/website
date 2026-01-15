---
layout: section
---
# Memory Protection
ARM: MPU, RISC-V: PMP

---
layout: two-cols
---
# Memory Protection

<style>
.two-columns {
    grid-template-columns: 2.5fr 3fr;
}
</style>

memory access defined region by region

- **restricts** access to **physical memory**
- uses **physical addresses**

The processor works in three modes:
- **machine** mode (*optional*) - used at boot, allows access to everything
- **supervisor** mode - restricts access to some registers and accesses memory through Memory Protection EL2 (*if machine mode exists*)
- **user** mode - allows only ALU and memory access through Memory Protection


:: right ::

<img src="./memory_protection.svg" class="w-120 rounded">
