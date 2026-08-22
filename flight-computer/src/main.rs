
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::Hertz;



use mpu6050::Mpu6050;
use bmp280_ehal::{BMP280, Control, Oversampling, PowerMode};
use shared_bus::BusManagerSimple;

fn calculate_altitude(p: f64) -> f64 {
    let center = p / 1013.25;
    let alt = 44330.77 * (1.0 - (center.powf(0.190263)));
    (alt * 1000.0).round() / 1000.0
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio11;
    let scl = peripherals.pins.gpio12;




    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c = I2cDriver::new(peripherals.i2c0, sda, scl, &config).unwrap();
    
    let bus = BusManagerSimple::new(i2c);

    let mut mpu = Mpu6050::new(bus.acquire_i2c());
    mpu.init(&mut FreeRtos).unwrap();


    let mut bmp = BMP280::new(bus.acquire_i2c()).unwrap();
    bmp.reset();

    bmp.set_control(Control {
        osrs_t: Oversampling::x8,   // temperature oversampling
        osrs_p: Oversampling::x8,   // pressure oversampling
        mode: PowerMode::Normal,
    });


    loop {
        let accel = mpu.get_acc().unwrap();
        let gyro = mpu.get_gyro().unwrap();
        log::info!("\nAccel: x={:.2} y={:.2} z={:.2}", accel.x, accel.y, accel.z);


        let pressure = bmp.pressure() / 100.0;
        let altitude = calculate_altitude(pressure);
        let bmp_temp = bmp.temp();
        log::info!("\nBMP Temp: {:.2} C\nAltitude: {:?} m", bmp_temp, altitude);


        FreeRtos::delay_ms(100);
    }
}
