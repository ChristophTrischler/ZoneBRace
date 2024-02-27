#include <Arduino.h>
#include <WiFi.h>

class WifiSetup
{
private:
    const char *ssid;
    const char *password;
    const char *hostname;

    uint8_t tries;
    uint8_t connectTimeout; 
public:
    WifiSetup(const char ssid[], const char password[], const char hostname[], uint8_t tries = 4, uint8_t connectTimeout = 15);

    bool connect();

    bool update();

    wl_status_t status();
};

