#include <Arduino.h>
#include "WifiSetup.h"

WifiSetup::WifiSetup(const char ssid[], const char password[], const char hostname[], uint8_t tries, uint8_t connectTimeout){
    this->ssid = ssid;
    this->password = password;
    this->tries = tries;
    this->hostname = hostname;
    this->connectTimeout = connectTimeout;
}

bool WifiSetup::connect(){
    WiFi.setHostname(this->hostname);
    for (uint8_t i = 0; i < this->tries; i++)
    {
        WiFi.begin(this->ssid, this->password);

        for (uint8_t j = 0; j < this->connectTimeout; j++)
        {
            sleep(1);
            if (this->status() == WL_CONNECTED)
            {
                return true;
            }
        }
    }
    return false;
}

bool WifiSetup::update(){
    if (this->status() == WL_CONNECTED)
    {
        return true;
    }

    Serial.print("Connecting to WiFi \"" + String(this->ssid) + "\"...");
    if (!this->connect())
    {
        Serial.println(" error.");
        return false;
    }
    else
    {
        Serial.println(" done.");
        return true;
    }
}

wl_status_t WifiSetup::status(){
    return WiFi.status();
}

