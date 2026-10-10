use protocol::telemetry_data::{FlightState, TelemetryData};
use std::collections::VecDeque;

pub struct PacketParser {
    buffer: VecDeque<u8>,
}

impl PacketParser {
    pub fn new() -> Self {

        const SIZE: usize = 256; // Packet size is 36, so this should be enough to hold multiple packets

        PacketParser {
            buffer: VecDeque::with_capacity(SIZE),
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Option<Vec<TelemetryData>> {
        self.buffer.extend(bytes);
        // Check if the buffer contains packet/s
        println!("Buffer : {:?}", self.buffer);
        

        None
    }

    pub fn parse(&mut self) -> Option<Vec<TelemetryData>> {
        // Implement packet parsing logic here
        // For example, you can look for packet delimiters and extract packets from the buffer
        None
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
                };
                protocol::encode(&telemetry_data, i).unwrap()
            })
            .flatten()
            .collect();
        stream
    }

    #[test]
    fn test_packet_parser() {
        let mut stream = encode_telemetry_data();

        let mut parser = PacketParser::new();
        while !stream.is_empty() {
            let mut chunk_size: usize = rng().random_range(1..=20);
            if chunk_size > stream.len() {
                chunk_size = stream.len();
            }
            let chunk: Vec<u8> = stream.drain(0..chunk_size).collect();
            parser.push(&chunk);
        } 
    
    
    
    }        

}