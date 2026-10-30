use anyhow::Result;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use uuid::Uuid;
use fastwebsockets::{WebSocket, OpCode, Role};
use tokio::net::{TcpListener, TcpStream};
use std::sync::atomic::Ordering;
use protocol::telemetry_data::TelemetryData;
use log::error;



#[derive(Debug, Clone)]
pub enum OutboundData {
    Text(String),
    Binary(Vec<u8>)
}

#[derive(Debug)]
enum WSCommand {
    Reply((Uuid, OutboundData)),
    BroadCast(OutboundData)
}


pub struct WebsocketServer {
    runtime: Option<JoinHandle<()>>,
    shutdown_tx: Option<tokio::sync::watch::Sender<bool>>, 
    addr: &'static str,
    main_tx: tokio::sync::mpsc::Sender<OutboundData>,
    ws_tx: Option<tokio::sync::mpsc::Sender<WSCommand>>,
}

impl WebsocketServer {
    pub fn new(addr: &'static str, main_tx: tokio::sync::mpsc::Sender<OutboundData>) -> Self {
        WebsocketServer {
            runtime: None,
            shutdown_tx: None,
            addr,
            main_tx,
            ws_tx: None,
        }
    }

    pub fn serve(&mut self) -> Result<()> {

        let (shutdown_tx, mut shutdown_rx) = tokio::sync::watch::channel::<bool>(true);
        let (ws_tx, mut ws_rx) = tokio::sync::mpsc::channel::<WSCommand>(10);
        let addr_clone = self.addr;
        let main_tx_clone = self.main_tx.clone();

        self.runtime = Some(tokio::spawn(async move {
            let _ = websocket_lifetime(addr_clone, main_tx_clone, ws_rx, shutdown_rx).await;
        }));

        self.ws_tx = Some(ws_tx);
        self.shutdown_tx = Some(shutdown_tx);


        Ok(())
    }

    pub fn stop(&mut self)  -> Result<()> {
        
        self.runtime.take().map(|handle: JoinHandle<()>| handle.abort());
        Ok(())
    }

    pub async fn reply(&mut self, uuid: Uuid, data: OutboundData) -> Result<()> {
        match &self.ws_tx {
            Some(tx) => {
                tx.send(WSCommand::Reply((uuid, data))).await?;
            }
            None => {
                return Err(anyhow::anyhow!("called broadcast on empty websocket server"));
            }
        }

        Ok(())
    }

    pub async fn broadcast(&mut self, data: OutboundData) -> Result<()> {
        match &self.ws_tx {
            Some(tx) => {
                tx.send(WSCommand::BroadCast(data)).await?;
            }
            None => {
                return Err(anyhow::anyhow!("called broadcast on empty websocket server"));
            }
        }

        Ok(())
    }
}


// websocket lifetime thread
async fn websocket_lifetime(
    address: &str,
    main_tx: tokio::sync::mpsc::Sender<OutboundData>,
    mut ws_rx: tokio::sync::mpsc::Receiver<WSCommand>,
    mut shutdown_rx: tokio::sync::watch::Receiver<bool>,
) -> Result<()>
{

    let listener = TcpListener::bind(address).await?;
    let mut client_threads: HashMap<Uuid, JoinHandle<()>> = HashMap::new();
    let mut client_channels: Arc<Mutex<HashMap<Uuid, tokio::sync::mpsc::Sender<OutboundData>>>> = Arc::new(Mutex::new(HashMap::new()));


    loop {
        tokio::select! {
            // New client connected
            result = listener.accept() => {
                let (socket, _) = result?;
                let client_id = Uuid::new_v4();
                
                let (client_tx, client_rx) = tokio::sync::mpsc::channel::<OutboundData>(10);
                {
                    let mut client_guard = client_channels.lock().map_err(|e| anyhow::anyhow!("Error locking client channel mutex :: {:?}", e))?;
                    client_guard.insert(client_id, client_tx);
                }

                let client_channels_clone = client_channels.clone();

                
                let main_tx_clone = main_tx.clone();
                let shutdown_clone = shutdown_rx.clone();

                client_threads.insert(client_id, tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, client_channels_clone, client_id.clone(), main_tx_clone, client_rx, shutdown_clone).await {
                        eprintln!("Error handling connection: {:?}", e);
                    }
                }));
            }

            //outbound to client(s)
            command = ws_rx.recv() => {
                match command {
                    Some(WSCommand::Reply((uuid, data))) => {
                        
                    },
                    Some(WSCommand::BroadCast(data)) => {
                        println!("Broadcasting :: {:?}", data);
                        let client_txs = {
                        let clients = client_channels
                            .lock()
                            .map_err(|_| anyhow::anyhow!("Failed to lock mutex"))?;

                            clients.values().cloned().collect::<Vec<_>>()
                        };

                        for tx in client_txs {
                            tx.send(data.clone()).await?;
                        }
                    }
                    None => {
                        error!("Received a None out to client?");
                    }
                }
            }

            //shutdown handle
            _ = shutdown_rx.changed() => {

                break;
            }
        }
    }



    Ok(())

}

// Per-client thread that handles talking to client
async fn handle_client(
    stream: TcpStream, 
    clients: Arc<Mutex<HashMap<Uuid,tokio::sync::mpsc::Sender<OutboundData>>>>, 
    client_id: Uuid, 
    mut main_tx: tokio::sync::mpsc::Sender<OutboundData>,
    mut client_rx: tokio::sync::mpsc::Receiver<OutboundData>,
    mut shutdown_rx: tokio::sync::watch::Receiver<bool> 
) -> Result<()> {

    let mut ws: WebSocket<TcpStream> = WebSocket::after_handshake(stream, Role::Server);

    loop {
        tokio::select! {

            out = client_rx.recv() => {

            }
            
            //shutdown handle
            _ = shutdown_rx.changed() => {
                break;
            }

        }   
    }

    {
        let mut clients_lock = clients.lock().map_err(|e| anyhow::anyhow!("Failed to lock clients :: {:?}", e))?;
        clients_lock.remove(&client_id);
    }

    Ok(())
}