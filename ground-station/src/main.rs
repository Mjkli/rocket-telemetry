use anyhow::Result;
use protocol::decode;
use protocol::telemetry_data::{FlightState, TelemetryData};

fn main() -> Result<()>{
    let mut port = serialport::new("/dev/serial0", 9600)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;
    
    let mut buffer = [0u8; 256];

    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                println!("Received {} bytes: {:02X?}", n, &buffer[..n]);
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            }
        }
    }
    

    Ok(())
}
