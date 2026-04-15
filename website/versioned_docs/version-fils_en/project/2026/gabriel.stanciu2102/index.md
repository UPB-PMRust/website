# Overture
A medical-grade perfusor

:::info

**Author**: Gabriel Stanciu\
**GitHub Project Link**: https://github.com/stanciugabriel

::::

## Description
This is a medical device intended to push on the syringe's plunger at a specified flow rate. While that seems simple, it achives ~0.01mL accuracy which is crucial when administering medication like noradrenaline, propofol etc. Overture is feature-packed with useful features: Automatic syringe and medication detection via NFC stickers, bolus mode(ability to push medication as fast as possible), drug library, kvo(keep vein open) - at the end of the dosage, it pushes medication just so little so that the veins dont close. Usually, when trying to improve on an existing product, you should first pick one existing solution from the market as "the gold standard" you want to dethrone. For me it is the [B.Braun Space Plus Perfusor](https://catalogs.bbraun.com/en-01/p/PRID00011858/spaceplus-perfusor?bomUsage=marketingDocuments), which is also the device we used to use on the ambulance(we used the previous generation but things are kinda the same, they just slapped a touchscreen on the old device).

## Motivation
While I was a paramedic on the SMURD ambulances, I was constantly working with perfusors. While they were working as intended, their menus were clunky which meant losing valuable time, especially when someone's life is at stake.

## Log
## Week 5
I already know what I want to build so naturally the first thing i would search is if there are any open-source projects that achieve a similar task. I did find a really cool project - an open-source syringe pump meant for laboratory in mass spectroscopy developed at [Moscow State University by Andrey Samokhin](https://www.mass-spec.ru/projects/diy/syringe_pump/eng/). Naturally I started ordering what i needed for the project and started 3d printing parts.

## Week 6
I started testing the NEMA17 motor, but I cannot drive it very well because I found at a later date that the driver needs a big enough voltage difference across the input and motor output, and in my current config it was almost the same voltage. This resulted in really low torque, low speed and jittering. I got the 3D printed parts, and I can almost assemble it, but I still need to wait on some bearings and screws to arrive. In the meantime I started designing the UI interface in Figma as this is my UI/UX tool of choice. 

## UI
 Welcome to the first major part of this project. Making a really good user interface is mandatory as this is how the user interacts with the device. I knew that this can be some sort of challenge as the screen I had on hand has only 170x320px and that meant only one thing: *prioritizing*. I had to make crucial information pop-up, make sure data has some hierarchy, and over all that is intuitive. I started making the screen where a user would spend most of the time - the perfusion data screen. 
The interface needed to be really fast to read, especially the significant data like the medication and concentration, the flow rate, VTBI, and time remaining as well as the state of the device(paused, perfusing) because at flow rates like 0.1 mL/h you can hardly see its movement. I decided to go with a grid design and decided on this layout. 
![Screen Blueprint](design_blueprint.webp)

Later on, I used a black background and accent colors. The contrast is really high, the medication being delivered and its concentration has a bright backdrop to make the eye naturally go there, and the same color is applied to the labels. Having a grid-like design means users know where they can find certain information based on position. Postion that is predictable. I also wanted to get a feel for how this would look like on the actual screen, and check if it is readable.

![Display Test](display_test.webp)

I really liked how it ended up on the screen. Now, when designing the flow(or state machine) of this device I had to keep in mind that I would navigate it using a rotary encoder and buttons. Also, I wanted to make it simple enough that I could easily implement it using embedded-graphics crate. To cut to the chase, you can find below all the screens that I designed.
![All screens](screens.webp)
An artist is never fully happy with its work, but I had to settle for this. For now, at least.

## Hardware

| Device | Usage | Price |
|--------|--------|-------|
|Raspberry Pico 2W | Microcontroller | 35 RON|
|ST7789 Display|	display	|0 RON (had it on hand)|
|NEMA 17 Stepper	|used to move the syringe|	60 RON|
|TMC2208 Driver	|used to drive the stepper|	30 RON|
|NFC Module	|to scan the nfc stickers on the syringe |10 RON|
|Bearings|LM8UU, 688ZZ|25RON|
|Guide Rail|8mm x 315mm|30RON|
|Lead Screw| 8mm x 315mm, 2mm pitch |20RON |
|Misc Hardware| screws, bolts, nuts|10 RON|
