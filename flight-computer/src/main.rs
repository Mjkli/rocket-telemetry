
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::Hertz;



use mpu6050::Mpu6050;
use bmp280_ehal::{BMP280, Control, Oversampling, PowerMode};
use shared_bus::BusManagerSimple;


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
        log::info!("Accel: x={:.2} y={:.2} z={:.2}", accel.x, accel.y, accel.z);


        let pressure = bmp.pressure() / 100.0;
        let bmp_temp = bmp.temp();
        let bmp_tempF = (bmp_temp.clone() * 1.8) + 32.0; 
        log::info!("BMP Temp: {:.2} C\n{:.2} F,\nPressure: {:.2} hPa", bmp_temp, bmp_tempF, pressure);




        FreeRtos::delay_ms(100);


    }
}
