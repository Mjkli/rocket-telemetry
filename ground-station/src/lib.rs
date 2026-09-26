use protocol::telemetry_data::{FlightState, TelemetryData};

pub struct PacketParser {
    buffer: Vec<u8>,
}

impl PacketParser {
    pub fn new() -> Self {
        PacketParser {
            buffer: Vec::new(),
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Option<Vec<TelemetryData>> {
        self.buffer.extend(bytes);
        // Check if the buffer contains packet/s 
        

        None
    }


}