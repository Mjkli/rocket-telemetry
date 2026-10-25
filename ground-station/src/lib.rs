pub mod parcer;
pub mod buffer_reader;

use anyhow::Result;
use crate::buffer_reader::buf_reader::buf_reader;


pub fn ground_station() -> Result<()>{
    
    buf_reader("/dev/serial0");
     
    Ok(())
}