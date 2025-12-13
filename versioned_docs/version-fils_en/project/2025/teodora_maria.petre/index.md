# Smart Home Devices

This project aims to automate basic home devices based on the state of the room.

:::info

**Author:** Petre Teodora-Maria
**GitHub Project Link:**

:::

##Description

This project implements a Secure Access Smart Door system using the STM32 microcontroller and the Rust programming language. The main goal is to provide autonomous entry control: the door system activates when motion is detected and grants access only after a person provides the correct PIN code or RFID scan. A servo motor manages the lock, and colored LEDs, along with a screen, provide feedback, displaying a personalized welcome message upon successful authentication.


##Motivation

Since I was little I used to make dollhouses out of cardboard, but I’ve always felt like something was missing. I wanted to make them as real as possible – functional lights, doors, almost everything that a real house has – but I didn’t know how to.

##Components Overview

* Servomotor SG90 – mechanically closes and opens the door
* PIR Motion Detector (HC-SR501) – detects the person approaching the door
* RFID Reader Module – collects the security credentials for authentication
* LEDs – provides immediate visual feedback for granted/denied access
* LCD Screen – displays personalized **“Welcome, [user’s name]”** message upon successful entry

##Schematics

