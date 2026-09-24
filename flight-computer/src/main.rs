
use std::collections::VecDeque;
use std::time::Instant;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::{PinDriver};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver, I2c};
use esp_idf_hal::units::Hertz;
use esp_idf_hal::uart::{UartDriver, config::Config};

use mpu6050::Mpu6050;
use mpu6050::device::ACCEL_HPF;
use bmp280_ehal::{BMP280, Control, Oversampling, PowerMode};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use shared_bus::BusManagerSimple;

use nalgebra::{Vector3};
use libm::{atan2, sqrt};

const TIME_RATE: u32 = 100;


fn calculate_altitude(p: f64) -> f64 {
    let center = p / 1013.25;
    let alt = 44330.77 * (1.0 - (center.powf(0.190263)));
    (alt * 1000.0).round() / 1000.0
}

fn get_roll_angle(acc_vec: Vector3<f32>) -> f64 {
    atan2(acc_vec.y as f64, sqrt((acc_vec.x as f64).powi(2) + (acc_vec.z as f64).powi(2)))
}

fn get_pitch_angle(acc_vec: Vector3<f32>) -> f64 {
    atan2(-acc_vec.x as f64, sqrt((acc_vec.y as f64).powi(2) + (acc_vec.z as f64).powi(2)))
}


fn calibrate_accel<I, E>(mpu: &mut Mpu6050<I>, samples: usize) -> (Vector3<f32>, Vector3<f32>)
where
    I: Write<Error = E> + WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    let mut sum = Vector3::new(0.0f32, 0.0, 0.0);
    let mut sample_data: Vec<Vector3<f32>> = Vec::with_capacity(samples);
    let mut variance_sum = Vector3::new(0.0f32, 0.0, 0.0);
    for _ in 0..samples {
        let a = mpu.get_acc().unwrap();
        sample_data.push(a);
        sum += a;
        FreeRtos::delay_ms(1);
    }
    let mean = sum / samples as f32;

    let var_sum = sample_data.iter().fold(Vector3::new(0.0f32, 0.0, 0.0), |acc, &sample| {
        let diff = sample - mean;
        acc + Vector3::new(diff.x.powi(2), diff.y.powi(2), diff.z.powi(2))
    });

   ( mean, var_sum / (samples - 1) as f32)

}

fn calibrate_accel_angles<I, E>(mpu: &mut Mpu6050<I>, samples: usize) -> (f64, f64)
where
    I: Write<Error = E> + WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    let mut roll_samples = Vec::with_capacity(samples);
    let mut pitch_samples = Vec::with_capacity(samples);

    for _ in 0..samples {
        let a = mpu.get_acc().unwrap();
        roll_samples.push(get_roll_angle(a));
        pitch_samples.push(get_pitch_angle(a));
        FreeRtos::delay_ms(1);
    }
    let roll_mean = roll_samples.iter().sum::<f64>() / samples as f64;
    let pitch_mean = pitch_samples.iter().sum::<f64>() / samples as f64;

    let roll_variance = roll_samples
        .iter()
        .map(|&x| (x - roll_mean).powi(2))
        .sum::<f64>()
        / (samples - 1) as f64;
    
    let pitch_variance = pitch_samples
        .iter()
        .map(|&x| (x - pitch_mean).powi(2))
        .sum::<f64>()
        / (samples - 1) as f64;

    (roll_variance, pitch_variance)
}

fn calibrate_gyro<I, E>(mpu: &mut Mpu6050<I>, samples: usize) -> (Vector3<f32>, Vector3<f32>)
where
    I: Write<Error = E> + WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    let mut sum = Vector3::new(0.0f32, 0.0, 0.0);
    let mut sample_data: Vec<Vector3<f32>> = Vec::with_capacity(samples);
    let mut variance_sum = Vector3::new(0.0f32, 0.0, 0.0);
    for _ in 0..samples {
        let a = mpu.get_gyro().unwrap();
        sample_data.push(a);
        sum += a;
        FreeRtos::delay_ms(1);
    }
    let mean = sum / samples as f32;

    let var_sum = sample_data.iter().fold(Vector3::new(0.0f32, 0.0, 0.0), |acc, &sample| {
        let diff = sample - mean;
        acc + Vector3::new(diff.x.powi(2), diff.y.powi(2), diff.z.powi(2))
    });

    (mean, var_sum / (samples - 1) as f32)
}


struct SensorData {
    roll: f64,
    pitch: f64,
    altitude: f64,
    temperature: f64,
}


fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mut red_led = PinDriver::output(peripherals.pins.gpio0).unwrap();
    red_led.set_high().unwrap();
    let mut green_led = PinDriver::output(peripherals.pins.gpio46).unwrap();

    let sda = peripherals.pins.gpio11;
    let scl = peripherals.pins.gpio12;


    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    let bus = BusManagerSimple::new(i2c);

    let config = Config::new().baudrate(Hertz(9600));

    let uart = UartDriver::new(
        peripherals.uart1,
        peripherals.pins.gpio6, // RX
        peripherals.pins.gpio5, // TX
        Option::<esp_idf_hal::gpio::AnyIOPin>::None, // RTS
        Option::<esp_idf_hal::gpio::AnyIOPin>::None, // CTS
        &config,
    ).unwrap();



    let mut mpu = Mpu6050::new(bus.acquire_i2c());
    mpu.init(&mut FreeRtos).unwrap();
    log::info!("Calibrating, keep sensor still...");
    let (gyro_bias, gyro_v) = calibrate_gyro(&mut mpu, 2000); // ~2 seconds at 1ms delay
    let (accel_bias, accel_v) = calibrate_accel(&mut mpu, 2000); // ~2 seconds at 1ms delay
    let (R_roll, R_pitch) = calibrate_accel_angles(&mut mpu, 2000); // ~2 seconds at 1ms delay


    red_led.set_low().unwrap();
    green_led.set_high().unwrap();


    let mut bmp = BMP280::new(bus.acquire_i2c()).unwrap();
    bmp.reset();

    bmp.set_control(Control {
        osrs_t: Oversampling::x8,   // temperature oversampling
        osrs_p: Oversampling::x8,   // pressure oversampling
        mode: PowerMode::Normal,
    });

    let mut previous = Instant::now();

    // Kalman filter state
    let mut roll_angle = 0.0f64;
    let mut pitch_angle = 0.0f64;
    
    let mut P_roll = 1.0f64; // our sensor uncertainty
    let mut P_pitch = 1.0f64;
    let Q_gyro = gyro_v; // sensor variance

    loop {
        let raw_gyro = mpu.get_gyro().unwrap();
        let corrected_gyro = Vector3::new(
            raw_gyro.x - gyro_bias.x,
            raw_gyro.y - gyro_bias.y,
            raw_gyro.z - gyro_bias.z,
        );

        
        let now = Instant::now();
        let dt = (now - previous).as_secs_f32();
        previous = now;

        // Predicted Angles
        roll_angle += (corrected_gyro.x as f64) * (dt as f64);
        pitch_angle += (corrected_gyro.y as f64) * (dt as f64);

        // Update the uncertainty
        P_roll += (Q_gyro.x as f64) * (dt as f64).powi(2);
        P_pitch += (Q_gyro.y as f64) * (dt as f64).powi(2);

        let raw = mpu.get_acc().unwrap();
        let roll = get_roll_angle(raw);
        let pitch = get_pitch_angle(raw);


        // Measurement Update
        let K_roll = P_roll / (P_roll + R_roll);
        let K_pitch = P_pitch / (P_pitch + R_pitch);
        roll_angle += K_roll * (roll - roll_angle);
        pitch_angle += K_pitch * (pitch - pitch_angle);


        P_roll = (1.0 - K_roll) * P_roll;
        P_pitch = (1.0 - K_pitch) * P_pitch;

    
        let pressure = bmp.pressure() / 100.0;
        let altitude = calculate_altitude(pressure);
        let bmp_temp = bmp.temp();






        // Log the data
        let sensor_data = SensorData {
            roll: roll_angle.to_degrees(),
            pitch: pitch_angle.to_degrees(),
            altitude,
            temperature: bmp_temp,
        };
        // log::info!("Roll: {:.2}, Pitch: {:.2}, Altitude: {:.2} m, Temperature: {:.2} °C", sensor_data.roll, sensor_data.pitch, sensor_data.altitude, sensor_data.temperature);

        uart.write("Hello from ESP32!\n".as_bytes()).unwrap();
        FreeRtos::delay_ms(TIME_RATE);
    }
}
