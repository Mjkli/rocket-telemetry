use cobs;
use anyhow::Result;
use postcard::to_allocvec;
use crc::{Crc, CRC_16_IBM_SDLC};

const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);


pub fn encode<T>(value: &T) -> Result<Vec<u8>> 
where
T: serde::Serialize,
{
    let bytes: Vec<u8> = to_allocvec(value)?;
    let bytes_with_crc: Vec<u8> = {
        let mut b = bytes.clone();
            let crc: u16 = CRC16.checksum(&b);
            // Bit shift then drop the lower 8 bits to get the upper 8 bits
            b.push((crc >> 8) as u8);
            // Drop the upper 8 bits to get the lower 8 bits, then push it to the vector
            b.push((crc & 0xFF) as u8);
            b
            
        };

    Ok(cobs::encode_vec(&bytes_with_crc))

}

fn validate_crc(data: &[u8]) -> Result<()> {
    if data.len() < 2 {
        return Err(anyhow::anyhow!("Data is too short to contain CRC"));
    }

    let data_without_crc = &data[..data.len() - 2];
    let received_crc = u16::from_be_bytes([data[data.len() - 2], data[data.len() - 1]]);
    let calculated_crc = CRC16.checksum(data_without_crc);

    if received_crc != calculated_crc {
        return Err(anyhow::anyhow!("CRC validation failed"));
    }

    Ok(())
}

pub fn decode<T>(data: &[u8]) -> Result<T> 
where T: serde::de::DeserializeOwned
{
    let decoded_data = cobs::decode_vec(data)?;
    validate_crc(&decoded_data)?;

    let data_without_crc = &decoded_data[..decoded_data.len() - 2];
    let value: T = postcard::from_bytes(data_without_crc)?;

    Ok(value)
}