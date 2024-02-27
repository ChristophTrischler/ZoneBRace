#include "Ultrasonic.h"
#include "Arduino.h"

Ultrasonic::Ultrasonic(int triger, int echo) {
    this->triger = triger; 
    this->echo = echo; 
}

void Ultrasonic::init(){
  pinMode(this->triger, OUTPUT); 
  pinMode(this->echo, INPUT_PULLDOWN);     
}

float Ultrasonic::measurDistance() {
    digitalWrite(triger, HIGH); 
    delayMicroseconds(10);
    digitalWrite(triger, LOW); 
    long time = pulseIn(echo, HIGH);
    float distance =  0.017 * time;
    return distance;   
}

void Ultrasonic::waitFortrigger() {
  float last = 0.1; 
  float diff = 0; 
  while (diff<0.3){
    float current = this->measurDistance();
    diff = (last - current)/last; 
    last = current;
    Serial.printf("dis: %f, diff: %f\n", current, diff);
    delay(100); //preventing echo form last triger 
  }
  
}

void Ultrasonic::waitFortrigger(void(*update)()) {
  float last = -1; 
  float diff = 0; 
  while (diff<0.2){
    float current = this->measurDistance(); 
    diff = (last - current)/last; 
    last = current;
    update();
  }
}