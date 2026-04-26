# RustShips v1.0

A two-player Battleship game running on devices having a 4" touchscreen display and wireless peer-to-peer communication.

:::info 

**Author**: Mîrza Daniel \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/fils-project-2026-Danum1z

:::

## Description

RustShips is a fully embedded, wireless two-player Battleship game. Each player owns a dedicated device consisting of an STM32U545RE Nucleo-64 board connected to a 4" TFT touchscreen (ST7796S, 320×480) wtih action buttons and a passive buzzer. The two devices communicate wirelessly via NRF24L01+ radio modules over SPI.

Each player places a fleet of 5 ships on a 10×10 grid, then takes turns firing at the opponent's grid. The game tracks two matrices per device: `my_fleet` (own ships and incoming hits) and `enemy_radar` (shots fired and confirmed results). A dedicated button toggles between the two views.

The game features three special mechanics, each usable once per game:
- **Sonar Scan**: hold the action button for 3 seconds to reveal a 3×3 area of the enemy grid for 2 seconds
- **Airstrike**: double-press the action button to instantly sink an entire enemy ship
- **Ferris Repair**: named after Ferris the Rust mascot — fully repair a damaged ship and move it to a new position

The entire game logic is written as a `no_std` Rust library (`game-core`) that compiles for both the real MCU and a desktop simulator built with `embedded-graphics-simulator`.


## Motivation

The choice of this project comes from a long-standing interest in classical strategy games like Battleships and a desire to translate tactile, paper-based mechanics into a robust digital embedded system. Beyond the childhood nostalgia, this project serves as a comprehensive learning platform for several reasons:

- **Software Complexity**: Developing a full-featured game from scratch in Rust requires mastering state machines and no_std logic.
- **Hardware Integration**: The project offers hands-on experience with SPI bus sharing between high-resolution displays, touch controllers, and wireless radio modules.
- **Future Potential**: This handheld terminal is designed as a versatile gaming platform. Its low energy consumption and independent peer-to-peer wireless link make it ideal for offline use (trains, planes, or remote trips). It establishes a foundation for a suite of offline multiplayer games like or Connect 4, Tic-Tac-Toe, or even singleplayer Sudoku.

## Architecture 

The project is split into three crates inside a Cargo workspace:

**`game-core`** — `no_std` pure logic library, no hardware dependencies. Compiles for both x86_64 (simulator) and `thumbv8m.main-none-eabihf` (MCU). Contains the grid, ship fleet, game state machine, special mechanics, and network message types.

**`simulator`** — `std` desktop application. Uses `embedded-graphics-simulator` (SDL2) to render the game in a window on the development laptop. Uses the exact same `game-core` types as the real hardware. Used for developing and testing all game logic before using hardware.

**`embassy-app`** — `no_std` Embassy application for the real hardware. Spawns independent async tasks that communicate via Embassy channels and signals.

**Game State Machine (per device):**
```text
Placement ──► WaitingForOpponent ──► Battle ──► GameOver
                                        ├── Aiming
                                        ├── WaitingForResult
                                        ├── OpponentTurn
                                        └── FerrisRepair                        
```

**Communication protocol** (NRF24L01+, peer-to-peer, no server):

Each game action is serialized into a `NetworkMessage` and sent to the opponent. Message types include `FireShot`, `ShotResult`, `SonarRequest/Response`, `AirstrikeResult`, `FerrisRepairUpdate`, and `GameOver`.

```mermaid
graph TD
    subgraph "Handheld Unit (x2)"

    MCU["STM32U545RE Nucleo"]
    DISP["4.0 Inch ST7796S Display"]
    TOUCH["XPT2046 Touch"]
    RADIO["NRF24L01+ Radio"]
    BUZZ["Passive Buzzer"]
    BTNS["Action Buttons"]
    
    MCU -->|SPI Bus| DISP
    MCU -->|SPI Bus| TOUCH
    MCU -->|SPI Bus| RADIO
    MCU -->|PWM| BUZZ
    BTNS -->|GPIO Interrupts| MCU
    end
    
    RADIO <-->|2.4GHz Wireless| RADIO2["Opponent Device"]
```

## Log

### Week 5 - 7

- Searched for a suitable project idea
- Adopted the BattleShips game concept
- Defined the project logic and workflow

### Week 8 - 9

- Chose STM32U545RE + Embassy async Rust stack
- Ordered two 4.0"SPI TFT Modules
- Set up the Cargo workspace with `game-core`, `simulator`, and `embassy-app` crates
- Implemented the `grid.rs`, `ship.rs`, and `state.rs` (in `game-core`)
- Received the ordered TFT modules
- Ordered two NUCLEO-U545RE-Q boards
- Passed the Documentation Milestone

### Week 10 - 11

- _to be continued..._

### Week 12 - 13

- _to be continued..._

## Hardware

Each player's device consists of:

- **STM32U545RE-Q Nucleo-64**: Arm Cortex-M33 microcontroller running at up to 160 MHz, 512 KB flash, 274 KB SRAM. Runs the Embassy async runtime with multiple concurrent tasks.
- **4.0" TFT LCD (ST7796S, 320×480)**: Connected via SPI. Displays both game grids, ship positions, hit/miss markers, and special ability highlights. Driven by the `mipidsi` crate with `embedded-graphics`.
- **XPT2046 resistive touch controller**: Shares the SPI bus with the display (separate CS pin). Used for cell selection on the touchscreen.
- **NRF24L01+ radio module**: Connected via SPI. Provides 2.4 GHz peer-to-peer wireless communication between the two devices. No server or router required.
- **Passive buzzer**: Driven via PWM. Provides audio feedback for shots, hits, sinks, special abilities, and win/lose outcomes.
- **Navigation buttons**: Arrow buttons (up/down/left/right) for cursor movement and ship placement, plus Action, View-Toggle, and Rotate buttons.

### Schematics

_KiCAD schematic to be uploaded later..._

### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [STM32U545RE Nucleo-64](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-U545RE-Q?qs=mELouGlnn3cp3Tn45zRmFA%3D%3D&countryCode=RO&currencyCode=RON) | Main microcontroller board (×2) | 130 RON × 2 |
| [4.0" TFT LCD ST7796S 320×480](https://www.amazon.es/-/en/gp/product/B0CQ87KN3Q/ref=ox_sc_act_title_1?smid=A3E5QOOXTLF93M&th=1) | Touchscreen display (×2) | 130 RON × 2 |
| NRF24L01+ module* | 2.4GHz wireless link (×2) | __ RON × 2 |
| Passive buzzer* | Audio feedback (×2) | __ RON × 2 |
| Tactile push buttons* | Navigation + action input (×2 sets) | __ RON × 2 |
| Jumper wires + breadboard | Prototyping connections | ~30 RON |
| **Total (estimated)** | | **~~ 550 RON** |

_*to be ordered..._

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy](https://github.com/embassy-rs/embassy) | Async embedded framework | Task scheduler, async SPI/GPIO/PWM drivers for STM32U5 |
| [mipidsi](https://github.com/almindor/mipidsi) | MIPI display driver | Drives the ST7796S TFT display over SPI |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Draws grids, ships, cursors, highlights, text |
| [embedded-graphics-simulator](https://github.com/embedded-graphics/simulator) | SDL2-based simulator | Runs the full game on laptop for logic testing |
| [xpt2046](https://crates.io/crates/xpt2046) | Resistive touch driver | Reads touch coordinates from XPT2046 controller |
| [heapless](https://github.com/japaric/heapless) | Fixed-size collections | Stack-allocated Vec/Queue replacements for no_std |
| [defmt](https://github.com/knurling-rs/defmt) | Embedded logging | RTT-based debug logging via probe-rs |

## Links

1. [Battleship - Wikipedia](https://en.wikipedia.org/wiki/Battleship_(game)): General rules and history of the classic board game.
2. [Sea Battle (App Store)](https://apps.apple.com/ro/app/sea-battle-online/id884947296?l=ro): A popular mobile version that inspired the digital UI and special mechanics.
3. [Embassy - Async Rust for Embedded](https://embassy.dev/): Documentation for the framework powering the game's concurrency.
