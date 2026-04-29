# Why is the output not ideal?

<div grid="~ cols-2 gap-2" m="t-2">

<div>

### The two corresponding voltage output levels are affected by:
- power supply voltage
- output current 
- temperature 
- variations in the manufacturing process 
</div>

<div>
<img src="./d_output.svg" class="rounded">
</div>

</div>

---
layout: two-cols
---

# ICs same voltage


### Usually will work as is

- usually, they will be compatible
- conditions:

$$ V_{OH\_transmiter} > V_{IH\_receiver} $$
$$ V_{OL\_transmiter} < V_{IL\_receiver} $$

:: right ::

 <img src="./ICs_same_voltage.svg" class="rounded h 80">

---
layout: two-cols
---

# VCC1 > VCC2


### Might work, might produce magic smoke
<div style="color: red;">

$$ V_{OH\_transmiter} > VCC_{receiver} $$

**PROBLEM**

</div>

### Solutions:
- level shifter
- resistor divider / voltage limiter 

Examples:

- [Bi-Directional Level Shifter with 4 Channels](https://www.optimusdigital.ro/ro/interfata-convertoare-de-niveluri/181-translator-de-nivel-bidirectional-cu-4-canale.html)
- [Level Shifter Multi-Channel](https://www.tme.eu/en/details/tp240610/development-kits-accessories/total-phase/level-shifter/)
- [8 Channels Level Shifter](https://www.adafruit.com/product/395)

:: right ::

 <img src="./vcc1_larger_vcc2.svg" class="rounded h 80">
 

---
layout: two-cols
---

# VCC1 < VCC2


### Might work

<div style="color: orange;">

$$ V_{CC\_transmiter} \lesssim V_{IH\_receiver} $$ 

**Might work in an intermittent mode - hard to debug!**

</div>

### Solutions:
- level shifter
- resistor divider / voltage limiter 

Examples:

- [Bi-Directional Level Shifter with 4 Channels](https://www.optimusdigital.ro/ro/interfata-convertoare-de-niveluri/181-translator-de-nivel-bidirectional-cu-4-canale.html)
- [Level Shifter Multi-Channel](https://www.tme.eu/en/details/tp240610/development-kits-accessories/total-phase/level-shifter/)
- [8 Channels Level Shifter](https://www.adafruit.com/product/395)

:: right ::

 <img src="./VCC1_smaller_VCC2.svg" class="rounded h 80">


---

# Why Pull-Down R

<div grid="~ cols-2 gap-2" m="t-2">

<div>

- Without pull-down – when the button is not pressed, it leaves the input pin floating.
- The second design ensures that the voltage level has a well-defined state, regardless of the button's state.
- R1 is called a "pull-down" resistor.

</div>

<div align="center">
 <img src="./pull_down.png" class="rounded" h="100">
</div>

 </div>

---

# Why Pull-Up R

<div grid="~ cols-2 gap-2" m="t-2">

<div>

- Same reasoning
- R1 is called a "pull-up" resistor.

##Obs:
- most microcontrollers have at least a pull-up resistor incorporated on GPIOs - that can be activated in software
- some have both pull-up and pull-down
- typically, these are sized for a 50 - 10 nA current consumption

</div>

<div align="center">
<img src="./pull_up.png" class="rounded" h="50">
</div>

</div>

---

# Notes on output pins

- most microcontrollers have a limit of around 10mA per output PIN
- ! do not connect an LED without a resistor in series (to limit the current) 
- ! do not connect a motor / any type of inductive load

## Solutions:
- use a transistor
- use an IC with incorporated Darlinghtons (eg: ULN2003)
