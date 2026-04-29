# RustMed Locker

Dulap inteligent pentru medicamente cu 4 compartimente independente, realizat folosind Rust si placa STM32 NUCLEO-U545RE-Q.

## 1. Descrierea functionalitatii

RustMed Locker este un proiect embedded care are ca scop realizarea unui dulap inteligent pentru medicamente. Sistemul este format din 4 compartimente independente, dispuse intr-o structura de tip 2 x 2. Fiecare compartiment are propriul mecanism de inchidere, actionat cu ajutorul unui servomotor, si propriul senzor pentru detectarea starii usii.

Sistemul este controlat de o placa STM32 NUCLEO-U545RE-Q, pe care ruleaza o aplicatie scrisa in Rust. Placa gestioneaza cele 4 servomotoare, cei 4 senzori de usa, buzzerul si comunicarea prin interfata seriala USB.

Ideea principala este ca fiecare compartiment sa poata fi controlat separat. Sistemul poate debloca doar compartimentul selectat, poate detecta daca usa a fost deschisa sau inchisa si poate semnaliza evenimente importante cu ajutorul unui buzzer. Comunicarea cu utilizatorul se face prin USB serial, unde se pot trimite comenzi si se poate urmari starea sistemului.

Proiectul este gandit ca un prototip educational de sistem embedded si nu ca un dispozitiv medical certificat. Scopul lui este sa demonstreze integrarea mai multor componente hardware si software intr-un sistem functional.

Functionalitatile principale ale proiectului sunt:

* control independent pentru 4 compartimente;
* blocarea si deblocarea fiecarui compartiment cu ajutorul unui servomotor;
* detectarea starii fiecarei usi cu ajutorul unui microswitch;
* semnalizare sonora prin buzzer;
* comunicare prin USB serial pentru comenzi, status si debugging;
* organizarea logicii software sub forma de stari pentru fiecare compartiment;
* alimentarea separata a servomotoarelor pentru stabilitate;
* posibilitatea de extindere ulterioara cu programare pe ore sau cu un modul RTC.

Fluxul general de functionare este urmatorul:

1. Sistemul porneste si initializeaza toate componentele.
2. Toate compartimentele sunt puse initial in starea blocata.
3. Utilizatorul trimite o comanda prin interfata seriala.
4. Sistemul verifica daca acea comanda este valida.
5. Daca se cere deblocarea unui compartiment, placa trimite semnal catre servomotorul corespunzator.
6. Compartimentul selectat este deblocat pentru o perioada limitata.
7. Microswitch-ul detecteaza daca usa a fost deschisa sau inchisa.
8. Dupa inchiderea usii sau dupa expirarea unui timeout, compartimentul este blocat din nou.
9. Sistemul transmite prin seriala starea actualizata.

Exemple de comenzi pentru interfata seriala:

```text
status
unlock 1
unlock 2
unlock 3
unlock 4
lock 1
lock 2
lock 3
lock 4
```

Exemple de mesaje transmise de sistem:

```text
SYSTEM READY
COMPARTMENT 1 -> LOCKED
COMPARTMENT 1 -> UNLOCKED
COMPARTMENT 1 -> DOOR OPEN
COMPARTMENT 1 -> DOOR CLOSED
ERROR: INVALID COMMAND
```

## 2. Motivatie

Am ales acest proiect deoarece mi s-a parut o idee practica si potrivita pentru un proiect embedded. Un dulap inteligent pentru medicamente combina mai multe parti importante: controlul unor componente fizice, citirea unor senzori, comunicarea cu utilizatorul si organizarea software a mai multor stari.

Proiectul nu este doar un exemplu simplu cu un LED sau cu un singur motor. Sistemul trebuie sa controleze 4 compartimente separate, iar fiecare compartiment are propriul servomotor si propriul senzor de usa. Din acest motiv, partea software trebuie organizata clar, astfel incat fiecare compartiment sa aiba propria stare si propriul comportament.

Mi s-a parut o idee buna si pentru ca rezultatul poate fi demonstrat usor. In timpul demo-ului se poate vedea clar cum un compartiment se deblocheaza, cum senzorul detecteaza usa si cum sistemul raporteaza evenimentele prin interfata seriala.

Prin acest proiect vreau sa exersez lucrul cu periferice, generarea de semnale PWM pentru servomotoare, citirea intrarilor digitale, folosirea comunicarii seriale si structurarea unei aplicatii embedded scrise in Rust.

## 3. Arhitectura

### 3.1 Prezentare generala

Sistemul este construit in jurul placii STM32 NUCLEO-U545RE-Q. Aceasta placa actioneaza ca unitate centrala de control si ruleaza aplicatia scrisa in Rust.

Placa este conectata la:

* 4 servomotoare, cate unul pentru fiecare compartiment;
* 4 microswitch-uri, cate unul pentru fiecare usa;
* 1 buzzer pentru semnalizare sonora;
* o sursa externa pentru alimentarea servomotoarelor;
* un PC/laptop prin USB, pentru comunicare seriala.

### 3.2 Componente si interconectare

#### STM32 NUCLEO-U545RE-Q

Placa STM32 NUCLEO-U545RE-Q controleaza intregul sistem. Aceasta genereaza semnalele PWM pentru servomotoare, citeste starea senzorilor de usa, controleaza buzzerul si comunica prin USB serial cu calculatorul.

#### Servomotoare

Fiecare servomotor este folosit pentru a actiona mecanismul de inchidere al unui compartiment. In functie de pozitia servomotorului, compartimentul este blocat sau deblocat. Pentru fiecare compartiment exista cate un semnal de control separat.

#### Microswitch-uri

Fiecare microswitch este folosit pentru a detecta daca usa unui compartiment este inchisa. Daca usa se deschide, starea citita de placa se modifica, iar software-ul poate actualiza starea compartimentului.

#### Buzzer

Buzzerul este folosit pentru feedback sonor. Acesta poate semnaliza pornirea sistemului, deblocarea unui compartiment, o comanda gresita sau o usa ramasa deschisa prea mult timp.

#### Alimentare externa pentru servomotoare

Servomotoarele pot consuma mai mult curent decat poate oferi in siguranta placa de dezvoltare. Din acest motiv, ele sunt alimentate separat, folosind un suport de baterii. Masa sursei externe trebuie conectata la masa placii, pentru ca semnalele de control sa aiba aceeasi referinta.

### 3.3 Diagrama arhitecturii

```text
                    +----------------------+
                    |      PC / Laptop     |
                    |   terminal serial    |
                    +----------+-----------+
                               |
                               | USB serial
                               |
                    +----------v-----------+
                    | STM32 NUCLEO-U545RE-Q|
                    |   aplicatie Rust     |
                    +----+----------+------+
                         |          |
                         |          |
             +-----------+          +------------------+
             |                                      |
             v                                      v
       +-----------+                         +---------------+
       |  Buzzer   |                         |  4 senzori    |
       |  alerta   |                         |  de usa       |
       +-----------+                         +---------------+
                                                    |
                                                    v
                                          +--------------------+
                                          | 4 compartimente    |
                                          | 2 x 2              |
                                          +--------------------+
                                                    ^
                                                    |
                                          +--------------------+
                                          | 4 servomotoare     |
                                          | mecanisme lock     |
                                          +--------------------+
                                                    ^
                                                    |
                                          +--------------------+
                                          | alimentare externa |
                                          | pentru servouri    |
                                          +--------------------+
```

In varianta finala a proiectului, aceasta diagrama poate fi redesenata in diagrams.net si exportata ca imagine, pastrand aceeasi structura logica.

## 4. Weekly log

### Saptamana 1

* Am analizat cerintele proiectului.
* Am inceput sa caut o idee potrivita pentru un proiect embedded.
* Am ales directia generala: un sistem inteligent pentru depozitarea medicamentelor.

### Saptamana 2

* Am definit ideea de smart medicine locker.
* Am stabilit ca proiectul va avea mai multe compartimente, nu doar unul singur.
* Am analizat ce componente ar fi necesare pentru un astfel de sistem.

### Saptamana 3

* Am comparat mai multe variante de implementare.
* Am renuntat la componente mai scumpe, cum ar fi display, tastatura sau RTC, pentru a pastra proiectul in buget.
* Am decis ca functionalitatea principala va fi controlul a 4 compartimente independente.

### Saptamana 4

* Am ales placa STM32 NUCLEO-U545RE-Q.
* Am stabilit lista finala de componente.
* Am pregatit descrierea initiala a proiectului pentru confirmarea temei.

### Saptamana 5

* Am comandat componentele necesare.
* Am verificat lista de piese si bugetul total.
* Am inceput sa pregatesc structura documentatiei.

### Saptamana 6

* Am primit componentele.
* Am verificat fizic placa, servomotoarele, microswitch-urile si buzzerul.
* Am inceput planificarea conexiunilor hardware.

### Saptamana 7

* Voi planifica schema generala de conectare.
* Voi alege pinii necesari pentru servomotoare, senzori si buzzer.
* Voi stabili structura de baza a aplicatiei software.

### Saptamana 8

* Voi testa individual un servomotor.
* Voi verifica modul de citire pentru un microswitch.
* Voi testa buzzerul ca iesire digitala.

### Saptamana 9

* Voi actualiza documentatia proiectului.
* Voi completa sectiunile de functionalitate, motivatie si arhitectura.
* Voi pregati materialele pentru milestone-ul de documentatie.

### Saptamana 10

* Voi integra primul compartiment complet: servomotor plus microswitch.
* Voi verifica logica de blocare si deblocare pentru un compartiment.
* Voi incepe testarea comunicarii seriale.

### Saptamana 11

* Voi extinde conexiunile pentru mai multe compartimente.
* Voi verifica stabilitatea alimentarii pentru servomotoare.
* Voi pregati partea de hardware pentru milestone.

### Saptamana 12

* Voi imbunatati structura codului Rust.
* Voi separa logica pentru compartimente, senzori si servomotoare.
* Voi adauga mesaje de status prin interfata seriala.

### Saptamana 13

* Voi finaliza logica software pentru toate cele 4 compartimente.
* Voi testa scenarii de utilizare: unlock, lock, door open, door closed si comenzi invalide.
* Voi pregati partea de software pentru milestone.

### Saptamana 14

* Voi realiza poze cu prototipul final.
* Voi completa documentatia finala cu poze, schema si eventuale modificari.
* Voi pregati demo-ul live pentru prezentarea finala.

## 5. Hardware design

### 5.1 Descriere hardware

Hardware-ul proiectului este format dintr-o placa STM32 NUCLEO-U545RE-Q, 4 servomotoare, 4 microswitch-uri, un buzzer, breadboard, fire jumper si o sursa externa pentru servomotoare.

Fiecare compartiment are doua componente principale:

* un servomotor, care actioneaza mecanismul de blocare;
* un microswitch, care detecteaza starea usii.

Cele 4 compartimente sunt controlate de aceeasi placa, dar fiecare are propriile semnale de control si citire. Aceasta separare face posibila controlarea independenta a fiecarui compartiment.

### 5.2 Componente hardware folosite

| Componenta                                        | Cantitate | Rol                                    |
| ------------------------------------------------- | --------: | -------------------------------------- |
| STM32 NUCLEO-U545RE-Q                             |         1 | Placa principala de control            |
| DFRobot SER0053 servo motor                       |         4 | Actionarea mecanismelor de inchidere   |
| Omron SS-5GL-3 microswitch                        |         4 | Detectarea starii usilor               |
| PUI Audio AI-1223-TWT-3V-2-R buzzer               |         1 | Feedback sonor                         |
| BusBoard BB170-R breadboard                       |         1 | Prototipare                            |
| BusBoard ZW-MM-10 jumper wires                    |     1 set | Conexiuni intre componente             |
| Eagle Plastic Devices 12BH331/C-GR battery holder |         1 | Alimentare externa pentru servomotoare |

### 5.3 Conexiuni hardware propuse

Conexiunile exacte pot fi ajustate in timpul testarii, dar sistemul are nevoie de:

* 4 iesiri PWM pentru servomotoare;
* 4 intrari digitale pentru microswitch-uri;
* 1 iesire digitala pentru buzzer;
* conexiune USB pentru seriala;
* masa comuna intre placa si alimentarea externa a servomotoarelor.

Un exemplu de alocare a pinilor este:

| Functie              | Pin propus   | Observatii           |
| -------------------- | ------------ | -------------------- |
| Servo compartiment 1 | PWM 1        | Semnal control servo |
| Servo compartiment 2 | PWM 2        | Semnal control servo |
| Servo compartiment 3 | PWM 3        | Semnal control servo |
| Servo compartiment 4 | PWM 4        | Semnal control servo |
| Senzor usa 1         | GPIO input 1 | Intrare digitala     |
| Senzor usa 2         | GPIO input 2 | Intrare digitala     |
| Senzor usa 3         | GPIO input 3 | Intrare digitala     |
| Senzor usa 4         | GPIO input 4 | Intrare digitala     |
| Buzzer               | GPIO output  | Iesire digitala      |
| USB serial           | USB          | Comenzi si loguri    |

Pinii exacti vor fi completati dupa validarea conectarii pe placa, pentru a evita conflicte intre perifericele folosite de timer, GPIO si seriala.

### 5.4 Alimentare

Placa STM32 NUCLEO-U545RE-Q este alimentata prin USB. Servomotoarele sunt alimentate separat, prin suportul de baterii. Aceasta separare este importanta deoarece servomotoarele pot avea varfuri de curent care pot afecta stabilitatea placii.

Pentru functionare corecta, masa alimentarii externe trebuie conectata la masa placii. Astfel, semnalele PWM transmise de placa catre servomotoare au aceeasi referinta electrica.

Schema de principiu pentru alimentare este:

```text
USB PC ---------------> STM32 NUCLEO-U545RE-Q

Suport baterii 3 x AA -> V+ servomotoare
GND suport baterii ---> GND servomotoare
GND suport baterii ---> GND placa STM32
GPIO/PWM placa -------> semnal control servo
```

### 5.5 Schema hardware

Schema logica a conexiunilor este urmatoarea:

```text
STM32 NUCLEO-U545RE-Q

PWM_1  -----------------> Servo 1 signal
PWM_2  -----------------> Servo 2 signal
PWM_3  -----------------> Servo 3 signal
PWM_4  -----------------> Servo 4 signal

GPIO_IN_1 <------------- Microswitch usa 1
GPIO_IN_2 <------------- Microswitch usa 2
GPIO_IN_3 <------------- Microswitch usa 3
GPIO_IN_4 <------------- Microswitch usa 4

GPIO_OUT --------------> Buzzer
GND -------------------- GND comun
USB -------------------- PC / terminal serial
```

Schema electrica detaliata va fi redesenata in KiCad dupa validarea pinilor si a conexiunilor pe breadboard. Aceasta va pastra structura de mai sus si va include alimentarea separata a servomotoarelor si conexiunea de masa comuna.

### 5.6 Design mecanic

Mecanic, proiectul va fi construit ca un dulap mic cu 4 compartimente in format 2 x 2. Fiecare compartiment va avea o usa si un mecanism simplu de inchidere actionat de un servomotor.

Partea mecanica este importanta deoarece servomotorul trebuie sa fie pozitionat astfel incat sa blocheze si sa deblocheze usa in mod repetabil. De asemenea, microswitch-ul trebuie montat intr-o pozitie in care sa detecteze corect cand usa este inchisa.

Posibile variante pentru constructia fizica:

* cutie din carton rigid sau placaj subtire;
* structura printata 3D;
* compartimente realizate manual si fixate pe un suport comun.

### 5.7 Poze ale dispozitivului

Pozele reale ale dispozitivului vor fi adaugate dupa asamblarea prototipului fizic. Pentru milestone-ul hardware vor fi incluse:

* poza de ansamblu cu locker-ul;
* poza cu placa si breadboard-ul;
* poza cu un servomotor montat pe un compartiment;
* poza cu microswitch-ul pentru detectia usii;
* poza cu alimentarea separata pentru servomotoare.

## 6. Software design

### 6.1 Obiectiv software

Software-ul are rolul de a coordona cele 4 compartimente independente. Aplicatia trebuie sa citeasca senzorii de usa, sa controleze servomotoarele, sa activeze buzzerul si sa comunice prin interfata seriala.

Aplicatia va fi scrisa in Rust si va rula pe placa STM32 NUCLEO-U545RE-Q. Pentru implementare se poate folosi Embassy-rs sau o abordare bare-metal cu HAL-ul corespunzator placii.

### 6.2 Structura software propusa

Aplicatia poate fi organizata in mai multe module logice:

* `main` - initializarea hardware si pornirea aplicatiei;
* `serial_interface` - primirea comenzilor si trimiterea mesajelor de status;
* `locker_controller` - coordonarea celor 4 compartimente;
* `compartment` - logica unui singur compartiment;
* `servo_driver` - controlul servomotoarelor prin PWM;
* `sensor_driver` - citirea microswitch-urilor;
* `buzzer_driver` - controlul buzzerului;
* `events` - definirea si raportarea evenimentelor din sistem.

Aceasta structura ajuta la separarea responsabilitatilor. De exemplu, logica unui compartiment nu trebuie sa fie amestecata cu codul pentru seriala sau cu functiile de nivel jos pentru PWM.

### 6.3 Masina de stari

Fiecare compartiment poate fi modelat cu o masina de stari simpla. Aceasta abordare ajuta la organizarea codului si la evitarea situatiilor neclare.

Starile posibile pentru un compartiment sunt:

* `Locked` - compartimentul este blocat;
* `Unlocking` - servomotorul se misca spre pozitia de deblocare;
* `Unlocked` - compartimentul este deblocat;
* `Open` - usa compartimentului este deschisa;
* `Relocking` - servomotorul se misca spre pozitia de blocare;
* `Error` - sistemul a detectat o situatie neasteptata.

Tranzitiile principale sunt:

```text
Locked -> Unlocking -> Unlocked -> Open -> Relocking -> Locked
```

In cazul unei erori, sistemul poate trece in starea `Error`, transmite mesaj prin seriala si semnalizeaza cu buzzerul.

### 6.4 Interfata seriala

Interfata seriala este folosita pentru comenzi si debugging. Utilizatorul poate trimite comenzi simple pentru a verifica starea sistemului sau pentru a controla un compartiment.

Exemple de comenzi:

```text
status
unlock 1
lock 1
unlock 2
lock 2
unlock 3
lock 3
unlock 4
lock 4
```

Exemple de raspunsuri:

```text
SYSTEM READY
COMPARTMENT 1 STATE: LOCKED
COMPARTMENT 2 STATE: OPEN
COMPARTMENT 3 STATE: LOCKED
COMPARTMENT 4 STATE: UNLOCKED
ERROR: INVALID COMPARTMENT
```

### 6.5 Diagrama functionala

```text
Pornire sistem
      |
      v
Initializare periferice
      |
      v
Blocare toate compartimentele
      |
      v
Asteptare comanda seriala sau eveniment senzor
      |
      +-------------------------------+
      |                               |
      v                               v
Primire comanda                  Detectie usa
      |                               |
      v                               v
Validare comanda                 Actualizare stare
      |                               |
      v                               v
Control servo / buzzer           Trimitere status
      |                               |
      +---------------+---------------+
                      |
                      v
              Revenire in bucla principala
```

### 6.6 Tratarea erorilor

Aplicatia trebuie sa trateze cateva situatii de eroare:

* comenzi invalide primite prin seriala;
* numar de compartiment invalid;
* usa ramasa deschisa prea mult timp;
* stare neasteptata a unui senzor;
* incercarea de a bloca un compartiment cu usa deschisa.

In aceste cazuri, sistemul va trimite un mesaj de eroare prin seriala si poate activa buzzerul pentru avertizare.

## 7. Bill of materials

### 7.1 Hardware bill of materials

Componentele principale din tabelul de mai jos sunt exact componentele comandate si folosite pentru partea electronica a proiectului.

| Componenta                                        | Cantitate | Rol in proiect                                                  |
| ------------------------------------------------- | --------: | --------------------------------------------------------------- |
| STM32 NUCLEO-U545RE-Q                             |         1 | Microcontroller principal si placa de dezvoltare                |
| DFRobot SER0053 servo motor                       |         4 | Controlul mecanismelor de inchidere pentru cele 4 compartimente |
| Omron SS-5GL-3 microswitch                        |         4 | Detectarea starii usilor pentru cele 4 compartimente            |
| PUI Audio AI-1223-TWT-3V-2-R buzzer               |         1 | Semnalizare sonora                                              |
| BusBoard BB170-R breadboard                       |         1 | Prototipare circuit                                             |
| BusBoard ZW-MM-10 jumper wires                    |     1 set | Conexiuni intre placa, breadboard si componente                 |
| Eagle Plastic Devices 12BH331/C-GR battery holder |         1 | Suport pentru alimentarea externa a servomotoarelor             |

Pe langa aceste componente comandate, mai sunt necesare cateva elemente auxiliare pentru montajul fizic:

| Componenta auxiliara                    |   Cantitate | Rol in proiect                                        |
| --------------------------------------- | ----------: | ----------------------------------------------------- |
| Baterii AA                              |           3 | Alimentarea servomotoarelor prin suportul de baterii  |
| Materiale mecanice pentru cutie         |       1 set | Constructia celor 4 compartimente                     |
| Banda dublu adeziva / suruburi / adeziv | dupa nevoie | Fixarea servomotoarelor, usilor si microswitch-urilor |

### 7.2 Software bill of materials

| Software / Tool               | Rol                                          |
| ----------------------------- | -------------------------------------------- |
| Rust                          | Limbajul principal de programare             |
| Cargo                         | Build system si package manager              |
| probe-rs / cargo-embed        | Flashing si debugging pe placa               |
| Embassy-rs sau bare-metal HAL | Framework pentru programare embedded in Rust |
| Terminal serial               | Trimitere comenzi si monitorizare status     |
| Git / GitHub                  | Versionarea codului sursa                    |
| Docusaurus / website fork     | Publicarea documentatiei proiectului         |
| diagrams.net                  | Realizarea diagramei de arhitectura          |
| KiCad                         | Realizarea schemei hardware                  |

## 8. Stadiul curent al proiectului

In acest moment, ideea proiectului este definita, componentele au fost alese si comandate, iar componentele electronice principale au fost primite. Urmatorii pasi sunt testarea componentelor individuale si apoi integrarea lor intr-un sistem complet.

Pasii urmatori sunt:

1. testarea unui servomotor cu placa STM32;
2. testarea unui microswitch ca intrare digitala;
3. testarea buzzerului;
4. integrarea unui singur compartiment complet;
5. extinderea sistemului la toate cele 4 compartimente;
6. realizarea schemei hardware finale;
7. construirea structurii fizice 2 x 2;
8. completarea documentatiei cu poze si diagrame reale.

## 9. Concluzie

RustMed Locker este un proiect embedded care combina controlul mai multor actuatoare, citirea senzorilor, comunicarea seriala si organizarea software pe stari. Complexitatea proiectului vine din faptul ca sistemul controleaza 4 compartimente independente, fiecare cu propriul servomotor si propriul senzor de usa.

Proiectul este potrivit pentru a demonstra cunostintele acumulate la curs, deoarece include atat partea de hardware, cat si partea de software scrisa in Rust. La final, obiectivul este obtinerea unui prototip functional care poate fi prezentat live si care arata clar interactiunea dintre software si componentele fizice.
