---
layout: section
---
# Signals
Digital Signals - Recap 

---

# Signals
Analog vs Digital

<div grid="~ cols-2">

<div>

- *analog signals* are *real* signals
- *digital signals* are *a numerical representation* of an analog signal (software level)
- hardware usually works with two-level digital signals (hardware level)

#### Exceptions
- in wireless and in high-speed cable communication things get more complicated

> for PCB level / between integrated circuits on the same board / inside the same chip - things are a "a little simpler" - as detailed in the following

</div>

![AD](./a_d.png)

</div>

---

# Why use digital in computing?

<div grid="~ cols-2">

<div>

Signal that we *want* to generate with an output pin

<div style="background: white; padding: 5px" class="rounded">

![Digital Step](./digital_step.svg)

</div>

</div>

<div>
<v-click>

Signal that what we actually generate

<div align="center" style="background: white; padding: 5px" class="rounded">

![Analog Step](./analog_step.svg)

</div>

</v-click>
</div>

</div>

> Why we still use it? Because after passing through an IC or a gate inside an IC - the signal is "rebuilt" and if the "digital discipline" described in the following is respected - we can preserve the information after numerous "passes". Thus, each element can behave with a large margin for error, yet the final result is correct.

---

# Noise Margin

<div align="center">

<div align="center" style="background: white; padding: 5px" class="rounded">

![Noise](./noise.svg)

</div>

</div>
