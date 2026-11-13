use anyhow::Result;
use serialport::SerialPort;
use crate::parcer::parcer::PacketParser;
use crate::buffer::buffer::RingBuffer;
use tokio::sync::mpsc::Sender;


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

    let mut buffer = RingBuffer::<u8>::new(256, 0);

    loop {
        match port.read(&mut inbound) {
            Ok(n) => {
                buffer.push(&inbound[..n]);
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