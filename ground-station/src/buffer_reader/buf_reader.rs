use anyhow::Result;
use serialport::SerialPort;
use crate::parcer::parcer::PacketParser;
use tokio::sync::mpsc::Sender;


pub fn buf_reader<T>(serial_port: &str, main_tx: Sender<T>) -> Result<()> {
 let mut port = serialport::new(serial_port, 9600)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;
    
    let mut buffer = [0u8; 256];
    let mut parser = PacketParser::new();
    let mut loop_speed = 5;
    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                match parser.push(&buffer[..n]) {
                    Some(packets) => {
                        for packet in packets {
                            println!("Received packet: {:?}", packet); 
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

        std::thread::sleep(std::time::Duration::from_millis(loop_speed));
    }

    Ok(())
}