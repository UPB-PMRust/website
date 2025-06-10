# Morse Code Door Lock

Proiectul consta intr-un sistem de deblocare a unei usi controlat prin codul Morse. Utilizatorul introduce, folosind doua butoane, unul pentru “punct” si unul pentru “linie” aferent codului morse, un cod pentru a debloca usa, care este blocata de catre un servo motor.

Motivatia Proiectului
Am ales acest proiect deoarece combina securitatea cu o metoda alternativa si creativa de introducere a unui parole – codul morse. Este o metoda interesanta care combine un element pe care il folosim cu totii zilnic, incuietoarea usii, cu un mod interactiv de a debloca usa, folsoind codul morse in locul unei chei obisnuite. 

Arhitectura:
Componentele utilizare:
-   2x Raspberry Pico 2W – a doilea folosit pentru debug 
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
