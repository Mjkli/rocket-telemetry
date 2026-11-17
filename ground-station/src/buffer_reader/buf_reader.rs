use anyhow::Result;
use serialport::SerialPort;
use crate::parcer::parcer::PacketParser;
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


    let mut inbound = [0u8; 256];
    let mut parser = PacketParser::new();
    let mut loop_speed = 5;

    let mut count: Option<u16> = None;
    let mut caught: u64 = 0;
    let mut missed: u64 = 0;

    let mut test_vec: Vec<u8> = Vec::new();

    let mut buffer = DataRingBuffer::new(256);

    loop {
        match port.read(&mut inbound) {
            Ok(n) => {
                buffer.push(&inbound[..n]);
                if inbound[..n].contains(&0) {
                    if let Some(num) = buffer.peek() {
                        if num == 0 {
                            // we are on 0 here
                            let possible_packet = buffer.get(40);
                            let packet = match decode::<TelemetryData>(&possible_packet) {
                                Ok(packet) => packet,
                                Err(e) => {
                                    error!("Error decoding packet: {:?}", e);
                                    println!("Error decoding packet: {:?} :: {:?}", e, possible_packet);
                                    continue;
                                }
                            };

                            println!("Packet :: {:?}", packet);
                        } else {
                            let len_to_0 = buffer.to_next_0();
                            buffer.advance_read(len_to_0);
                            println!("Advancing buffer");
                        }
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