# Morse Code Door Lock

**Author**: Tudor-Cristian Costache
**GitHub Project Link**: (https://github.com/UPB-PMRust-Students/proiect-iJackT.git

## Description

The project give the user the possibility to unlock the door of a house unsing the Morse code. The input comes from the user using two buttons.

## Motivation

I chose this project because it combines security with an alternative and creative method of entering a password – Morse code. It's an interesting approach that merges something we all use daily, a door lock, with an interactive way to unlock the door by using Morse code instead of a traditional key.

## Architecture 
![Uploading diagram.svg…]()

Componentele utilizare:
- 2x Raspberry Pico 2W – a doilea folosit pentru debug 
-	1 buzzer – ofera feedback auditiv pentru codul gresit introdus
-	2x led-uri – unul verde si unul rosu
-	1x servomotor – folosit pentru blocarea/deblocarea usii
-	2x butoane – unul pentru “punct” si celalalt pentru “linie”, aferent codului morse
-	Rezistente – folosite pentru  a proteja componentele si corecta functionare a circuitului


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
