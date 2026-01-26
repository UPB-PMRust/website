# SpotiPi Record Player
This project recreates a version of a mini vinyl record player using RFID cards, a Raspberry Pi Pico 2 W, and Spotify playback. RFID cards act as "records". When a card is scanned, the system starts playing a corresponding album via Spotify.

## Features
- RFID-based music selection
- Raspberry Pi Pico 2 W (MicroPython)
- Wi-Fi communication
- Spotify API integration
- Modular architecture

## Motivation
Traditional music playback lacks physical interaction. This project aims to reintroduce
tangible media interaction using modern streaming services, and can work as a meaningful (and cheap) gift for one's friends.

## Technologies Used
- Raspberry Pi Pico 2 W
- MicroPython
- RFID (RC522)
- Wi-Fi
- Spotify Web API
- Python (Flask, Spotipy)

## Architecture
<img width="587" height="550" alt="image" src="https://github.com/user-attachments/assets/88ece040-7064-4e46-93f1-fca095626a19" />

The system is divided into two main components:

### 1. Embedded Device (Pico)
- Reads RFID card UID
- Connects to Wi-Fi
- Sends UID to server via HTTP

### 2. Host Server
- Receives UID
- Maps UID to Spotify URI
- Controls playback using Spotify API

## Software Design

### Pico Firmware
- Written in MicroPython
- Uses SPI for RFID
- Uses HTTP POST for communication

### Server Software
- Python Flask application
- Spotipy library for Spotify API
- UID-to-playlist mapping

## Hardware Configuration

### Components
- Raspberry Pi Pico 2 W
- RC522 RFID Reader
- RFID (NFC) Cards
- Breadboard and jumper wires
- (optional) LEDs for testing

### Wiring

| RC522 Pin | Pico 2 W Pin |
|---------|--------------|
| SDA | GP17 |
| SCK | GP18 |
| MOSI | GP19 |
| MISO | GP16 |
| RST | GP20 |
| 3.3V | 3V3 |
| GND | GND |

### Hardware
![WhatsApp Image 2026-01-22 at 03 52 36](https://github.com/user-attachments/assets/9eda1e75-00cc-44bb-9b27-dede2f083d37)

<img width="820" height="752" alt="image" src="https://github.com/user-attachments/assets/ea6a4828-e7c8-432f-baa9-b258d2e45907" />

## Bill of Materials

| Device | Usage | Price (Lei) |
|--------|-------|-------------|
| NFC Tags | Vinyl Cards | 40 |
| USB to USB cable | Connecting Pico to Laptop | 38 |
| RFID RC522 Reader | Reads NFC Card | 25 |
| Raspberry Pi Pico | Reads UID & sends it to server | 30 |
| Wires | Connection | 25 |
| Solder Kit | Connection | 38 |
| **Total** | | **196** |

## Software Architecture

Spotipi/
├── firmware/
│   └── src/
│       └── main
└── host/
    ├── player
    └── uid_storage
    
### Flow
- Scan mini-album
- UID is detected
- Open app & play album

### Result
<img width="1155" height="134" alt="image" src="https://github.com/user-attachments/assets/da6f58fe-6a35-4923-a687-f2c1b65143d4" />

## Setback
Spotipi relies on Spotify and user Spotify API that is exclusively obtainable via a Spotify Developer account and app. Spotify has recently disabled app creation indefinetly, which didn't line up with the project deadlines. 
As a temporary solution, UID scanning will take the user to the YouTube equivalent of the playlist, which doesn't require an API, or even a Premium account. Once this issue is resolved, Spotipi will be modified to reflect its original intended functionality.

## Limitations

- Spotify playback control requires a Premium account
- Host device must be connected to the same local network
- The system currently supports only basic “play” actions for predefined tags

## Links
- https://www.youtube.com/watch?v=-jGWjFR936o&t=600s
