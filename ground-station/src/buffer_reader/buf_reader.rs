use anyhow::Result;
use serialport::SerialPort;
use crate::buffer::buffer::DataRingBuffer;
use tokio::sync::mpsc::Sender;
use protocol::decode;
use protocol::telemetry_data::{TelemetryData};
use log::error;


pub fn buf_reader<T>(serial_port: &str, main_tx: Sender<T>) -> Result<()> {
    let mut port = serialport::new(serial_port, 9600)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .flow_control(serialport::FlowControl::None)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;


    let mut inbound = [0u8; 39];
    let mut buffer = DataRingBuffer::new(256);

    loop {
        match port.read(&mut inbound) {
            Ok(n) => {
                buffer.push(&inbound[..n]);
                while let Some(frame) = buffer.pop_frame() {
                    match decode::<TelemetryData>(&frame) {
                        Ok(packet) => println!("Packet :: {:?}", packet),
                        Err(e) => eprintln!("Error decoding packet: {e:?}")
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                // no data yet. keep waiting
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}