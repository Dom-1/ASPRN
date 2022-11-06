from microbit import *

uart.init(115200)
while True:
    data = accelerometer.get_strength()
    uart.write("Accl (milli-g): " + str(data))
    uart.write("\r")
    sleep(1000)
