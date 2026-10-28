use anyhow::Result;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use uuid::Uuid;
use fastwebsockets::{WebSocket, OpCode, Role};
use tokio::net::{TcpListener, TcpStream};
use std::sync::atomic::Ordering;



pub struct WebsocketServer {
    clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>,
    runtime: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>, 
    addr: &'static str,
    main_tx: tokio::sync::mpsc::Sender<String>,
}

impl WebsocketServer {
    pub fn new(addr: &'static str, main_tx: tokio::sync::mpsc::Sender<String>) -> Self {
        WebsocketServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
            stop_flag: Arc::new(AtomicBool::new(false)),
            runtime: None,
            addr,
            main_tx,
        }
    }

    pub fn serve(&mut self) -> Result<()> {

        let clients_clone = Arc::clone(&self.clients);
        let stop_flag_clone = Arc::clone(&self.stop_flag);
        let addr_clone = self.addr;
        let main_tx_clone = self.main_tx.clone();

        self.runtime = Some(tokio::spawn(async move {
            let _ = websocket_lifetime(clients_clone, stop_flag_clone, addr_clone, main_tx_clone).await;
        }));
        Ok(())
    }

    pub fn stop(&mut self)  -> Result<()> {
        self.stop_flag.store(true, Ordering::SeqCst);
        self.runtime.take().map(|handle: JoinHandle<()>| handle.abort());
        Ok(())
    }
}

async fn websocket_lifetime(
    clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>, 
    stop_flag: Arc<AtomicBool>,
    address: &str,
    main_tx: tokio::sync::mpsc::Sender<String>
) -> Result<()>
{

    let listener = TcpListener::bind(address).await?;

    loop {
        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        let (socket, _) = listener.accept().await?;
        let client_id = Uuid::new_v4();
        
        let clients_clone = Arc::clone(&clients);
        let mut clients_lock = clients.lock().unwrap();
        let main_tx_clone = main_tx.clone();
        clients_lock.insert(client_id, tokio::spawn(async move {
            if let Err(e) = handle_client(socket, clients_clone, client_id.clone(), main_tx_clone).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        }));
    }



    Ok(())

}

async fn handle_client(stream: TcpStream, clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>, client_id: Uuid, main_tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {

    let mut ws: WebSocket<TcpStream> = WebSocket::after_handshake(stream, Role::Server);

    loop {
        let frame = match ws.read_frame().await{
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Error reading frame: {:?}", e);
                continue;
            }
        };

        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => { 
                let message: String = match String::from_utf8(frame.payload.to_vec()) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Invalid UTF-8: {:?}", e);
                        continue;
                    }
                };
            }
            _ => {}
        }
    }

    {
        let mut clients_lock = clients.lock().map_err(|e| anyhow::anyhow!("Failed to lock clients :: {:?}", e))?;
        clients_lock.remove(&client_id);
    }

    Ok(())
}