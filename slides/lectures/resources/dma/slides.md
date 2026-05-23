---
layout: section
---

# DMA
Direct Memory Access

---

# Bibliography
for this section

1. **Raspberry Pi Ltd**, *[RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)*
   - Chapter 12 - *Peripherals*
     - Chapter 16.6 - *DMA*

2. **STMicroelectronics**, *[STM32U545RE Reference Manual](https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-armbased-32bit-mcus-stmicroelectronics.pdf)*
   - Chapter 17 - *General purpose direct memory access controller*

---
layout: two-cols
---

# DMA

<style>
.two-columns {
    grid-template-columns: 3fr 4fr;
}
</style>

- offloads the MCU from doing **memory to memory** operations
- due to MMIO, usually implies **transfers from and to peripherals**
- raises an interrupt when a transfer is done

<v-click>

⚠️ DMA does not know about the data stored in cache.

</v-click>

<v-click>

- for chips that use cache
  - the DMA buffer's memory region has to be set manually to *nocache* (if MCU knows)
  - or, the cache has to be flushed before and, possibly after, a DMA transfer

</v-click>

:: right ::

<img src="./dma.svg" class="rounded" style="background-color: white; padding: 5px;">

---
layout: two-cols
---

# RP2

- 12 (RP2040) channels or 16 (RP2350) channels
- Transfers
  - Memory to Peripheral
  - Peripheral to Memory
  - Memory to Memory

:: right ::

# STM32U545RE

- 16 General Purpose DMA (GPDMA) channels
- 4 Low Power DMA (LPDMA) channels
- 4 priority levels
- Transfers
  - Memory to Peripheral
  - Peripheral to Memory
  - Memory to Memory
  - Peripheral to Peripheral
