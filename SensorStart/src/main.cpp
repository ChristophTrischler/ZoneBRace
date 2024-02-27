#include <Arduino.h>
#include <WiFi.h>
#include <U8x8lib.h>
#include <LoRa.h>
#include <NTPClient.h>

#include "Ultrasonic.h"
#include "WifiSetup.h"
#include "secrets.h"
#include "HTTPClient.h"


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

WifiSetup wifi(ssid, pw, "Start Sensor");

WiFiUDP wifiudp; 
NTPClient timeclient(wifiudp); 

#define url "https://zone-b-race.shuttleapp.rs"
#define token "15857776252302822368" 
HTTPClient http; 

int getPopQue();
void postTime(const char* path, int run_id, long time);

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

  //wifi
  if(!wifi.connect()) {
    Serial.print("wifi failed"); 
    display.clear(); 
    display.drawString(0,0,"wifi failed"); 
    while (1); 
  }

  // //time 
  timeclient.update(); 

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

void loop() {
  display.clear(); 
  display.drawString(0,0,"waiting for"); 
  display.drawString(0,1,"next rider"); 
  int id = 0 ; 
  while (id == 0 ){
    sleep(5); 
    id = getPopQue(); 
  }
  display.clear();                               
  display.drawString(0,0,"waiting for");
  display.drawString(0,1,"start");  
  sensor.waitFortrigger(); 
  long start_time = timeclient.getEpochTime(); 
  postTime(url"/data/start", id, start_time);

  display.clearLine(1); 
  display.drawString(0,1,"finish");

  //wait for other sensor 
  while(1) {
    if (LoRa.parsePacket() && LoRa.available()) {
      String l = LoRa.readString(); 
      Serial.println(l); 
      if (l.equals("finished")) break;
    }
  }
  long finish_time = timeclient.getEpochTime(); 
  postTime(url"/data/finish", id, finish_time); 
  delay(1500);
}


int getPopQue() {
  http.begin(url"/que/pop"); 
  http.addHeader("token", token);
  if(http.GET()!=200) return 0;
  String payload = http.getString();
  int id;  
  if(sscanf(payload.c_str(),"%d", &id) != 1) return 0; 
  return id;  
}

void postTime(const char* path, int run_id, long time){
  http.begin(path);
  http.addHeader("Content-Type", "application/x-www-form-urlencoded"); 
  http.addHeader("token", token); 
  char payload[255]; 
  sprintf(payload, "id=%d&time=%d", run_id, time); 
  if (http.POST(payload) != 200)Serial.println("error posting time");
  display.clear(); 
  display.drawString(0,3,"ERROR");  
}