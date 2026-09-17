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
a better appx.

# 09-13-2026
Starting on the Kalman filter now for orientation. Had to swap the mean bias for the gyro to varience calculation.

# 09-15-2026
Okay got the Kalman filter implemented, crudely... But will take it now. Will probably clean it up later,
If i stare at it for a bit. I think now I can start creating the protocol and start connecting to the ground station.

# 09-17-2026
Taking a look at the mpu6050 docs from the lib i am using. There is a get_acc_angels function. I tested this against the calculated kalman filter and found that it was more noisy. I am confident that using the kalman filter angles are better.