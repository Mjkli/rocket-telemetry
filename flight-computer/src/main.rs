
use std::collections::VecDeque;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::{PinDriver};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver, I2c};
use esp_idf_hal::units::Hertz;

use mpu6050::Mpu6050;
use mpu6050::device::ACCEL_HPF;
use bmp280_ehal::{BMP280, Control, Oversampling, PowerMode};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use shared_bus::BusManagerSimple;

use nalgebra::{Vector3};


const TIME_RATE: u32 = 100;


fn calculate_altitude(p: f64) -> f64 {
    let center = p / 1013.25;
    let alt = 44330.77 * (1.0 - (center.powf(0.190263)));
    (alt * 1000.0).round() / 1000.0
}


fn g_to_mpss(acc_vec: Vector3<f32>) -> Vec<f32> {
    let mut mpss_vec: Vec<f32> = Vec::new();
    mpss_vec.push(acc_vec.x * 9.80665);
    mpss_vec.push(acc_vec.y * 9.80665);
    mpss_vec.push(acc_vec.z * 9.80665);
    
    mpss_vec
}

fn calibrate_accel<I, E>(mpu: &mut Mpu6050<I>, samples: usize) -> Vector3<f32>
where
    I: Write<Error = E> + WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    let mut sum = Vector3::new(0.0f32, 0.0, 0.0);
    for _ in 0..samples {
        let a = mpu.get_acc().unwrap();
        sum += a;
        FreeRtos::delay_ms(10);
    }
    let avg = sum / samples as f32;
    Vector3::new(avg.x, avg.y, avg.z)
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

    let mut mpu = Mpu6050::new(bus.acquire_i2c());
    mpu.init(&mut FreeRtos).unwrap();
    log::info!("Calibrating, keep sensor still...");
    let bias = calibrate_accel(&mut mpu, 2000); // ~2 seconds at 10ms delay
    log::info!("Bias: {:?}", bias);
    red_led.set_low().unwrap();
    green_led.set_high().unwrap();


    let mut bmp = BMP280::new(bus.acquire_i2c()).unwrap();
    bmp.reset();

    bmp.set_control(Control {
        osrs_t: Oversampling::x8,   // temperature oversampling
        osrs_p: Oversampling::x8,   // pressure oversampling
        mode: PowerMode::Normal,
    });

    loop {
        let raw_gyro = mpu.get_gyro().unwrap(); 
        log::info!("\nGyro: {:?}", raw_gyro);
        
        
        let raw = mpu.get_acc().unwrap();
        let corrected = raw - bias;
        let accel = g_to_mpss(corrected);


        
        let pressure = bmp.pressure() / 100.0;
        let altitude = calculate_altitude(pressure);
        let bmp_temp = bmp.temp();
        // log::info!("\nBMP Temp: {:.2} C\nAltitude: {:?} m", bmp_temp, altitude);

        FreeRtos::delay_ms(TIME_RATE);
    }
}
