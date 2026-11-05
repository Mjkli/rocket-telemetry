use anyhow::Result;
use serialport::SerialPort;
use crate::parcer::parcer::PacketParser;
use tokio::sync::mpsc::Sender;
use websocket_server::OutboundData;
use postcard;
use protocol::telemetry_data::TelemetryData;

pub fn buf_reader(serial_port: &str, stream_tx: Sender<OutboundData>) -> Result<()> {
 let mut port = serialport::new(serial_port, 9600)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;
    
    let mut buffer = [0u8; 120];
    // let mut parser = PacketParser::new();
    let mut buff = Vec::with_capacity(256);
    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                buff.extend_from_slice(&buffer[..n]);
                println!("Buffer : {:02X?}", buff);

                // while let Some(end) = buff.iter().position(|&b| b == 0) {
                //     // Include the trailing delimiter
                //     let frame: Vec<u8> = buff.drain(..=end).collect();

                //     if frame.len() <= 1 {
                //         continue;
                //     }

                //     println!("RX frame length: {}", frame.len());
                //     println!("RX frame: {:02X?}", frame);

                //     match protocol::decode::<TelemetryData>(&frame) {
                //         Ok(data) => println!("Decoded telemetry: {:?}", data),
                //         Err(e) => println!("Decode error: {:?}", e),
                //     }
                // } 


            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // no data yet. keep waiting
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(())
}