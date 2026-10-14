# Binary Telemetry Protocol

`protocol` is a small Rust crate for framing serialized telemetry for UART or similar byte-stream transports. It combines `postcard` serialization, CRC-16 validation, and COBS encoding to turn structured telemetry into a stream-safe packet format.

It is an encoding layer, not a full networking protocol: it does not provide acknowledgements, retransmission, packet types, or a full stream-state machine.

## Current packet format

The current implementation is intentionally simple and matches the code in `src/lib.rs`:

1. Serialize the Rust value with `postcard`
2. Append a CRC-16/IBM-SDLC checksum as the final 2 bytes
3. Encode the combined payload and CRC with COBS
4. Prepend a `0x00` header byte to mark the start of a packet on the serial stream

The CRC covers the serialized payload only. The CRC bytes are appended in big-endian order before COBS encoding.

This means the frame layout is effectively:

```text
[0x00][COBS(postcard(payload) + CRC16_BIG_ENDIAN)]
```

The `0x00` byte is used as a framing delimiter for the receiving side, while COBS avoids embedded zero bytes in the encoded packet body.

## API

The actual API in the current code is:

```rust
pub fn encode<T>(value: &T) -> anyhow::Result<Vec<u8>>
where
    T: serde::Serialize;

pub fn decode<T>(data: &[u8]) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned;
```

`decode` performs the following steps:

1. COBS-decodes the input packet
2. Validates the trailing CRC-16 value
3. Removes the CRC bytes
4. Deserializes the remaining payload with `postcard`

Corrupt packets are rejected before deserialization.

## Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Telemetry {
    roll: f64,
    pitch: f64,
    altitude: f64,
    temperature: f64,
}

let data = Telemetry {
    roll: 12.5,
    pitch: 4.1,
    altitude: 123.4,
    temperature: 21.7,
};

let encoded = protocol::encode(&data)?;
let decoded: Telemetry = protocol::decode(&encoded)?;

assert_eq!(decoded, data);
# Ok::<(), anyhow::Error>(())
```

## Telemetry data model

The project uses a shared `TelemetryData` struct, which currently looks like this:

```rust
pub struct TelemetryData {
    pub roll: f64,
    pub pitch: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub flight_state: FlightState,
    pub count: u16,
}
```

The `count` field is part of the serialized telemetry object, not a separate protocol-level header. Each message is just a structured payload with CRC protection and COBS framing.

## Tests

The crate includes tests for round-tripping payloads and for verifying the packet size stays within a practical serial-safe bound.

Run them from the repository root with:

```sh
cargo test --manifest-path protocol/Cargo.toml
```

## Dependencies

- `postcard` for compact Rust serialization
- `cobs` for zero-safe serial framing
- `crc` with `CRC_16_IBM_SDLC`
- `serde` for the serialization traits
- `anyhow` for error handling
- `rand` as a development dependency for randomized tests