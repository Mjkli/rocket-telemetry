# Ground Station

Rust application that receives telemetry packets from the rocket over a serial connection, decodes them, and prints the parsed flight data from the onboard flight computer.

## Overview

The ground station is a small Linux-side receiver that listens on `/dev/serial0` at 9600 baud, buffers incoming bytes, and reconstructs telemetry packets from the flight computer. It uses the shared `protocol` crate to decode the COBS-encoded and CRC-protected payload, then prints the parsed telemetry fields.

## Current Behavior

The current implementation does the following:

- opens a serial port with `serialport`
- reads raw bytes in a loop
- feeds those bytes into `PacketParser`
- detects packet boundaries using the `0x00` delimiter
- decodes each packet as `TelemetryData`
- prints the received `roll` and `pitch` values to stdout

This is a receiver/decoder layer, not yet a full dashboard or database-backed application.

## Packet Parsing

The parser in `src/lib.rs`:

- accumulates a rolling buffer of incoming bytes
- waits until it has enough data for a packet size threshold
- scans for framed packet boundaries
- calls `protocol::decode::<TelemetryData>()`
- returns one or more decoded telemetry packets when available

The packet format is shared with the flight computer and includes:

- `roll`
- `pitch`
- `altitude`
- `temperature`
- `flight_state`
- `count`

## Build and Run

From this directory:

```sh
cargo run
```

The program expects the rocket telemetry stream to be connected to `/dev/serial0`.

## Notes

The ground station currently acts as a telemetry receiver and parser. It does not yet implement:

- persistent database storage
- a GUI dashboard
- signal quality monitoring
- packet replay or archive storage
- radio/USB abstraction beyond the serial path

## Project Files

- `src/main.rs` - serial receiver loop and console output
- `src/lib.rs` - packet buffering and decode logic
- `Cargo.toml` - Rust dependencies and project config
- `../protocol` - shared telemetry encoding/decoding format