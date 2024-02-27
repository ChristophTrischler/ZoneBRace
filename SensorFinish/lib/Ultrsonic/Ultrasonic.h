

class Ultrasonic
{
private:
    int triger, echo; 
public:
    Ultrasonic(int triger,int echo);
    void init();
    float measurDistance();
    void waitFortrigger(); 
    void waitFortrigger(void(*update)());  
}; 
