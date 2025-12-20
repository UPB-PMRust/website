
# HydroSense

An ESP32-based irrigation controller written in Rust, using soil moisture + rain sensing and MQTT telemetry.

:::info

**Author:** Cătălina Nedelcu-Holtea (`catalina_nedelcu.holtea`)  
**GitHub Project Link:**

:::

## Description

<div style="text-align: justify;">
HydroSense is an ESP32-based irrigation controller written in Rust, designed to automate watering decisions using real sensor data. The system reads a soil moisture sensor and a rain sensor, then switches a 12 V water pump through an N-channel MOSFET stage when irrigation is needed. In parallel, it publishes status and measurements via MQTT, which enables remote monitoring and makes testing/debugging significantly easier.
</div>

## Motivation

<div style="text-align: justify;">
I am particularly interested in embedded and IoT systems because they connect software to the physical world in a direct and measurable way—through sensing, decision-making, and reliable control of real hardware. These concepts are fundamental to understanding many technologies we interact with daily, from smart home devices and environmental monitoring to agriculture/greenhouse automation and industrial control.
</div>

<div style="text-align: justify;">
At the same time, HydroSense started from a personal need. I genuinely enjoy taking care of plants, but watering often becomes a routine task that is easy to forget during busy periods. I have experienced situations where inconsistent watering led to plants drying out and dying. This project is my practical solution: a small, dependable system that handles watering based on actual conditions, not memory or fixed schedules.
</div>

## Architecture

![Architecture diagram](./architecture.svg)

<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="460" viewBox="0 0 1024 460">
  <defs>
    <marker id="arrow" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
      <path d="M0,0 L10,3 L0,6 Z" fill="#111"/>
    </marker>
    <style>
      .box { fill: #fff; stroke: #111; stroke-width: 2; }
      .label { font-family: Arial, sans-serif; font-size: 16px; fill: #111; }
      .small { font-family: Arial, sans-serif; font-size: 13px; fill: #111; }
      .wire { stroke: #111; stroke-width: 2; fill: none; marker-end: url(#arrow); }
      .wire2 { stroke: #111; stroke-width: 2; fill: none; }
      .title { font-family: Arial, sans-serif; font-size: 20px; font-weight: bold; fill: #111; }
    </style>
  </defs>

  <text x="20" y="32" class="title">Architecture</text>

  <!-- Left side: Sensors -->
  <rect x="40" y="90" width="210" height="70" class="box"/>
  <text x="55" y="120" class="label">Soil moisture sensor</text>
  <text x="55" y="145" class="small">ADC / GPIO input</text>

  <rect x="40" y="190" width="210" height="70" class="box"/>
  <text x="55" y="220" class="label">Rain sensor</text>
  <text x="55" y="245" class="small">GPIO input</text>

  <!-- Center: ESP32 -->
  <rect x="335" y="130" width="240" height="120" class="box"/>
  <text x="355" y="170" class="label">ESP32</text>
  <text x="355" y="195" class="small">Control logic (automatic)</text>
  <text x="355" y="218" class="small">Wi-Fi + MQTT client</text>

  <!-- Right side: Relay + Pump -->
  <rect x="690" y="115" width="220" height="80" class="box"/>
  <text x="705" y="145" class="label">Relay module</text>
  <text x="705" y="170" class="small">GPIO control + power switch</text>

  <rect x="690" y="225" width="220" height="80" class="box"/>
  <text x="705" y="255" class="label">5 V water pump</text>
  <text x="705" y="280" class="small">Powered from 5 V supply</text>

  <rect x="690" y="335" width="220" height="70" class="box"/>
  <text x="705" y="365" class="label">Tube</text>
  <text x="705" y="390" class="small">Water delivery</text>

  <!-- Top: MQTT + phone -->
  <rect x="335" y="60" width="240" height="55" class="box"/>
  <text x="355" y="93" class="label">MQTT Broker</text>

  <rect x="690" y="45" width="220" height="70" class="box"/>
  <text x="705" y="75" class="label">Phone / Dashboard</text>
  <text x="705" y="100" class="small">Monitor + manual ON/OFF</text>

  <!-- Wires: sensors to ESP32 -->
  <path d="M250,125 L335,170" class="wire"/>
  <text x="262" y="140" class="small">ADC/GPIO</text>

  <path d="M250,225 L335,210" class="wire"/>
  <text x="262" y="220" class="small">GPIO</text>

  <!-- ESP32 to Relay -->
  <path d="M575,190 L690,155" class="wire"/>
  <text x="600" y="155" class="small">GPIO</text>

  <!-- Relay to Pump -->
  <path d="M800,195 L800,225" class="wire"/>
  <text x="812" y="220" class="small">switched 5 V</text>

  <!-- Pump to Tube -->
  <path d="M800,305 L800,335" class="wire"/>
  <text x="812" y="330" class="small">water flow</text>

  <!-- ESP32 to MQTT broker (Wi-Fi) -->
  <path d="M455,130 L455,115" class="wire"/>
  <text x="470" y="125" class="small">Wi-Fi</text>

  <!-- MQTT broker to phone (publish/subscribe) -->
  <path d="M575,87 L690,80" class="wire"/>
  <text x="602" y="70" class="small">pub/sub</text>

  <!-- Notes: automatic + manual -->
  <text x="335" y="280" class="small">Automatic control: soil dry AND no rain → pump ON</text>
  <text x="335" y="300" class="small">Manual override: phone publishes command via MQTT</text>

  <!-- Power note -->
  <text x="40" y="425" class="small">Note: Pump uses a dedicated 5 V supply; relay provides electrical switching between supply and pump.</text>
</svg>
