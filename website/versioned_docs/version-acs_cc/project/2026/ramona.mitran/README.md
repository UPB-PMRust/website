## Architecture 

The system is built around a central microcontroller that interfaces with various input and output modules. Below is the logical data flow of the weather station:

```mermaid
flowchart LR
    subgraph Intrari ["Intrări (Senzori & Butoane)"]
        direction TB
        BME[BME280\nTemp/Umid/Pres]
        MQ[MQ-135\nSenzor Gaze]
        UV[ML8511\nSenzor UV]
        BTN1((Buton\nUpload))
        BTN2((Buton\nMute))
    end

    ESP{ESP32\nMicrocontroller}

    subgraph Iesiri ["Ieșiri & Cloud"]
        direction TB
        BUZ((Buzzer))
        CLOUD[(Adafruit IO\nDashboard)]
    end

    %% Conexiuni
    BME -- I2C (SDA 21, SCL 22) --> ESP
    MQ -- Analog (IO 35) --> ESP
    UV -- Analog (IO 34) --> ESP
    BTN1 -- Digital Polling (IO 27) --> ESP
    BTN2 -- Digital Polling (IO 26) --> ESP

    ESP -- PWM (IO 25) --> BUZ
    ESP -- Wi-Fi / MQTT --> CLOUD
