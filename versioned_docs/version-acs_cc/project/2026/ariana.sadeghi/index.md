---
id: ariana.sadeghi
title: Sistem Automat de Sortare Optica
---

# Sistem Automat de Sortare Optica

## 1. Descriere Generala
Sistemul automat de sortare simuleaza o linie industriala de control al calitatii. Acesta deplaseaza obiecte fizice pe un traseu mecanic si citeste proprietatile lor optice (opacitate/culoare) folosind un senzor de lumina. 

Software-ul proceseaza aceste date analogice, comparandu-le cu un prag presetat (threshold) pentru a clasifica fiecare obiect ca fiind "acceptat" sau "respins". Ca iesiri, sistemul comanda un motor pas cu pas pentru avansul continuu al pieselor, declanseaza un brat mecanic (servomotor) care ejecteaza fizic piesele defecte directionandu-le in containere separate si actualizeaza o interfata vizuala (LCD) cu statistici in timp real privind numarul total de piese procesate, acceptate si respinse.

## 2. Design Hardware

Designul hardware presupune o constructie pe doua niveluri ("etaje"). La nivelul inferior se afla placa de dezvoltare protejata, iar la nivelul superior se afla mecanismul de sortare (discul rotativ) prevazut cu un orificiu care se aliniaza perfect cu fotorezistorul de pe shield.

### Lista de componente:
* Placa de dezvoltare **STM32 Nucleo-U545RE-Q** (unitatea de procesare principala)
* **Shield PMIMA V2.0** din care se utilizeaza:
  * Fotorezistorul (senzor ADC pentru citirea opacitatii pieselor)
  * Display-ul LCD SPI (pentru afisarea interfetei cu utilizatorul)
  * Butoanele si potentiometrul (pentru calibrarea sensibilitatii la lumina)
* **Motor pas cu pas 28BYJ-48** cu driver ULN2003 (actioneaza banda/discul de avans al pieselor)
* **Micro Servomotor SG90** (actioneaza macazul/bratul de sortare)

### Schema Bloc
*(Aici vei adauga imaginea cu schema ta bloc. Asigura-te ca fisierul este in acelasi folder cu index.md)*

![Schema Bloc Hardware](./schema_bloc.svg)

## 3. Design Software

Sistemul este implementat in limbajul **Rust**, utilizand un mediu de executie asincron (ex. framework-ul Embassy) sau intreruperi hardware (bare-metal) pentru a permite controlul simultan al celor doua motoare si citirea senzorului fara a bloca executia principala a procesorului.

Logica centrala este modelata sub forma unei **Masini de Stari (State Machine)**, avand urmatoarele stari principale:
1. **AVANS (IDLE/MOVING):** Motorul pas cu pas avanseaza piesele prin generarea de secvente precise pe pinii GPIO. In paralel, se citeste continuu valoarea convertorului analog-digital (ADC) de la fotorezistor.
2. **DETECTIE (DETECTED):** Cand nivelul de lumina scade brusc, inseamna ca o piesa se afla deasupra senzorului. Avansul este oprit temporar, iar valoarea luminii este comparata cu pragul de calibrare.
3. **EJECTARE (SORTING):** Se genereaza un semnal PWM specific catre servomotor: 
   * Unghi de 0° pentru piesele catalogate ca "Defecte".
   * Unghi de 180° pentru piesele catalogate ca "Bune".
4. **ACTUALIZARE (UPDATING):** Se actualizeaza contoarele interne si se trimit noile statistici catre LCD prin magistrala SPI. Bratul servomotorului revine la pozitia neutra (90°).

## 4. Rezultate

In prezent, proiectul se afla in faza de asamblare a componentelor mecanice si prototipare a codului de control pentru motoare.

## 5. Concluzii


## 6. Bibliografie / Resurse
* [Rust Embedded Book](https://docs.rust-embedded.org/book/)
* Laboratoarele cursului de Proiectarea cu Microprocesoare (UPB)
* Datasheet STM32U545RE si documentatia senzorilor folositi.