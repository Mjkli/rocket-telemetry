# 08-31-2026
Starting this document. I have setup the arduino to collect Acceleration / gyro data from the MPU6050, 
and Pressure data from the BMP280. I am able to derive the elevation in meters currently.
I want to start looking into the mathmatical formulas needed to calculate the orientation of the rocket.
Once they are figured out. I will need to combine them as the gyroscope has drift in the data,
to do this I will use a kalman filter.

Once orientation / altitude is established then I will start connecting the RF chip / using the custom protocol.