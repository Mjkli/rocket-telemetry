use cobs;
use anyhow::Result;
use postcard::to_allocvec;
use crc::{Crc, CRC_16_IBM_SDLC};

const CRC16: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);


pub fn encode<T>(value: &T, count: u16) -> Result<Vec<u8>> 
where
T: serde::Serialize,
{
    let mut buffer: Vec<u8> = {
        let mut b = Vec::new();
        b.push((count >> 8) as u8);
        b.push((count & 0xFF) as u8);
        b
    };
    
    buffer.append(&mut to_allocvec(value)?);
    
    let crc: u16 = CRC16.checksum(&buffer);
    // Bit shift then drop the lower 8 bits to get the upper 8 bits
    buffer.push((crc >> 8) as u8);
    // Drop the upper 8 bits to get the lower 8 bits, then push it to the vector
    buffer.push((crc & 0xFF) as u8);

    let bytes_with_cobs: Vec<u8> = cobs::encode_vec(&buffer);

    Ok(bytes_with_cobs)
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

pub fn decode<T>(data: &[u8]) -> Result<(T, u16)> 
where T: serde::de::DeserializeOwned
{
    let decoded_data = cobs::decode_vec(data)?;
    validate_crc(&decoded_data)?;

    let data = &decoded_data[2..decoded_data.len() - 2];
    let count = u16::from_be_bytes([decoded_data[0], decoded_data[1]]);
    let value: T = postcard::from_bytes(data)?;

    Ok((value, count))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct TestData {
            a: u32,
            b: String,
        }

        let original_data = TestData {
            a: 42,
            b: "Hello, world!".to_string(),
        };

        let encoded_data = encode(&original_data, 1).unwrap();
        let (decoded_data, count): (TestData, u16) = decode(&encoded_data).unwrap();

        assert_eq!(original_data, decoded_data);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_scaling_count() {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
        struct TestData {
            a: u32,
            b: String,
        }

        
        for count in 1..=65535 {
            let original_data = TestData {
                a: rand::random(),
                b: "Hello, world!".to_string(),
            };
            let encoded_data = encode(&original_data, count).unwrap();
            let (decoded_data, decoded_count): (TestData, u16) = decode(&encoded_data).unwrap();

            assert_eq!(original_data, decoded_data);
            assert_eq!(count, decoded_count);
        }
    }
}