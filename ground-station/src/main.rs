use anyhow::Result;
use ground_station::PacketParser;

fn main() -> Result<()>{
    let mut port = serialport::new("/dev/serial0", 9600)
        .timeout(std::time::Duration::from_millis(100))
        .open()?;
    
    let mut buffer = [0u8; 256];
    let mut parser = PacketParser::new();
    let mut loop_speed = 5; // 5 ms delay between reads
    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                match parser.push(&buffer[..n]) {
                    Some(packets) => {
                        for packet in packets {
                            println!("Received packet roll and pitch: {} :: {}", packet.roll, packet.pitch);
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
