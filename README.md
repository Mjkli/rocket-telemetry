# Rocket Telemetry

Rocket telemetry system spanning an ESP32-S3 flight computer, a serial ground station, a shared binary telemetry protocol, and a desktop visualization app.

## Repository Layout

- `flight-computer/` - ESP32-S3 firmware using the ESP-IDF Rust toolchain. The current application initializes the shared I2C bus, reads the MPU6050 and BMP280, calibrates the sensors, estimates roll and pitch, computes altitude from pressure, and transmits telemetry packets over UART every 150 ms.
- `ground-station/` - Rust receiver that listens on a serial port, buffers incoming bytes, parses framed packets, decodes the payload, and prints parsed telemetry values.
- `libraries/protocol/` - Shared binary telemetry library used by both the flight computer and ground station. It serializes `TelemetryData` with `postcard`, validates with CRC-16/IBM-SDLC, and wraps packets with COBS framing and a `0x00` packet delimiter.
- `libraries/websocket/` - WebSocket client/server code for future live data transport and integration with external clients. It is not yet wired into the main telemetry flow.
- `desktop-application/flight-viz/` - Tauri 2 desktop app scaffolded with React, TypeScript, and Vite. It is still a starter UI and is not yet connected to live or recorded telemetry data.
- `notes/` - Design notes and component logs that document the implementation progress and rationale.

## Current Status

Implemented and working:

- Flight computer reads MPU6050 and BMP280 data over a shared I2C bus.
- Startup calibration for gyro and accelerometer bias is performed.
- Roll and pitch are estimated from the fused gyro/accelerometer data.
- Pressure-derived altitude is computed and included in the telemetry packet.
- The shared protocol crate encodes and decodes structured telemetry packets with CRC and COBS framing.
- The ground station receives the UART stream, splits out packets, decodes them, and prints the payload.

Still in progress / future work:

- Real flight-state detection and flight sequencing (`Preflight`, `Launch`, `Apogee`, `Descent`, `Landed`).
- Autonomous parachute or recovery logic.
- Persistent telemetry logging or replay tooling.
- Full desktop dashboard and visualization connected to live ground-station data.
- End-to-end integration between the desktop app and the serial receiver pipeline.

## Flight Computer

The firmware uses the ESP-IDF Rust toolchain configured by `flight-computer/rust-toolchain.toml` and the ESP-IDF build integration in `flight-computer/build.rs`.

Current hardware wiring:

| Signal | ESP pin |
| --- | --- |
| I2C SDA | GPIO11 |
| I2C SCL | GPIO12 |
| UART RX | GPIO6 |
| UART TX | GPIO5 |

The I2C bus runs at 400 kHz and is shared by the MPU6050 and BMP280. The BMP280 is configured with 8x oversampling for both temperature and pressure.

Telemetry fields currently transmitted:

- `roll`
- `pitch`
- `altitude`
- `temperature`
- `flight_state`
- `count`

The firmware emits encoded packets over UART every 150 ms.

Build or flash the firmware from the flight-computer directory:

```sh
cd flight-computer
cargo build
```

For a release build:

```sh
cd flight-computer
cargo build --release
```

## Ground Station

The ground station is a Rust serial receiver that reads from a configured serial port, typically `/dev/serial0`, and reconstructs packet boundaries from the incoming stream.

Current behavior:

- opens the serial port with `serialport`
- reads raw bytes in a loop
- buffers incoming data
- scans for packet frames delimited by `0x00`
- calls `protocol::decode::<TelemetryData>()`
- prints the resulting telemetry values to stdout

Run it from the ground-station directory:

```sh
cd ground-station
cargo run
```

## Shared Protocol

The `libraries/protocol` crate is the shared serialization layer for the whole project. It currently implements:

- `postcard` serialization of the telemetry payload
- CRC-16/IBM-SDLC validation
- COBS encoding for zero-safe serial framing
- a leading `0x00` delimiter used for packet boundaries on the UART stream

The main API is:

```rust
pub fn encode<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: serde::Serialize;

pub fn decode<T>(data: &[u8]) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned;
```

This protocol is used by both the flight computer and ground station.

## Desktop Application

Requirements: Node.js, npm, Rust, and the Tauri prerequisites for your operating system.

```sh
cd desktop-application/flight-viz
npm install
npm run dev
```

Build the frontend and TypeScript app:

```sh
cd desktop-application/flight-viz
npm run build
```

Run the Tauri app in development mode:

```sh
cd desktop-application/flight-viz
npm run tauri dev
```

The current app is still the default scaffold and has not yet been wired to consume the live telemetry stream or replay recorded flight data.

## Development Notes

- Keep the protocol definition shared between the flight computer and ground station.
- The desktop visualizer should consume ground-station data rather than reading sensors directly.
- The ground-station and protocol layers are the integration points for the current real-time telemetry system.
- Component-specific notes and design logs are available in `desktop-application/Readme.md`, `ground-station/Readme.md`, `flight-computer/ReadMe.md`, and the files in `notes/`.
