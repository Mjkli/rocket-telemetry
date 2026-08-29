# Flight Computer

Rust firmware for the ESP32-S3 rocket flight computer. The current firmware initializes the onboard sensors over I2C and logs acceleration, gyroscope, temperature, pressure, and calculated altitude readings.

## Hardware

The current sensor wiring is:

| Signal | ESP32-S3 pin |
| --- | --- |
| I2C SDA | GPIO11 |
| I2C SCL | GPIO12 |

The I2C bus runs at 400 kHz and is shared by:

- MPU6050 accelerometer and gyroscope
- BMP280 temperature and pressure sensor

The BMP280 is configured with 8x oversampling for temperature and pressure. Sensor readings are logged every 100 ms through `EspLogger`.

## Prerequisites

- ESP32-S3 development board
- Rust with the ESP-IDF toolchain
- ESP-IDF build prerequisites for the host operating system
- MPU6050 and BMP280 connected to the I2C pins above

The required Rust toolchain is declared in `rust-toolchain.toml`:

```text
esp
```

## Build

From this directory, build the firmware with:

```sh
cargo build
```

To build an optimized release binary:

```sh
cargo build --release
```

Use the ESP Rust flashing and monitor workflow appropriate for the connected board to flash the resulting binary and view logs.

## Current Status

Implemented:

- ESP-IDF application startup and logging
- Shared I2C bus initialization
- MPU6050 accelerometer and gyroscope readings
- BMP280 temperature and pressure readings
- Pressure-based altitude calculation

Not implemented yet:

- Packet encoding and radio or serial transmission
- Ground-station communication
- Persistent flight-data storage
- Flight-state detection and control logic
    - Logic to calculate appoge
    - Once this is calculated we can figure out forced parachute deployment

## Project Files

- `src/main.rs` - Firmware entry point and sensor loop
- `build.rs` - ESP-IDF build integration
- `Cargo.toml` - Rust dependencies and ESP-IDF configuration
- `rust-toolchain.toml` - ESP Rust toolchain selection



### Notes
08/28/26
    Right now i am at a point that I can get calibrated data from the mpu6050. 
    But Right now I think I need to create a kalman filter so that i can get orientaiton data.
    After I have orientation data. Then I can calculate Traversal distances appx. 
