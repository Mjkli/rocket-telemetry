# 08-31-2026
Starting this document. I have setup the arduino to collect Acceleration / gyro data from the MPU6050, 
and Pressure data from the BMP280. I am able to derive the elevation in meters currently.
I want to start looking into the mathmatical formulas needed to calculate the orientation of the rocket.
Once they are figured out. I will need to combine them as the gyroscope has drift in the data,
to do this I will use a kalman filter.

Once orientation / altitude is established then I will start connecting the RF chip / using the custom protocol.


# 09-01-2026
Okay Got the roll and pitch angles from the accelerometer.
Now I just need to figure out the angles from the gyroscope and then we can use both to figure out true orientation.
Also found a good youtube channel that is explaing everything i need:
https://www.youtube.com/@carbonaeronautics

Later in the day:
Got the gyro calibrated and working properly now. Can almost get orientation from it by calculating the sum of the changes.
Except right now a 90deg rotation on x / y axis only goes to 15. Need to see how to convert the rad/s to deg/s properly as when i just multiply by 57.295779513082
I get even stranger numbers where a 90deg rotation can get to 800

# 09-02-2026
Okay, Was able to figure out how to get the proper degrees from the gyro. I was pretty close!
Now I can see both Pitch / Roll from both gyro and accelerometer. Now I can use a Kalman filter to get
a better appx. To the change