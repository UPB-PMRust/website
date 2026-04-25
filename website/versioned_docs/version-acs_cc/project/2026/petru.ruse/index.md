# Derby Predictor
Simulator de curse asincron cu control de precizie pe 4 axe liniare.

:::info

**Author:** Petru Ruse \
**GitHub Project Link:** [https://github.com/UPB-PMRust-Students/acs-project-2026-PetruRuse](https://github.com/UPB-PMRust-Students/acs-project-2026-PetruRuse)

:::

## Description
Sistemul este un simulator fizic de curse pe 4 axe liniare. Utilizatorul selectează un concurent prin butoane și LCD. La start, sistemul procesează un seed aleatoriu citind zgomotul de pe un pin ADC pentru a genera profile de viteză variabile și dinamice. Procesarea folosește multitasking asincron pentru a controla simultan și independent mișcarea celor 4 axe fără blocaje software. Intrările de la limitatoarele de cursă declanșează întreruperi hardware (EXTI) pentru calibrarea automată a poziției. Ieșirile includ mișcarea coordonată a axelor, afișarea stării pe LCD și feedback sonor.

## Motivation
Am ales acest proiect pentru a explora provocările controlului fizic de precizie combinat cu multitasking-ul asincron (Embassy) în Rust. Dorința de a construi un sistem electromecanic complex, care evită blocajele software prin folosirea DMA-ului și a întreruperilor hardware (EXTI), reprezintă o oportunitate excelentă de a aprofunda conceptele de programare aplicate pe hardware real (STM32).

## Architecture
![Schema Bloc](architecture.svg)

**Main Components:**
* **Input:** Butoane tactile (Selecție/Start), Potențiometru liniar (Seed ADC), 4x Limitatoare de cursă mecanice (EXTI pentru calibrare/finish).
* **Processing:** Placa STM32 Nucleo. Rulează mediul asincron și generează frecvențele variabile pentru pașii motoarelor.
* **Output:** Shield CNC Arduino v3 cu 4x Drivere A4988, 4x Motoare pas cu pas NEMA 17, Ecran LCD 1602 (I2C) și Buzzer pasiv.
* **Power:** Sursă în comutație de 12V 5A (60W) dedicată părții de forță (CNC Shield).

## Log
* **Săptămâna 20 - 26 Aprilie:** Cercetare componente, definirea arhitecturii software/hardware și completarea formularului de proiect pe Moodle. Am finalizat și plasat comenzile pentru toate componentele fizice.

## Hardware & Bill of Materials (BOM)
Sistemul utilizează componente electromecanice solide pentru axele liniare, controlate de drivere dedicate pentru a asigura o mișcare fluidă. Logica este izolată electric de circuitul de forță prin folosirea CNC Shield-ului.

| Device | Usage | Price |
| :--- | :--- | :--- |
| **STM32 Nucleo Board** | Unitatea centrală de procesare | Gratuit (Facultate) |
| **Motor Pas cu Pas NEMA 17 (x4)** | Acționare mecanică a axelor | ~218.69 Lei |
| **Driver Motoare A4988 (x4)** | Control curent și micropășire | ~35.67 Lei |
| **Shield CNC Arduino v3** | Rutare pini STEP/DIR și alimentare forță | ~15.30 Lei |
| **Axă T8 400mm + Piuliță (x4)** | Pista fizică de rulare | ~133.87 Lei |
| **Limitator Cursă Mecanic (x4)** | Senzori de poziție (Start/Finish) | ~19.44 Lei |
| **Cuplaj Flexibil 5-8mm (x4)** | Conexiune ax motor - axă filetată | ~26.41 Lei |
| **Sursă 12V 5A (60W)** | Alimentare dedicată modulelor A4988 | ~33.48 Lei |
| **Display LCD 1602 (Modul I2C)** | Interfața grafică UI | 23.22 Lei |
| **Potențiometru 10k & Butoane (x5)** | Input ADC și comenzi UI | ~16.00 Lei |
| **Buzzer Pasiv 3.3V** | Feedback sonor asincron (PWM) | ~1.12 Lei |
| **Breadboard & Set Fire Dupont** | Prototipare și conexiuni logice | ~36.00 Lei |

## Software
| Library | Description | Usage |
| :--- | :--- | :--- |
| **embassy-stm32** | HAL asincron pentru STM32 | Configurare pini STEP/DIR, EXTI, I2C (LCD) și ADC (Zgomot). |
| **embassy-executor** | Runtime asincron | Rularea simultană a celor 4 task-uri complet independente pentru motoare. |
| **embassy-time** | Timer asincron | Generarea delay-urilor non-blocante pentru profilul de viteză. |
| **defmt** | Logging framework | Depanarea stărilor sistemului și a datelor de pe senzorul ADC. |

## Links
* [Embassy Rust Documentation](https://embassy.dev/)
* [A4988 Stepper Motor Driver Datasheet](https://www.pololu.com/file/0J450/a4988_DMOS_microstepping_driver_with_translator.pdf)