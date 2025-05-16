---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Memory Protection
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: MA - 09 - Secure Execution
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: ma-09
background:
---

# Secure Execution
Lecture 9

---
---

# Secure Execution

- Security Model
  - Secure
  - Non Secure
  - Non Secure Callable
- ARM TrustZone
- RP2350 Secure 
- Software

---
---
# Memory Types

| Type | Symbol | Description |
|-|-|-|
| *Secure* | **S** | Can be accessed only by code running in **secure mode** |
| *NonSecure Callable* | **NSC** | code running in **non-secure mode** can make function calls into it with some restrictions |
| *NonSecure* | **NS** | any code running in **any mode** can access it |

---
---
# Security Attribution Unit

