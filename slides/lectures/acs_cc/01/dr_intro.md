# DISCLAIMER

<br>

* These slides represent a summary.
* The slides do not cover all the explanations, simulations, or demonstrations provided during the course.
* The slides do not limit, in any way, the material required for the exam.
* For the complete version, you are welcome to attend the course.

<br>
<br>
<br>
<br>
<br>

> (copyright info) These slides may contain materials shared with my colleagues Alexandru Radovici, Dan Tudose, Alexandru Vaduva, Razvan Tataroiu


---

# ENG

<div grid="~ cols-2 gap-20">
<div>


<br> 

### Scientific understanding of the natural world 
<br>

### Used to invent, design, and build things 

<br>

### Used to solve problems and achieve practical goal


</div>

<img src="/img/ing.png" class="w-80">

</div>

---

# Abstract level 

<img src="/img/abstract.png" class="w-full mx-auto block">

---

# Why PM


## Computing systems with microprocessors > everywhere

<br>

## Questions for an engineer:

* What is inside a computing system?
* How do the components interact?
* How do I design a system that interacts with the physical environment?
* How do I choose the best hardware option for an embedded system?


## "Data-based decisions" – based on IoT infrastructure require:

* Actual physical sensors
* Lots of IoT custom hardware

---

# 2026 VS 2025 


| **Modification**                                                                                     | **How it helps**                                                                                                                                                                                     |
| ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| We are dropping the Raspberry Pi Pico and moving to an ST MCU board with an integrated debugger. | Fewer hardware-related issues (the current Pico generation has had multiple hw problems). Also removes the need for 2 separate boards for the project. |
| New lab boards (as a shield) designed for the selected ST variant.                               | Cleaner, more reliable lab setup and easier reproduction of results across students.                                                                                                             |
| Added protection features on the shield (general I/O protections).                               | Fewer board failures and fewer “mystery” faults caused by wiring mistakes or accidental shorts.                                                                                                  |
| Dedicated power-supply protections (lab PCs have unstable/problematic USB power).                | More stable power, fewer USB-related resets/brownouts, and reduced risk of damaging boards.                                                                                                      |
| Boards implemented and extensively tested over the summer.                               | Higher reliability from day one in the lab; fewer unexpected issues during the semester.                                                                                                         |

---

# 2026 VS 2025 


| **Modification**                                                                                                                 | **How it helps**                                                                                                                                         |
| ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Each student will receive one board for the project.                                                                         | Lower overall costs for students.                                              |
| Add an introductory Rust lab. Increase the intro sequence to **3** labs (instead of **2**) before the GPIO lab.              | More time to get comfortable with Rust and embedded workflows before moving to hardware-facing labs (GPIO), reducing friction early in the semester. |
| Lab grading becomes attendance-based: **10 points** for being present and successfully flashing/uploading code to the board. | Less “grade pressure” during lab sessions and less focus on chasing points; more focus on learning and experimenting.                                |
| Publish lab solutions during the semester (for each lab, in the following week).                                             | Students get more code examples earlier, and can reuse patterns for the project while the semester is ongoing.
