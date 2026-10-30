pub mod parcer;
pub mod buffer_reader;

use anyhow::Result;
use crate::buffer_reader::buf_reader::buf_reader;
use tokio::sync::mpsc::{self, Sender, Receiver};
use websocket_server::{OutboundData, WebsocketServer};
use tokio::task::JoinHandle;



pub async fn ground_station() -> Result<()> {

    let (main_stream_tx, mut main_stream_rx)  = tokio::sync::mpsc::channel::<OutboundData>(100);
    let (main_message_tx, mut main_message_rx) = tokio::sync::mpsc::channel::<OutboundData>(100);
    
    let main_stream_tx_clone = main_stream_tx.clone();
    let buffer_thread: JoinHandle<Result<(), anyhow::Error>> = tokio::task::spawn_blocking(move || {
            buf_reader("/dev/serial0", main_stream_tx_clone)
        }
    );
    
    let mut message_ws = WebsocketServer::new("0.0.0.0:8080", main_message_tx.clone());
    let mut stream_ws = WebsocketServer::new("0.0.0.0:8081", main_stream_tx.clone());

    message_ws.serve()?;
    stream_ws.serve()?;

    println!("Servers have started: 8080 : 8081");

    let stream_thread: JoinHandle<()> = tokio::spawn(async move {
        stream_broadcast(stream_ws, main_stream_rx).await;
    });

    loop {
        tokio::select! {
            Some(_) = main_message_rx.recv() => {

            }
        }
    }


    buffer_thread.abort();
    stream_thread.abort();    
     
    Ok(())
}

async fn stream_broadcast(mut stream_ws: WebsocketServer, mut stream_rx: tokio::sync::mpsc::Receiver<OutboundData>) -> Result<()> {

    while let Some(packet) = stream_rx.recv().await {
        if let Err(e)  = stream_ws.broadcast(packet).await {
            println!("Error sending to broadcast fn :: {:?}", e);
        }
    }

    Ok(())
}