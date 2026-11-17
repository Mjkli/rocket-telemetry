
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;



// Custom Ring buffer implementtation for storing bytes received from the serial port.
/*
    Things i want this implementation to be able to do:
    - avoid the reallocation overhead and element shifting
    - a fixed-size array with read and write indices
    - Lock-free implementations rely on atomic operations to update indices safely without blocking threads

    The first two should be easy to implement. Need to figure out how to implement the lock-free part.
*/


// Going to just focus on the basic structure before thinking about concurrency

pub struct DataRingBuffer {
    buffer: Vec<Option<u8>>,
    write_index: usize,
    read_index: usize,
}

impl DataRingBuffer {
    pub fn new(size: usize) -> Self {
        DataRingBuffer {
            buffer: vec![None; size],
            write_index: 0,
            read_index: 0,
        }
    }

    pub fn push(&mut self, data: &[u8]) {

        for byte in data {
            self.buffer[self.write_index] = Some(*byte);
            self.write_index = (self.write_index + 1) % self.buffer.len();
            if self.write_index == self.read_index {
                self.read_index = (self.read_index + 1) % self.buffer.len();
            }
        }
    }

    pub fn get(&self, length: usize) -> Vec<u8> {
        let mut result = Vec::new();
        let mut index = self.read_index;

        for _ in 0..length {
            if let Some(byte) = self.buffer[index] {
                result.push(byte);
            } else {
                break;
            }
            index = (index + 1) % self.buffer.len();
        }

        result
    }

    pub fn to_next_0(&mut self) -> u8 {
        let mut index = self.read_index;
        let mut count = 0;
        while index != self.write_index {
            if self.buffer[index].is_none() {
                break;
            }
            if let Some(byte) = self.buffer[index] {
                if byte == 0 {
                    return count;
                }
            }
            index = (index + 1) % self.buffer.len();
            count += 1;
        }

        count
    } 

    pub fn peek(&self) -> Option<u8> {
        self.buffer[self.read_index]
    }

    pub fn advance_read(&mut self, length: u8) {
        for _ in 0..length {
            self.read_index = (self.read_index + 1) % self.buffer.len();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut ring_buffer = DataRingBuffer::new(95);
        let encoded_packets: Vec<Vec<u8>> = (1..=3)
            .map(|count| {
                let data = protocol::telemetry_data::TelemetryData {
                    roll: 20.0,
                    pitch: 30.0,
                    altitude: 192.0,
                    temperature: 15.0,
                    flight_state: protocol::telemetry_data::FlightState::Preflight,
                    count,
                };
                protocol::encode(&data).unwrap()
            })
            .collect();
        let mut incoming_stream = Vec::new();
        for packet in &encoded_packets {
            incoming_stream.extend_from_slice(packet);
        }

        let mut decoded_packets: Vec<protocol::telemetry_data::TelemetryData> = Vec::new();
        let mut bytes_since_delimiter = 0;

        while !incoming_stream.is_empty() {
            let chunk_size: usize = rand::random_range(1..=5);
            let chunk: Vec<u8> = incoming_stream.drain(0..chunk_size.min(incoming_stream.len())).collect();
            ring_buffer.push(&chunk);

            if chunk.contains(&0) {
                let possible_packet = ring_buffer.get(38);
                assert_eq!(possible_packet.len(), 38);
                let packet = protocol::decode::<protocol::telemetry_data::TelemetryData>(
                    possible_packet.as_slice(),
                )
                .expect("complete encoded packet should decode");
                decoded_packets.push(packet);
            }
        }

        assert_eq!(bytes_since_delimiter, 0, "input ended with an incomplete packet");
        let decoded_counts: Vec<_> = decoded_packets.iter().map(|packet| packet.count).collect();
        assert_eq!(decoded_counts, [1, 2, 3]);
    }
}