# Rocket Telemetry

Rocket telemetry software split across an ESP32-S3 flight computer, a planned ground-station service, a shared telemetry protocol, and a Tauri desktop visualizer.

## Repository Layout

- `flight-computer/` - Rust firmware for the ESP-IDF toolchain. The current firmware reads an MPU6050 accelerometer/gyroscope and a BMP280 pressure/temperature sensor over a shared I2C bus, then logs the readings every 100 ms.
- `ground-station/` - Reserved for the ground-station application that will receive rocket packets and store them locally. Implementation is not present yet.
- `protocol/` - Reserved for the binary telemetry protocol shared by the flight computer and ground station. Implementation is not present yet.
- `desktop-application/flight-viz/` - Tauri 2 desktop application with a React, TypeScript, and Vite frontend. The UI is currently the Tauri starter screen and includes a Rust-backed greeting command.

## Current Status

The flight computer sensor-reading path and the desktop application scaffold are in place. Packet transmission, ground-station storage, the shared binary protocol, and flight-data visualization still need to be implemented.

## Desktop Application

Requirements: Node.js, npm, Rust, and the Tauri prerequisites for your operating system.

```sh
cd desktop-application/flight-viz
npm install
npm run dev
```

To build the frontend and check the TypeScript project:

```sh
npm run build
```

To run the Tauri application in development mode:

```sh
npm run tauri dev
```

## Flight Computer

The firmware uses the ESP-IDF Rust toolchain configured by `flight-computer/rust-toolchain.toml` and the ESP-IDF build integration in `build.rs`. Its current sensor wiring is:

| Signal | ESP pin |
| --- | --- |
| I2C SDA | GPIO11 |
| I2C SCL | GPIO12 |

The I2C bus runs at 400 kHz. The BMP280 uses 8x oversampling for temperature and pressure, and measurements are logged through `EspLogger`.

Build or flash the firmware from the flight-computer directory using the ESP Rust workflow configured for your board:

```sh
cd flight-computer
cargo build
```

## Development Notes

- Keep the protocol definition shared between the flight computer and ground station once packet transport is added.
- The desktop visualizer should consume recorded or live ground-station data rather than sensor values directly.
- Component-specific setup notes are available in `desktop-application/Readme.md`, `ground-station/Readme.md`, and `protocol/Readme.md`.
