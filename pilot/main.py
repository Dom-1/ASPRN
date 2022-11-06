from microbit import *

while True:
    print(microbit.accelerometer.get_values())
    sleep(1000)

