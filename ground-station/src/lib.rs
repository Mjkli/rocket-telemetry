pub mod parcer;
pub mod buffer_reader;

use anyhow::Result;
use crate::buffer_reader::buf_reader::buf_reader;
use tokio::sync::mpsc::{self, Sender, Receiver};
use websocket_server::WebsocketServer;
use tokio::task::JoinHandle;



pub async fn ground_station() -> Result<()> {

    let (main_stream_tx, mut main_stream_rx)  = tokio::sync::mpsc::channel::<String>(100);
    let (main_message_tx, mut main_message_rx) = tokio::sync::mpsc::channel::<String>(100);
    
    let main_stream_tx_clone = main_stream_tx.clone();
    let buffer_thread: JoinHandle<Result<(), anyhow::Error>> = tokio::spawn( async move {
            buf_reader("/dev/serial0", main_stream_tx_clone).await
        }
    );
    
    let mut message_ws = WebsocketServer::new("0.0.0.0:8080", main_message_tx.clone());
    let mut stream_ws = WebsocketServer::new("0.0.0.0:8081", main_stream_tx.clone());

    message_ws.serve()?;
    stream_ws.serve()?;

    println!("Servers have started: 8080 : 8081");



    loop {
        tokio::select! {
            Some(_) = main_message_rx.recv() => {

            }
        }
    }
     
    Ok(())
}