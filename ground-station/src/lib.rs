use anyhow::Result;
use protocol::decode;
use protocol::telemetry_data::{FlightState, TelemetryData};
use std::collections::VecDeque;
use log::error;

pub struct PacketParser {
    buffer: Vec<u8>,
}

impl PacketParser {
    pub fn new() -> Self {

        const SIZE: usize = 256; // Packet size is 38, so this should be enough to hold multiple packets

        PacketParser {
            buffer: Vec::with_capacity(SIZE),
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Option<Vec<TelemetryData>> {
        self.buffer.extend(bytes);
        // Check if the buffer contains enough data for at least one packet (38 bytes)
        if self.buffer.len() < 38 {
            return None; // Not enough data for a packet
        }

        self.parse()
    }

    pub fn parse(&mut self) -> Option<Vec<TelemetryData>> {
        let mut packets = Vec::new();
        // Iterate through the buffer to find packets. when we find a 0x00 byte, we check if there are enough bytes after it to form a complete packet. 
        // If so, we attempt to decode it. If decoding is successful, we add the packet to the list and remove the bytes from the buffer.
        // If not, we log an error and continue searching for the next packet.
        while let Some(i) = self.buffer.iter().position(|&byte| byte == 0x00) {
            // need to have this check here so that we dont try to decode a packet that is not complete yet. 
            // If we dont have enough bytes for a full packet, we break out of the loop and wait for more data to be pushed into the buffer.
            if self.buffer.len() < i + 38 {
                break; // Not enough data for a full packet
            }

            let possible_packet: &[u8] = &self.buffer.as_slice()[i..(i + 38)];

            match decode::<TelemetryData>(&possible_packet) {
                Ok(packet) => {
                    packets.push(packet);
                    self.buffer.drain(0..(i + 38));
                }
                Err(e) => {
                    log::error!("Error decoding packet: {:?}", e);
                }
            }

        }

        if packets.is_empty() {
            None
        } else {
            Some(packets)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rng, Rng};
    use rand::distr::Uniform;
    use rand::distr::Distribution;
    use rand::RngExt;

    fn encode_telemetry_data() -> Vec<u8> {
        let stream: Vec<u8> = (1..11)
            .map(|i| {
                let telemetry_data = TelemetryData {
                    roll: rand::random::<f64>(),
                    pitch: rand::random::<f64>(),
                    altitude: rand::random::<f64>(),
                    temperature: rand::random::<f64>(),
                    flight_state: FlightState::Preflight,
                    count: i as u16,
                };
                protocol::encode(&telemetry_data).unwrap()
            })
            .flatten()
            .collect();
        stream
    }

    #[test]
    fn test_packet_parser() {
        let mut stream = encode_telemetry_data();

        let mut parser = PacketParser::new();
        let mut packets: Vec<TelemetryData> = Vec::new();
        while !stream.is_empty() {
            // Want to randomly chunk the stream into smaller pieces to simulate real-world data transmission
            let mut chunk_size: usize = rng().random_range(0..=20);
            if chunk_size > stream.len() {
                chunk_size = stream.len();
            }
            let chunk: Vec<u8> = stream.drain(0..chunk_size).collect();
            if let Some(new_packets) = parser.push(&chunk) {
                packets.extend(new_packets);
            }
        }
        println!("Packets received: {:?}", packets);
        assert_eq!(packets.len(), 10);
    }        

}