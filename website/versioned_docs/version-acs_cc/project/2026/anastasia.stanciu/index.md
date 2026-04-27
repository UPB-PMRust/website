
---

---

# Robotic Hand

A robotic hand that imitates the movement of a person's hand wearing a glove that has flex sensors on it.

> [!NOTE]
> **Author:** STANCIU Anastasia-Steliana
> **GitHub Project Link:**

## Description
The project is a robotic, 3D printed hand, that is actiond through some basic wires and servo-motors, that imitate the signals from a control glove. The glove has flex sensors, one on each finger, sending continuos signals to the board and servo-motors. The motion is intended to be as smooth and real-time as possible, providing a continuous flex of the fingers, rather than discrete.

## Motivation
This idea fits perfectly in the medical field. The main two innovations branching from my project are:

* It can be upgraded into a prosthetic hand that uses signals from an **EMG Muscle Sensor**, rather than from flex sensors.
* It can be used in **Kinetotherapy** to track the progress of patients and collect data and metrics, leading to both more customizable treatments and research data.
* It is a fun project, offering a great opportunity to discover hardware and human abilities in parallel, finding similarities and differences while learning how to model complex human movement.

This project is about using hardware and technology to consolidate inclusion and break the boundaries of disabilities, creating a world of possibilities.

## Arhitecture

In this section we will discuss components and the interconnections between them.

1. STM Board
Acționează ca unitatea de control de înaltă performanță. Procesează intrările analogice prin ADC-ul său de 12 biți (oferind o precizie mult mai mare decât Arduino) și generează semnale PWM pentru controlul celor 5 servomotoare. Funcționează la logică de 3.3V.
2. Servo-Motoare
    Convertesc semnalele PWM în mișcare unghiulară. Fiecare servo controlează un deget al mâinii robotice. Deși primesc semnal de control de la STM32, necesită o sursă de alimentare separată de 5V pentru a susține consumul de curent.

4. Flex Sensors
   Senzori rezistivi plasați pe mănușă. Aceștia fac parte dintr-un circuit de divizor de tensiune. Tensiunea variabilă rezultată este trimisă către pinii analogici ai STM32 pentru a detecta gradul de închidere a fiecărui deget.

6. Sursa de alimentare (baterii reincarcabile)

   ![Schematic](URL-ul_imaginii)



