# Tabla de Sah Automata by Babacea Alexandru

## Descriere Generala
Proiectul consta in realizarea unei table de sah complet automatizate, care permite atat jocul intre doi jucatori umani (validand mutarile), cat si jocul impotriva unui sistem de Inteligenta Artificiala. 

Elementul de noutate si complexitate este mecanismul ascuns care muta piesele singur. Acesta foloseste un sisitem de sine de tip CoreXY pozitionat sub suprafata de joc. Acesta permite ca un electromagnet sa se deplaseze sub tabla si sa mute piesele de sah (care au magneti in interior) catre noua pozitie. Detectia pozitiilor pieselor de pe tabla se realizeaza printr-o matrice de 64 de senzori magnetici.

## Functionalitati
- Mod Om vs. Om: Monitorizeaza mutarile si contorizeaza timpul.
- Mod Om vs. Computer: Utilizatorul face o mutare, iar sistemul calculeaza raspunsul prin algoritmul Minimax si muta fizic piesa oponentului.
- Autocalibrare: La pornire, masa XY se calibreaza folosind limitatoare de cursa (endstops) pentru a stabili originea sistemului de coordonate.
- Interfata: Sistem cu ecran LCD si butoane pentru selectia modurilor de joc si confirmarea finalizarii turului de catre jucatori.

## Schema Hardware
Sistemul este impartit in trei module principale:
1. Modulul de Procesare: Un microcontroller care ruleaza atat logica jocului (motorul de sah), cat si logica de miscare (generarea pasilor pentru motoare).
2. Modulul de Actionare: Doua motoare pas cu pas Nema 17, controlate prin drivere A4988, si un electromagnet controlat printr-un tranzistor de putere.
3. Modulul de Detectie: 64 de senzori magnetici cititi folosind multiplexoare externe, din cauza numarului limitat de pini ai microcontrollerului.

## Lista de Piese (BOM)
- 1x STM32 (microcontroller)
- 2x Motoare pas cu pas (Nema 17)
- 2x Drivere motoare A4988
- 1x Electromagnet (tensune: 12V, forta: 5kg)
- 1x Tranzistor de putere (TIP122) + dioda flyback
- 64x Senzori magnetici (Reed)
- 4x Multiplexoare CD74HC4067 (16 canale)
- 1x Display LCD
- 2x Limitatoare de cursa
- 2x Butoane
- Componente mecanice: profile de aluminiu V-slot (20x20), roti, curea GT2, fulii.
- 32x Magneti mici din neomidiu (pentru piesele de sah)

## Detalii de Implementare Software
Logica programului este scrisa in Rust pentru STM32. Proiectul implementeaza algoritmul Minimax optimizat cu Alpha-Beta pruning, utilizand putina memorie.

Miscarea corecta a pieselor: Programul calzuleaza coordonatele de plecare si destinatie. Apoi, transforma aceste coordonate logice in pasi de motor pentru masa CoreXY. De asemenea, algoritmii includ logica pentru evitarea coliziunilor intre piese.

## Jurnal de Activitate
- [x] Saptamana 1: Alegerea temei si documentarea componentelor necesare.
- [x] Saptamana 2: Redactarea paginii site-ului pentru GitHub.
- [x] Saptamana 3: Achizitia componentelor si testarea izolata a senzorilor.
- [ ] Saptamana 4: Asamblarea mecanica a axelor CoreXY.
- [ ] Saptamana 5: Realizarea circuitului electronic de control si testarea motoarelor.
- [ ] Saptamana 6: Integrarea software a motorului de sah cu functiile de miscare.

## Bibliografie si Resurse
- Tutorial de referinta Instructables: [Automated Chessboard by Greg06](https://www.instructables.com/Automated-Chessboard/?fbclid=PAVERTVgQkc31leHRuA2FlbQIxMABzcnRjBmFwcF9pZA81NjcwNjczNDMzNTI0MjcAAaf3d5kjWfOXYe6eTNwDwONbe9yrm8rgNCHcAq2UOxQGzEu7HfhKyTgcw8Q93w_aem_8qWs7lUFjMI_7Cdr_kOIEA)
- Link repository cod sursa: https://github.com/UPB-PMRust-Students/acs-project-2026-AlexandruBabacea-student
