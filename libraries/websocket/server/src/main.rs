
#[tokio::main]
async fn main() {
    let (main_stream_tx, mut main_stream_rx)  = tokio::sync::mpsc::channel::<websocket_server::OutboundData>(100);
    let mut test_ws = websocket_server::WebsocketServer::new("0.0.0.0:8080", main_stream_tx.clone());

    test_ws.serve().unwrap();

    println!("Websocket server started on 8080");

    loop {
        tokio::select! {
            Some(input) = main_stream_rx.recv() => {
                println!("Received data from websocket server: {:?}", input);
            }
        }
    }

}
