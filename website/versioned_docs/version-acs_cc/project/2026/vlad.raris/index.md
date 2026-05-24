# Lansator Automat de Basket (Miniatură)

Un sistem mecatronic destinat aruncării mingilor de ping-pong către un coș de basket în miniatură.

:::info

**Author**: Raris Vlad-Cristian \
**GitHub Project Link**: https://github.com/UPB-PMRust-Students/acs-project-2026-duduvlad

:::

## Descriere

Acest proiect implementează un lansator automat pentru basket în miniatură. Sistemul folosește o catapultă pentru mingi de ping-pong, montată pe o bază rotativă. Lansatorul detectează panoul coșului cu ajutorul unui senzor ultrasonic HC-SR04, se orientează pe axa orizontală și apoi declanșează mecanismul de aruncare.

Controlul este realizat cu o placă NUCLEO-U545RE-Q, iar codul proiectului este scris în Rust. Coșul este construit separat și are un panou gri în spate, folosit ca suprafață de detecție pentru senzor. Pentru a simplifica problema de poziționare, lansatorul este deplasat pe un cerc în jurul coșului, iar sistemul trebuie să determine unghiul potrivit de orientare către panou.

## Motivație

Alegerea acestui proiect a fost determinată de dorința de a combina controlul mecanic, măsurarea distanței și programarea embedded în Rust într-un sistem vizibil și ușor de testat.

* **Control embedded:** Proiectul folosește semnale PWM pentru servomotoare și semnale GPIO pentru senzorul de distanță.
* **Rust în sisteme embedded:** Implementarea urmărește folosirea Rust pentru controlul sigur și predictibil al perifericelor.
* **Integrare hardware-software:** Sistemul combină partea mecanică a catapultei cu detecția panoului și controlul mișcării.
* **Prototipare practică:** Coșul, panoul și mecanismul de lansare permit testarea rapidă a comportamentului real al sistemului.

## Arhitectură

### Schema Bloc

```mermaid
flowchart LR
    subgraph Cos ["Coș"]
        Panou[Panou gri]
        Inel[Inel metalic]
    end

    subgraph Lansator ["Unitate Lansator"]
        Senzor[HC-SR04] -->|TRIG / ECHO| MCU[NUCLEO-U545RE-Q]
        Buton[Buton] -->|GPIO| MCU
        MCU -->|GPIO| LED[LED RGB]
        MCU -->|PWM| S1[Servo MG995 Bază]
        MCU -->|PWM| S2[Servo SG90 Trăgaci]
        Alimentare[Sursă laborator / XL4005] --> S1
        Alimentare --> S2
        Catapulta[Catapultă printată 3D] --> Minge[Minge ping-pong]
        S2 --> Catapulta
        S1 --> Catapulta
    end

    Senzor -.->|măsoară distanța până la panou| Panou
    Minge -.->|aruncare| Inel
```

**Conexiunile Componentelor:**
Senzorul HC-SR04 este conectat la placa NUCLEO-U545RE-Q prin doi pini GPIO: unul pentru impulsul de declanșare (`TRIG`) și unul pentru citirea impulsului de răspuns (`ECHO`). Servo-ul MG995 rotește baza lansatorului, iar servo-ul SG90 acționează mecanismul de declanșare al catapultei. Butonul este folosit pentru comandă, iar LED-ul RGB oferă feedback vizual pentru stările sistemului.

Servomotoarele sunt alimentate separat, folosind sursa de laborator în timpul testelor și modulul XL4005 pentru o alimentare stabilizată la 5-6V. Masa sursei pentru servomotoare este comună cu masa plăcii Nucleo, însă tensiunea `+5V_SERVO` nu este conectată la pinul de 5V al plăcii.

### Schema Electrică

Schema electrică a fost realizată în EasyEDA și arată conexiunile dintre placa NUCLEO-U545RE-Q, senzorul de distanță, servomotoare, buton, LED RGB și alimentarea externă a servourilor. Varianta curentă folosește HC-SR04 pentru detecție, conectat prin `TRIG` și `ECHO`.

![Schema electrică](./schema-electrica.webp)

### Conexiuni Curente

| Componentă | Pin componentă | Pin Nucleo | Pin STM32 | Rol |
| :--- | :--- | :--- | :--- | :--- |
| HC-SR04 | VCC | 5V extern | - | Alimentare senzor |
| HC-SR04 | GND | GND comun | - | Masă comună |
| HC-SR04 | TRIG | D14 | PB7 | Impuls de declanșare |
| HC-SR04 | ECHO | D15 | PB6 | Citire durată ecou |
| Servo MG995 | Signal | D6 | PB10 / TIM2_CH3 | Rotire bază |
| Servo SG90 | Signal | D5 | PB4 / TIM3_CH1 | Declanșare catapultă |
| Buton | Signal | D7 | PA8 | Pornire ciclu |
| LED RGB | R | D3 | PB3 | Feedback vizual |
| LED RGB | G | D9 | PC6 | Feedback vizual |
| LED RGB | B | D10 | PC9 | Feedback vizual |

Servomotoarele sunt alimentate dintr-o sursă externă de 5V. Masa sursei externe este comună cu masa plăcii Nucleo, dar alimentarea de 5V a servourilor nu este conectată la pinul de 5V al plăcii.

## Galerie și demonstrații

### Fotografii

![Lansator - vedere de sus](./lansator-vedere-sus.webp)

![Lansator - vedere laterală](./lansator-lateral.webp)

![Ansamblu lansator și coș](./ansamblu-lansator-cos.webp)

![Coș cu panou gri](./cos-panou.webp)

### Video

Test de calibrare pentru servomotorul de bază: deși modelul era marcat ca servo de 180°, în teste s-a comportat ca un servo cu rotație continuă, rotindu-se necontrolat. Acest test a fost folosit pentru diagnosticarea problemei și pentru ajustarea strategiei de control.

<video controls preload="metadata" style={{width: '100%', maxWidth: '720px', aspectRatio: '16 / 9', background: '#111'}}>
  <source src={require('./servo-180-grade.mp4').default} type="video/mp4" />
</video>

Demonstrație reușită:

<video controls preload="metadata" style={{width: '100%', maxWidth: '720px', aspectRatio: '16 / 9', background: '#111'}}>
  <source src={require('./demo-reusit.mp4').default} type="video/mp4" />
</video>

Test nereușit folosit pentru calibrare:

<video controls preload="metadata" style={{width: '100%', maxWidth: '720px', aspectRatio: '16 / 9', background: '#111'}}>
  <source src={require('./test-esuat.mp4').default} type="video/mp4" />
</video>

## Jurnal de Proiect

### Săptămâna 1

Am realizat documentația inițială a proiectului și am stabilit ideea generală: un lansator automat pentru mingi de ping-pong, orientat către un coș de basket în miniatură. În această etapă am descris obiectivul proiectului, motivația, arhitectura inițială și principalele componente hardware/software.

### Săptămâna 2

Am simplificat arhitectura sistemului. În loc să pun electronică pe coș, am decis ca senzorul să fie montat pe lansator, iar coșul să rămână o țintă pasivă. Lansatorul este deplasat pe un cerc în jurul coșului, iar sistemul trebuie să determine unghiul potrivit de orientare către panou. Pentru detecție am ales inițial senzorul VL53L0X, care măsoară distanța până la panoul gri din spatele coșului.

### Săptămâna 3

Am ales și cumpărat componentele principale: senzorul VL53L0X, servomotorul MG995 pentru orientarea bazei, servomotorul SG90 pentru declanșare, modulul XL4005 pentru alimentarea servomotoarelor, breadboard-ul, modulul de buton și modulul LED RGB. Tot în această etapă am stabilit că alimentarea servourilor va fi separată de placa Nucleo, folosind sursa de laborator în timpul testelor.

### Săptămâna 4

Am construit coșul fizic, format din bază, suport vertical, panou gri și inel metalic. Am ales panoul gri ca suprafață de detecție pentru senzor, deoarece este mai ușor de identificat decât inelul propriu-zis. De asemenea, am ales modelul de catapultă printată 3D pentru mingi de ping-pong și am pregătit fișierele necesare pentru printare.

### Săptămâna 5

Am realizat schema electrică în EasyEDA și am asamblat partea hardware principală a proiectului. Schema include placa NUCLEO-U545RE-Q, senzorul VL53L0X, servomotoarele MG995 și SG90, butonul, LED-ul RGB, condensatorul de filtrare și alimentarea externă pentru servouri. Am documentat explicit faptul că `+5V_SERVO` este alimentare externă și nu trebuie conectată la pinul de 5V sau 3V3 al plăcii Nucleo, iar masa este comună între alimentarea servourilor și placa de dezvoltare.

### Săptămâna 6

Am trecut de la senzorul VL53L0X la senzorul ultrasonic HC-SR04, deoarece în teste senzorul ToF nu oferea citiri stabile pentru panoul coșului. Am actualizat firmware-ul în Rust pentru citirea distanței prin `TRIG` și `ECHO`, am adăugat loguri RTT pentru fiecare citire și am integrat scanarea bazei cu servomotorul MG995. În această etapă am testat și mecanismul de declanșare cu SG90, alimentarea externă pentru servouri și primele aruncări ale catapultei.

### Săptămâna 7

Am asamblat varianta curentă a prototipului: catapulta printată 3D este montată pe baza rotativă, senzorul HC-SR04 este montat frontal pe lansator, iar coșul cu panou gri este folosit ca țintă. Am realizat fotografii și video-uri cu montajul, testele de servo și demonstrațiile de lansare, pe care le-am adăugat în documentație.

## Hardware

Sistemul utilizează următoarele componente hardware principale:

* **NUCLEO-U545RE-Q:** Placa de dezvoltare folosită pentru controlul sistemului.
* **HC-SR04:** Senzor ultrasonic folosit în varianta curentă pentru măsurarea distanței până la panoul coșului.
* **VL53L0X:** Senzor ToF testat inițial pentru detecția panoului.
* **Servo MG995:** Servomotor pentru rotirea bazei lansatorului.
* **Servo SG90:** Servomotor pentru mecanismul de declanșare.
* **XL4005 Step-Down:** Modul coborâtor de tensiune pentru alimentarea servomotoarelor.
* **Buton:** Element de control pentru pornirea scanării sau declanșare.
* **LED RGB:** Element de feedback vizual.
* **Catapultă printată 3D:** Mecanismul de lansare pentru mingea de ping-pong.
* **Coș cu panou gri:** Ținta fizică folosită pentru testare și detecție.

Protocoale și semnale utilizate:

* **GPIO TRIG/ECHO:** Citirea senzorului ultrasonic HC-SR04.
* **PWM:** Controlul servomotoarelor MG995 și SG90.
* **GPIO:** Citirea butonului și controlul LED-ului RGB.

## Listă de materiale

| Dispozitiv | Utilizare | Preț Estimativ |
| :--- | :--- | :--- |
| NUCLEO-U545RE-Q | Unitatea centrală de control | disponibilă |
| Senzor HC-SR04 | Detectarea panoului coșului în varianta curentă | disponibil |
| Senzor VL53L0X | Test inițial pentru detecția panoului | ~30 RON |
| Servo MG995 | Orientarea orizontală a lansatorului | ~30 RON |
| Servo SG90 | Mecanismul de declanșare | ~10 RON |
| XL4005 Step-Down | Alimentarea stabilizată a servomotoarelor | ~14 RON |
| Breadboard | Prototipare conexiuni | ~7 RON |
| Modul LED RGB | Feedback vizual | ~2 RON |
| Modul buton | Control utilizator | ~4 RON |
| Fire, rezistențe, condensator | Conexiuni și stabilizare alimentare | disponibile |
| Catapultă printată 3D | Mecanism de aruncare | realizată |
| Coș și panou | Țintă pentru lansator | realizat |
| **Total cumpărat estimativ** | **-** | **~97 RON** |

## Software

### Prezentare Generală

Implementarea software folosește Rust pe placa NUCLEO-U545RE-Q. Aplicația controlează servomotoarele prin PWM, citește senzorul HC-SR04 prin semnale GPIO `TRIG`/`ECHO` și gestionează stările sistemului în funcție de buton și de distanțele măsurate.

### Design Detaliat

Codul este structurat pe patru module logice:

1. **Gestionarea stărilor:** Controlează succesiunea operațiunilor: IDLE, SCANARE, ALINIERE, AȘTEPTARE și DECLANȘARE.
2. **Scanarea panoului:** Rotește baza lansatorului pe un interval de unghiuri și citește distanța măsurată de HC-SR04.
3. **Controlul motoarelor:** Generează semnalele PWM pentru MG995 și SG90.
4. **Interfața utilizator:** Citește butonul și controlează LED-ul RGB pentru feedback.

### Diagramă Funcțională

```mermaid
flowchart TD
    A([Start Sistem]) --> B[Așteptare comandă]
    B -->|Apăsare buton| C[Scanare cu HC-SR04]
    C --> D[Determinare direcție panou]
    D --> E[Rotire MG995 către țintă]
    E --> F[Așteptare 3 secunde]
    F --> G[Declanșare SG90]
    G --> H([Resetare stare])
    H -.-> B
```
