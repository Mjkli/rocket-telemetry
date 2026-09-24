# Binary Telemetry Protocol

A lightweight binary framing library for serial communication between the flight computer and the ground station. This crate is not a full networking protocol; it is a compact encoding layer for transmitting Rust data over UART / serial links.

## What it does

The crate takes any `serde::Serialize` payload and converts it into a packet that is:

- compact and portable via `postcard`
- safe for UART streams using COBS encoding
- protected from corruption using a CRC16 checksum
- easy to decode back into the original type with `serde::Deserialize`

This is a good fit for embedded telemetry where a continuous byte stream needs an unambiguous way to detect packet boundaries.

## Packet format

Each payload is encoded in this order:

1. Serialize the struct with `postcard`
2. Append a CRC-16/IBM-SDLC checksum as 2 bytes
3. Run the result through COBS encoding before transmission

The final transmitted payload is the COBS-encoded buffer.

In practical terms:

- `postcard` handles compact binary serialization
- `crc` validates payload integrity
- `cobs` helps packet boundary detection in a continuous serial stream

## Why COBS

UART and serial interfaces are byte-oriented and do not provide packet framing by themselves. A raw binary payload may contain zero bytes (`0x00`), which makes it hard to know where one message ends and the next begins.

COBS avoids that by encoding zero bytes out of the data stream and replacing them with index markers. This makes packet recovery from a continuous stream much easier without storing a separate length prefix for every message.

## CRC validation

The crate currently uses the `CRC_16_IBM_SDLC` polynomial and validates the message on decode. The process is:

- calculate the checksum of the serialized payload
- append the 2-byte CRC to the payload
- decode the COBS data at the receiver
- verify that the received CRC matches the recalculated CRC
- reject corrupt packets before deserializing

## API

```rust
use protocol::{encode, decode};

let payload = encode(&my_struct)?;
let decoded: MyStruct = decode(&payload)?;
```

The public functions are:

- `encode<T>(&T) -> Result<Vec<u8>>`
- `decode<T>(&[u8]) -> Result<T>`

The type `T` must implement `serde::Serialize` and `serde::Deserialize`.

## Example

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Telemetry {
    altitude: f64,
    temperature: f64,
    speed: f64,
}

let data = Telemetry {
    altitude: 123.4,
    temperature: 21.7,
    speed: 18.2,
};

let encoded = protocol::encode(&data).unwrap();
let decoded: Telemetry = protocol::decode(&encoded).unwrap();

assert!((decoded.altitude - data.altitude).abs() < f64::EPSILON);
```

## Current usage in the project

The crate is currently used to serialize the telemetry payload from the flight computer, including values such as roll, pitch, altitude, temperature, and flight state. The packet format is designed to be sent over a UART connection between the flight computer and the ground-station / desktop application.

## Future direction

This crate is intentionally simple right now, but the architecture leaves room for additional metadata such as:

- message sequence numbers
- message type / packet ID tags
- acknowledgements or retransmission tracking
- end-to-end validation for lost packets

Those improvements can be layered on top of the current framing format without changing the core binary encoding idea.

## Dependencies

- `postcard`: compact binary serialization
- `cobs`: COBS encoding and decoding
- `crc`: CRC calculation
- `serde`: serialization and deserialization traits
- `anyhow`: error handling

## Summary

This crate provides a robust and compact binary encoding layer for serial telemetry. It is not meant to be a full protocol stack yet; it is a practical wire format for reliable, stream-friendly data transmission between embedded and host systems.