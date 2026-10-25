pub mod parcer;
pub mod buffer_reader;

use anyhow::Result;
use crate::buffer_reader::buf_reader::buf_reader;
use tokio::sync::mpsc::{self, Sender, Receiver};



pub async fn ground_station() -> Result<()>{
    let (main_tx, mut main_rx)  = tokio::sync::mpsc::channel(100);
    buf_reader("/dev/serial0", main_tx.clone())?;

    loop {
        tokio::select! {
            Some(_) = main_rx.recv() => {
                // Handle received data from buf_reader
            }
        
        
        }
    }
     
    Ok(())
}