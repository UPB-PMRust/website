/* Raspberry Pi Pico / RP2040 memory layout */
MEMORY {
    /* NOTE: RP2040 boots from external QSPI flash.
       The first 256 bytes are reserved for the second-stage bootloader. */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    /* Second-stage bootloader – must be the very first section in flash. */
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
