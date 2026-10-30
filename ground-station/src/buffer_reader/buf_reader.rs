use anyhow::Result;
use serialport::SerialPort;
use crate::parcer::parcer::PacketParser;
use tokio::sync::mpsc::Sender;
use websocket_server::OutboundData;
use postcard;
use log::error;

pub fn buf_reader(serial_port: &str, stream_tx: Sender<OutboundData>) -> Result<()> {
 let mut port = serialport::new(serial_port, 9600)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;
    
    let mut buffer = [0u8; 256];
    let mut parser = PacketParser::new();
    let mut loop_speed = 5;

    let mut count: Option<u16> = None;
    let mut caught: u64 = 0;
    let mut missed: u64 = 0;

    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                match parser.push(&buffer[..n]) {
                    Some(packets) => {
                        for packet in packets {
                            let packet_count = packet.count.clone();
                            match postcard::to_allocvec(&packet) {
                                Ok(bin_packet) => {
                                    stream_tx.blocking_send(OutboundData::Binary(bin_packet));
                                }
                                Err(e) => {
                                    // I dont know if we should continue here if we want to track packet metrics?
                                    // Should we count it as an error?
                                    error!("Error serializing telemetry packet :: {:?}", e);
                                    continue;
                                }
                            }

                            caught += 1;

                            if let Some(previous_count) = count {
                                let packet_delta = packet_count.wrapping_sub(previous_count);

                                if packet_delta > 1 && packet_delta <= u16::MAX / 2 {
                                    let recent_missed = u64::from(packet_delta - 1);
                                    println!("Packets Missed! :: {}", recent_missed);
                                    missed += recent_missed;

                                    let loss_percentage = missed as f64
                                        / (caught + missed) as f64
                                        * 100.0;
                                    println!("Percentage of packet loss :: {:.2}%", loss_percentage);
                                }
                            }

                            count = Some(packet_count);

                        }
                    }
                    None => {}
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