# Flight Computer

Rust firmware for the ESP32-S3 rocket flight computer. The current build initializes the onboard sensors over I2C, estimates roll and pitch from the MPU6050, computes pressure-derived altitude from the BMP280, and emits structured telemetry packets over UART for the ground station or radio link.

## Overview

The flight computer is a single ESP32-S3 application that runs a continuous telemetry loop. Each cycle reads IMU and pressure data, calibrates the raw gyro readings using startup bias offsets, applies a simple complementary/Kalman-style filter to estimate orientation, and then serializes telemetry with the protocol crate.

## Hardware and Wiring

The current sensor and telemetry wiring is:

| Function | ESP32-S3 pin |
| --- | --- |
| I2C SDA | GPIO11 |
| I2C SCL | GPIO12 |
| UART RX | GPIO6 |
| UART TX | GPIO5 |

The I2C bus runs at 400 kHz and is shared by:

- MPU6050 accelerometer and gyroscope
- BMP280 temperature and pressure sensor

The BMP280 is configured with 8x oversampling for temperature and pressure. The MPU6050 is calibrated at startup and then used for both gyro propagation and accelerometer correction.

## Implemented Functionality

### Sensor acquisition

- ESP-IDF startup and logging
- Shared I2C bus initialization via `shared-bus`
- MPU6050 accelerometer and gyroscope reads
- BMP280 pressure and temperature reads
- Pressure-to-altitude conversion using the standard atmospheric approximation

### Orientation estimation

The current firmware estimates roll and pitch with a two-step approach:

1. Raw gyro data is corrected using startup bias calibration.
2. The gyro estimate is propagated over time with a basic uncertainty model.
3. Accelerometer roll and pitch are computed from gravity vectors.
4. Their results are blended into the gyro estimate using a Kalman-style update.

This is intentionally a lightweight orientation estimate for now; it is not yet a full flight-state or guidance system.

### Telemetry output

Each telemetry frame contains:

- `roll`
- `pitch`
- `altitude`
- `temperature`
- `flight_state`
- `count`

The data is serialized with the protocol crate using `postcard`, then protected with CRC-16 and encoded with COBS. A `0x00` header byte is prepended before transmission over UART.

## Current Status

Implemented:

- ESP-IDF application startup and logging
- Shared I2C bus initialization
- MPU6050 sensor reads and startup calibration
- BMP280 pressure and temperature reads
- Pressure-based altitude calculation
- Roll and pitch estimation from accelerometer and gyro data
- Telemetry packet encoding and UART transmission

Not implemented yet:

- Real flight-state detection and apogee calculation
- Autonomous parachute deployment logic
- Persistent on-device data logging or black-box storage

## Build

From this directory, build the firmware with:

```sh
cargo build
```

For a release build:

```sh
cargo build --release
```

The project uses the ESP Rust toolchain declared in `rust-toolchain.toml` and the ESP-IDF build system configured in `build.rs` and `Cargo.toml`.

## Flashing and Monitoring

Use the normal ESP-IDF/ESP Rust workflow for your specific board and host environment to flash the firmware and monitor serial output. The application outputs telemetry through the configured UART channel, which can be connected to a radio module or a telemetry receiver.

## Project Files

- `src/main.rs` - Firmware entry point, calibration, sensor loop, and packet emission
- `build.rs` - ESP-IDF build integration
- `Cargo.toml` - Rust dependencies and ESP-IDF configuration
- `rust-toolchain.toml` - ESP Rust toolchain selection
- `../protocol` - Shared telemetry serialization and packet framing crate

## Notes

The telemetry format is intentionally structured for later ground-station use. The current `FlightState` enum supports `Preflight`, `Launch`, `Apogee`, `Descent`, and `Landed`, but the state machine is still a future task.
