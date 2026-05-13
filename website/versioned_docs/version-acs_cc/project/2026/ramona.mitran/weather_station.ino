/*
 * ================= TABEL PINI ESP32 =================
 * BME280 (SDA)   | GPIO 21     | 3V3
 * BME280 (SCL)   | GPIO 22     | 3V3
 * ML8511 (OUT)   | GPIO 34     | 3V3 
 * MQ-135 (A0)    | GPIO 35     | 5V / VIN !
 * Buton Upload   | GPIO 27     | GND (pe diagonală)
 * Buton Mute     | GPIO 26     | GND (pe diagonală)
 * Buzzer (+/S)   | GPIO 25     | GND
 * ====================================================
 */

#include <WiFi.h>
#include "Adafruit_MQTT.h"
#include "Adafruit_MQTT_Client.h"
#include <Wire.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_BME280.h>

// Setări Wi-Fi și Adafruit IO

#define WLAN_SSID       "NUME_RETEA_WIFI"
#define WLAN_PASS       "PAROLA_WIFI"
#define AIO_USERNAME    "userul_tau_adafruit"
#define AIO_KEY         "CHEIA_TA_SECRETA_ADAFRUIT"

#define AIO_SERVER      "io.adafruit.com"
#define AIO_SERVERPORT  1883

// Pini Hardware
const int UV_PIN = 34; 
const int MQ135_PIN = 35;
const int BUTTON_PIN = 27; 
const int MUTE_BUTTON_PIN = 26;
const int BUZZER_PIN = 25;

// Praguri de alarmă
const float PRAG_TEMP = 28.0;     
const uint32_t PRAG_GAZE = 1000;  

Adafruit_BME280 bme;
WiFiClient client;
Adafruit_MQTT_Client mqtt(&client, AIO_SERVER, AIO_SERVERPORT, AIO_USERNAME, AIO_KEY);

// Fluxuri Adafruit IO
Adafruit_MQTT_Publish tempFeed = Adafruit_MQTT_Publish(&mqtt, AIO_USERNAME "/feeds/temperatura");
Adafruit_MQTT_Publish humFeed  = Adafruit_MQTT_Publish(&mqtt, AIO_USERNAME "/feeds/umiditate");
Adafruit_MQTT_Publish presFeed = Adafruit_MQTT_Publish(&mqtt, AIO_USERNAME "/feeds/presiune");
Adafruit_MQTT_Publish uvFeed   = Adafruit_MQTT_Publish(&mqtt, AIO_USERNAME "/feeds/uv");
Adafruit_MQTT_Publish gasFeed  = Adafruit_MQTT_Publish(&mqtt, AIO_USERNAME "/feeds/gaze");

// Variabile Globale
float t = 0, h = 0, p = 0, uvIndex = 0;
uint32_t mqRaw = 0;

unsigned long lastReadTime = 0;
const unsigned long readInterval = 3000;

// Variabile Butoane
bool lastButtonState = HIGH; 
bool lastMuteButtonState = HIGH;
unsigned long lastDebounceTime = 0;
unsigned long lastMuteDebounce = 0;
const unsigned long debounceDelay = 50; 

// Variabile Snooze/Mute
bool isSnoozed = false;
unsigned long snoozeStartTime = 0;
const unsigned long SNOOZE_DURATION = 300000; // 5 minute in milisecunde (300.000 ms)

// Functii Buzzer
void beepSuccess() {
  for(int i = 0; i < 200; i++) {
    digitalWrite(BUZZER_PIN, HIGH); delayMicroseconds(500);
    digitalWrite(BUZZER_PIN, LOW);  delayMicroseconds(500);
  }
}

void alarmaTemperatura() {
  Serial.println("ALARMĂ: Temperatură ridicată!");
  for(int j = 0; j < 3; j++) {
    for(int i = 0; i < 100; i++) {
      digitalWrite(BUZZER_PIN, HIGH); delayMicroseconds(200);
      digitalWrite(BUZZER_PIN, LOW);  delayMicroseconds(200);
    }
    delay(100);
  }
}

void alarmaGaze() {
  Serial.println("ALARMĂ: Nivel gaze periculos!");
  for(int i = 0; i < 500; i++) {
    digitalWrite(BUZZER_PIN, HIGH); delayMicroseconds(800);
    digitalWrite(BUZZER_PIN, LOW);  delayMicroseconds(800);
  }
}

float mapfloat(float x, float in_min, float in_max, float out_min, float out_max) {
  return (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

void MQTT_connect() {
  if (mqtt.connected()) return;
  Serial.print("Conectare la Adafruit IO... ");
  int8_t ret; uint8_t retries = 3;
  while ((ret = mqtt.connect()) != 0) {
       Serial.println(mqtt.connectErrorString(ret));
       mqtt.disconnect(); delay(2000); retries--;
       if (retries == 0) { Serial.println("Esec conectare."); return; }
  }
  Serial.println("Conectat!");
}

void setup() {
  Serial.begin(115200);

  pinMode(BUTTON_PIN, INPUT_PULLUP);
  pinMode(MUTE_BUTTON_PIN, INPUT_PULLUP);
  pinMode(BUZZER_PIN, OUTPUT);

  if (!bme.begin(0x76)) Serial.println("Eroare: BME280!");
  
  Serial.print("Conectare Wi-Fi...");
  WiFi.begin(WLAN_SSID, WLAN_PASS);
  while (WiFi.status() != WL_CONNECTED) { delay(500); Serial.print("."); }
  Serial.println("\nWiFi Conectat!");
}

void loop() {
  // ================= 1. CITIRE BUTON UPLOAD =================
  bool currentButtonState = digitalRead(BUTTON_PIN);
  if (currentButtonState != lastButtonState) lastDebounceTime = millis(); 
  if ((millis() - lastDebounceTime) > debounceDelay) {
    if (currentButtonState == LOW) {
      Serial.println("\n>>> BUTON UPLOAD DETECTAT <<<");
      MQTT_connect();
      if (mqtt.connected()) {
        bool success = tempFeed.publish(t) && humFeed.publish(h) && presFeed.publish(p) && uvFeed.publish(uvIndex) && gasFeed.publish(mqRaw);
        if (success) { Serial.println("Upload SUCCES!"); beepSuccess(); } 
        else { Serial.println("Eroare upload."); }
        mqtt.disconnect(); 
      }
      while(digitalRead(BUTTON_PIN) == LOW) { delay(10); }
    }
  }
  lastButtonState = currentButtonState;

  // ================= 2. CITIRE BUTON MUTE/SNOOZE =================
  bool currentMuteState = digitalRead(MUTE_BUTTON_PIN);
  if (currentMuteState != lastMuteButtonState) lastMuteDebounce = millis();
  if ((millis() - lastMuteDebounce) > debounceDelay) {
    if (currentMuteState == LOW) {
      isSnoozed = true;
      snoozeStartTime = millis();
      Serial.println("\n>>> BUTON MUTE APASAT - Alarme oprite pt 5 minute! <<<");
      while(digitalRead(MUTE_BUTTON_PIN) == LOW) { delay(10); } // asteapta eliberarea
    }
  }
  lastMuteButtonState = currentMuteState;

  // Resetare automată Snooze după 5 minute
  if (isSnoozed && (millis() - snoozeStartTime >= SNOOZE_DURATION)) {
    isSnoozed = false;
    Serial.println("\n>>> SNOOZE EXPIRAT - Alarmele sunt din nou active! <<<");
  }

  // ================= 3. CITIRE SENZORI =================
  if (millis() - lastReadTime >= readInterval) {
    lastReadTime = millis();
    
    t = bme.readTemperature();
    h = bme.readHumidity();
    p = bme.readPressure() / 100.0F;

    int uvRaw = analogRead(UV_PIN);
    float uvVoltage = uvRaw * (3.3 / 4095.0);
    uvIndex = mapfloat(uvVoltage, 0.99, 2.8, 0.0, 15.0);
    if (uvIndex < 0.0) uvIndex = 0.0;

    mqRaw = analogRead(MQ135_PIN);

    // Declanșare alarme DOAR dacă nu suntem în Snooze
    if (!isSnoozed) {
      if (t > PRAG_TEMP) alarmaTemperatura();
      if (mqRaw > PRAG_GAZE) alarmaGaze();
    }
  }
}
