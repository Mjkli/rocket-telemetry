
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
            // let write_index = self.write_index.load(Ordering::Relaxed);
            self.buffer[self.write_index] = *byte;
            self.write_index = (self.write_index + 1) % self.buffer.len();
            // self.write_index.store((write_index + 1) % self.buffer.len(), Ordering::Relaxed);
        }
    }

    pub fn get(&self, slice_size: usize) -> Vec<T> 
    where
        T: Copy,
    {
        if slice_size > (self.buffer.len() - self.read_index) {
            let res: Vec<T> = self.buffer[self.read_index..].to_vec();
            let remaining_size = slice_size - res.len();
            let mut result = res;
            result.extend_from_slice(&self.buffer[0..remaining_size]);
            result
        } else {
            (&self.buffer[self.read_index..(self.read_index + slice_size)]).to_vec()
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut ring_buffer = RingBuffer::new(256, 0);
        let telemetry_data = protocol::telemetry_data::TelemetryData {
            roll: 1.25,
            pitch: -2.5,
            altitude: 123.0,
            temperature: 18.5,
            flight_state: protocol::telemetry_data::FlightState::Preflight,
            count: 7,
        };
        let uart_data = protocol::encode(&telemetry_data).unwrap();

        ring_buffer.push(&uart_data);

        assert_eq!(ring_buffer.get(uart_data.len()), uart_data);
        assert_eq!(ring_buffer.get(uart_data.len()).last(), Some(&0x00));

    }
}