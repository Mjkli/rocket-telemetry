use protocol::decode;
use protocol::telemetry_data::{TelemetryData};

pub struct PacketParser {
    pub buffer: Vec<u8>,
}

impl PacketParser {
    pub fn new() -> Self {
        const SIZE: usize = 256;

        PacketParser {
            buffer: Vec::with_capacity(SIZE),
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Option<Vec<TelemetryData>> {
        self.buffer.extend_from_slice(bytes);
        self.parse()
    }

    pub fn parse(&mut self) -> Option<Vec<TelemetryData>> {
        let mut packets = Vec::new();
        let mut cursor = 0;

        while cursor < self.buffer.len() {
            let packet_end = match self.buffer[cursor..].iter().position(|&b| b == 0x00) {
                Some(pos) => cursor + pos,
                None => break,
            };

            let candidate = &self.buffer[cursor..=packet_end];

            if candidate.len() == 1 {
                cursor += 1;
                continue;
            }

            match decode::<TelemetryData>(candidate) {
                Ok(packet) => {
                    packets.push(packet);
                    cursor = packet_end + 1;
                }
                Err(e) => {
                    log::error!("Error decoding packet: {:?}", e);
                    cursor += 1;
                }
            }
        }

        if cursor > 0 {
            self.buffer.drain(..cursor);
        }

        if packets.is_empty() {
            None
        } else {
            println!("Parsed {} packets from buffer", packets.len());
            Some(packets)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::telemetry_data::FlightState;

    fn encoded_packet(count: u16) -> Vec<u8> {
        let telemetry_data = TelemetryData {
            roll: 1.25,
            pitch: -2.5,
            altitude: 123.0,
            temperature: 18.5,
            flight_state: FlightState::Preflight,
            count,
        };
        protocol::encode(&telemetry_data).unwrap()
    }

    #[test]
    fn parses_complete_packets_and_preserves_values() {
        let mut parser = PacketParser::new();
        let packet = encoded_packet(7);

        let packets = parser.push(&packet).unwrap();

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].roll, 1.25);
        assert_eq!(packets[0].pitch, -2.5);
        assert_eq!(packets[0].altitude, 123.0);
        assert_eq!(packets[0].temperature, 18.5);
        assert_eq!(packets[0].count, 7);
        assert!(parser.buffer.is_empty());
    }

    #[test]
    fn retains_partial_packet_until_more_bytes_arrive() {
        let mut parser = PacketParser::new();
        let packet = encoded_packet(3);
        let split = 17;

        assert!(parser.push(&packet[..split]).is_none());
        assert_eq!(parser.buffer, packet[..split]);

        let packets = parser.push(&packet[split..]).unwrap();

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].count, 3);
        assert!(parser.buffer.is_empty());
    }

    #[test]
    fn parses_multiple_packets_from_one_chunk() {
        let mut parser = PacketParser::new();
        let mut stream = encoded_packet(1);
        stream.extend(encoded_packet(2));
        let packets = parser.push(&stream).unwrap();

        assert_eq!(packets.iter().map(|packet| packet.count).collect::<Vec<_>>(), [1, 2]);
        assert!(parser.buffer.is_empty());
    }

    #[test]
    fn skips_noise_and_corrupt_frames() {
        let mut parser = PacketParser::new();
        let mut stream = vec![0x42, 0x24, 0x11];
        let mut corrupt_packet = encoded_packet(1);
        corrupt_packet[10] ^= 0x01;
        stream.extend(corrupt_packet);
        stream.extend(encoded_packet(2));
        let packets = parser.push(&stream).unwrap();

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].count, 2);
        assert!(parser.buffer.is_empty());
    }
}