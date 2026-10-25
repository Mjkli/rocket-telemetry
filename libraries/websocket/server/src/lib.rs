use anyhow::Result;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use uuid::Uuid;
use fastwebsockets::{WebSocket, OpCode, Role};
use tokio::net::{TcpListener, TcpStream};
use std::sync::atomic::Ordering;



struct WebsocketServer {
    clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>,
    runtime: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>, 
    addr: &'static str,
}

impl WebsocketServer {
    fn new(addr: &'static str) -> Self {
        WebsocketServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
            stop_flag: Arc::new(AtomicBool::new(false)),
            runtime: None,
            addr,
        }
    }

    fn serve(&mut self) -> Result<()> {

        let clients_clone = Arc::clone(&self.clients);
        let stop_flag_clone = Arc::clone(&self.stop_flag);
        let addr_clone = self.addr;

        self.runtime = Some(tokio::spawn(async move {
            let _ = websocket_lifetime(clients_clone, stop_flag_clone, addr_clone).await;
        }));
        Ok(())
    }

    fn stop(&mut self)  -> Result<()> {
        self.stop_flag.store(true, Ordering::SeqCst);
        self.runtime.take().map(|handle: JoinHandle<()>| handle.abort());
        Ok(())
    }
}

async fn websocket_lifetime(
    clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>, 
    stop_flag: Arc<AtomicBool>,
    address: &str
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
        clients_lock.insert(client_id, tokio::spawn(async move {
            if let Err(e) = handle_client(socket, clients_clone, client_id.clone()).await {
                eprintln!("Error handling connection: {:?}", e);
            }
        }));
    }



    Ok(())

}

async fn handle_client(stream: TcpStream, clients: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>, client_id: Uuid) -> Result<()> {

    let mut ws: WebSocket<TcpStream> = WebSocket::after_handshake(stream, Role::Server);

    loop {
        let frame = match ws.read_frame().await{
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("Error reading frame: {:?}", e);
                break;
            }
        };

        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => { 
                if ws.write_frame(frame).await.is_err() {
                    eprintln!("Error writing frame");
                    break;
                }
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