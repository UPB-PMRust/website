Project Name
STM32 Rust Smart Car
:::
Author: Florea Delia Cristina
GitHub Project Link: https://github.com/UPB-PMRust-Students/acs-project-2026-Deliutz
:::
________________________________________
Description
Proiectul constă într-o mașinuță controlată wireless, programată în Rust pe o placă STM32 Nucleo-U545RE-Q. Mașinuța poate fi controlată de pe laptop prin Bluetooth și este capabilă să detecteze obstacole și să înregistreze imagini/video pe un card SD folosind o cameră conectată direct la microcontroller. Datele capturate de cameră sunt procesate de STM32 și salvate pe cardul SD.
________________________________________
Motivation
Am ales acest proiect pentru a învăța programare embedded low-level în Rust și pentru a înțelege cum funcționează interacțiunea directă cu hardware-ul (GPIO, UART, senzori, memorie externă). De asemenea, am vrut să construiesc un sistem complet: control, sensing și captură de date.
________________________________________
Architecture
Componente principale:
•	STM32 (controller principal)
→ rulează logica mașinuței (control, senzori, comunicație și procesare date) 
•	Driver motoare (L298N)
→ controlează cele 4 motoare DC 
•	Bluetooth (HC-05)
→ comunicare între mașinuță și aplicația Rust de pe laptop 
•	Aplicație desktop (Rust)
→ UI pentru control (forward, back, left, right, stop) 
•	Senzor distanță (HC-SR04)
→ detectează obstacole 
•	Cameră (OV7670)
→ capturează imagini 
•	Modul SD Card
→ stochează imaginile capturate de cameră 
________________________________________
Conexiuni
Laptop (Rust App)
        ↓ Bluetooth
      HC-05
        ↓ UART
      STM32
     /   |    \     \
 L298N  Sensor  Camera  SD Card
   ↓       ↓        ↓       ↓
Motoare  Distanță  Imagine  Stocare
________________________________________
Log
Week 5 - 11 May
•	Setup STM32 + Rust (Embassy) 
•	Control motoare prin L298N 
•	Test mișcare basic (forward/back) 
Week 12 - 18 May
•	Implementare Bluetooth (HC-05) 
•	Creare aplicație Rust pentru control 
•	Control în timp real cu touch 
Week 19 - 25 May
•	Adăugare senzor obstacole 
•	Integrare citire distanță 
•	Plan integrare cameră + SD 
________________________________________
Hardware
Componente folosite:
•	STM32 Nucleo-U545RE-Q 
•	L298N Motor Driver 
•	4x Motoare DC 
•	HC-05 Bluetooth 
•	HC-SR04 Ultrasonic Sensor 
•	Cameră OV7670 
•	Modul card SD 
•	Baterii (separate pentru STM și motoare) 
________________________________________
Bill of Materials
Device	Usage	Price
STM32 Nucleo-U545RE-Q	Microcontroller principal	~150 RON
L298N	Control motoare	~20 RON
HC-05	Bluetooth	~25 RON
HC-SR04	Senzor obstacole	~10 RON
Modul SD Card	Stocare date	~15 RON
Cameră OV7670	Captură imagine	~30 RON
Motoare DC x4	Mișcare	~40 RON
Baterii	Alimentare	~50 RON
________________________________________
Software
Library	Description	Usage
embassy-stm32	HAL async pentru STM32	Control hardware
embassy-time	Timere async	PWM software
embedded-hal	Abstracții hardware	GPIO
serialport	Comunicare PC	Bluetooth
egui / eframe	UI desktop	Control mașinuță
________________________________________
Links
1.	https://embassy.dev 
2.	https://www.st.com/resource/en/reference_manual/rm0456-stm32u5-series-advanced-armbased-32bit-mcus-stmicroelectronics.pdf 
3.	https://docs.rs/embedded-hal 
4.	https://www.st.com 


