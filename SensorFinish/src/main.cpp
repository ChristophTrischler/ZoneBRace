#include <Arduino.h>
#include <U8x8lib.h>
#include <LoRa.h>

#include "Ultrasonic.h"

// OLED Pins
#define OLED_SCL 15   // GPIO 15
#define OLED_SDA  4   // GPIO  4
#define OLED_RST 16   // GPIO 16
 
// LoRa Pins
#define LoRa_SCK 5
#define LoRa_MISO 19
#define LoRa_MOSI 27
#define LoRa_SS 18
#define LoRa_RST 14
#define LoRa_DIO0 26

//LoRa Frequenz   
//866E6 for Europe
//866MHz 
#define BAND 866E6 

// UltrasonicSensor Pins
#define USS_Triger 13 
#define USS_Echo 12 

Ultrasonic sensor(USS_Triger, USS_Echo); 
U8X8_SSD1306_128X64_NONAME_SW_I2C display(/* clock=*/ OLED_SCL, /* data=*/ OLED_SDA, /* reset=*/ OLED_RST);

void setup() {
  Serial.begin(9600);
  Serial.flush(); 
  Serial.println("started");  
  //start display 
  display.begin();
  display.setPowerSave(0); 
  display.setFont(u8x8_font_5x7_f);
  Serial.println("started display"); 
  display.drawString(5,5, "started");

  
  //SPI LoRa pins
  SPI.begin(LoRa_SCK, LoRa_MISO, LoRa_MOSI, LoRa_SS);
  //setup LoRa transceiver module
  LoRa.setPins(SS, LoRa_RST, LoRa_DIO0);
  if (!LoRa.begin(BAND)) {
    Serial.println("Starting LoRa failed!");
    display.clear();
    display.drawString(0, 0, "Starting LoRa");
    display.drawString(0,1, "failed");  
    while (1);
  } 
  LoRa.setSyncWord(0xFF);
  Serial.println("started LoRa"); 

  //init sensor
  sensor.init();
  Serial.println("started Sensor"); 
}

void waiting() {
  Serial.print(".");
}

void loop() {
  display.clear(); 
  sensor.waitFortrigger(waiting); 
  LoRa.beginPacket(); 
  LoRa.print("finished");
  Serial.println("finished"); 
  LoRa.endPacket(); 
  display.fillDisplay();  
  delay(5000);  
}

