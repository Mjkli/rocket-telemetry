
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

pub struct RingBuffer<T> {
    buffer: Vec<T>,
    read_index: usize,
    write_index: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(size: usize, default: T) -> Self 
    where
        T: Copy,
    {
        RingBuffer {
            buffer: vec![default; size],
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn push(&mut self, data: &[T]) 
    where
        T: Copy,
    {
        for byte in data {
            if self.write_index == self.read_index {
                self.read_index = (self.read_index + 1) % self.buffer.len();
            }
            self.buffer[self.write_index] = *byte;
            self.write_index = (self.write_index + 1) % self.buffer.len();
        }
    }

    pub fn get(&self, slice_size: usize) -> Vec<T> 
    where
        T: Copy,
    {
        let mut result = Vec::with_capacity(slice_size);
        let mut index = self.read_index;

        for _ in 0..slice_size {
            result.push(self.buffer[index]);
            index = (index + 1) % self.buffer.len();
        }

        result
    }
    
}

mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut ring_buffer = RingBuffer::new(95, 0);
        
        let data = protocol::telemetry_data::TelemetryData {
            roll: 20.0,
            pitch: 30.0,
            altitude: 192.0 ,
            temperature: 15.0,
            flight_state: protocol::telemetry_data::FlightState::Preflight, // Staying in Preflight for now, will implement state machine later
            count: 1,
        };

        let encoded_data = protocol::encode(&data).unwrap();
        ring_buffer.push(&encoded_data);
        ring_buffer.push(&encoded_data); 
        ring_buffer.push(&encoded_data); 
        ring_buffer.push(&encoded_data); 

        let result = ring_buffer.get(encoded_data.len());
        assert_eq!(result, encoded_data);


    }
}