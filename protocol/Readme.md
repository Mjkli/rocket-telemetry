# Binary Telemetry Protocol

`protocol` is a small Rust crate for framing serialized telemetry for UART or other byte-stream transports. It combines `postcard` serialization, a `u16` message count, CRC validation, and COBS encoding.

It is an encoding layer, not a complete networking protocol: it does not provide acknowledgements, retransmission, packet types, or stream buffering.

## Packet format

Before COBS encoding, each packet is laid out as follows:

1. Message count as a big-endian `u16` (2 bytes)
2. Payload serialized with `postcard`
3. CRC-16/IBM-SDLC as a big-endian `u16` (2 bytes)

The CRC covers both the message count and the serialized payload. The complete buffer is then encoded with COBS and returned by `encode`.

COBS removes zero bytes from the encoded data, making the result suitable for framing on a continuous serial stream. The transport is responsible for supplying a complete COBS-encoded packet to `decode`.

## API

```rust
pub fn encode<T>(value: &T, count: u16) -> anyhow::Result<Vec<u8>>
where
    T: serde::Serialize;

pub fn decode<T>(data: &[u8]) -> anyhow::Result<(T, u16)>
where
    T: serde::de::DeserializeOwned;
```

`decode` first COBS-decodes the input, validates the CRC, reads the message count, and then deserializes the payload. Corrupt packets are rejected before deserialization.

## Example

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

let encoded = protocol::encode(&data, 1)?;
let (decoded, count): (Telemetry, u16) = protocol::decode(&encoded)?;

assert_eq!(decoded, data);
assert_eq!(count, 1);
# Ok::<(), anyhow::Error>(())
```

## Tests

The crate includes tests for payload round-tripping and for all message counts from `1` through `u16::MAX`.

Run them from the repository root with:

```text
cargo test --manifest-path protocol/Cargo.toml
```

## Dependencies

- `postcard` with the `alloc` feature for compact serialization
- `cobs` for stream-safe encoding
- `crc` using `CRC_16_IBM_SDLC`
- `serde` for serialization and deserialization traits
- `anyhow` for error handling
- `rand` as a development dependency for randomized tests