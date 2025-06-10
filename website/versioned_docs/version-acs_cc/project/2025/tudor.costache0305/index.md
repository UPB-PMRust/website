# Morse Code Door Lock

**Author**: Tudor-Cristian Costache
**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/proiect-iJackT.git

## Description

The project give the user the possibility to unlock the door of a house unsing the Morse code. The input comes from the user using two buttons. It gives feeback to the user using:
+ Sound signals using a buzzer
+ Visual signals using GREEN and RED LEDs

## Motivation

I chose this project because it combines security with an alternative and creative method of entering a password – Morse code. It's an interesting approach that merges something we all use daily, a door lock, with an interactive way to unlock the door by using Morse code instead of a traditional key.

## Architecture 
The following diagram shows the system architecture:

![diagram](https://github.com/user-attachments/assets/44a96d33-9b22-457d-b9d3-9be17945bc4b)

The Raspberry Pi Pico 2W acts as the CNS of the hole system, managing everything.
The buttons are used by the user to input the code for unlocking the door.
The servomotor is used as the lock of the door.
The LEDs and the buzzer provide the user feedback to let them know whether the inputede code is correct or not.

## Components

+ 2x Raspberry Pico 2W – the second one used for debugging
+ 1 buzzer – provides auditory feedback for incorrect code input
+ 2x LEDs – one green and one red, provideing visual feedback
+ 1x servomotor – used for locking/unlocking the door
+ 2x buttons – one for "dot" and the other for "dash", corresponding to Morse code
+ Resistors – used to protect components and ensure proper circuit operation


Functionare:
Utilizatorul apasa pe butoane pentru a introduce codul. Microcontroler-ul verifica daca daca parola corespundei celei stocate. In cazul in care secventa introdusa corespunde cu cea prestabilita, servomotorul va debloca usa, iar un led verde se va aprinde. In caz contrar, se va aprinde led-ul rosu, iar buzzer-ul va suna pentru a atentiona faptul ca nu a fost introdus codul corect. 

Descrierea hardware:
Cele doua microcontrolere sunt conectate confor diagramei din laborator pentru debug. Componentele sunt conectate astfel:
Pinurile 4-7 sunt folosite pentru debug
Pinul 39 este folosit pentru pentru alimentare
Pinul 38 este conectat la GND
Servomotorul este conectat la 3v3, GND si la pinul 2 al raspberry-ului
Pe pinul 9 este conectat butonul aferent punctului
Pe pinul 10 este conectat butonul aferent punctului
Pinii 11 respectiv 12 sunt folositi pentru ledul verde respectiv rosu
Pinul 14 este folosit pentru a controla buzzer-ul
