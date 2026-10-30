use protocol::decode;
use protocol::telemetry_data::{TelemetryData};
const PACKETSIZE: usize = 38;

pub struct PacketParser {
    pub buffer: Vec<u8>,
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
        if self.buffer.len() < PACKETSIZE {
            return None; // Not enough data for a packet
        }

        self.parse()
    }

    pub fn parse(&mut self) -> Option<Vec<TelemetryData>> {
        let mut packets = Vec::new();
        let mut cursor = 0;
        loop {
            let packet_start = match self.buffer[cursor..].iter().position(|&b| b == 0x00) {
                Some(pos) => cursor + pos,
                None => {
                    cursor = self.buffer.len();
                    break;
                }
            };

            let packet_end = match self.buffer[packet_start + 1..].iter().position(|&b| b == 0x00) {
                Some(pos) => packet_start + 1 + pos,
                None => {
                    // println!("End not found!");
                    break;
                }
            };
            
            let possible_packet: &[u8] = &self.buffer.as_slice()[packet_start..packet_end];

            match decode::<TelemetryData>(&possible_packet) {
                Ok(packet) => {
                    packets.push(packet);
                }
                Err(e) => {
                    println!(
                        "Invalid frame at offset {}: {:?}",
                        packet_start,
                        e
                    );

                }
                // Search for the next possible frame header.
            }
            cursor = packet_end;
        }

        self.buffer.drain(..cursor);

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