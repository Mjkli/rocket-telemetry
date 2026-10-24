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
    clients: Arc<Mutex<HashMap<Uuid, String>>>,
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
    clients: Arc<Mutex<HashMap<Uuid, String>>>, 
    stop_flag: Arc<AtomicBool>,
    address: &str
) -> Result<()>
{
    let listener = TcpListener::bind(address).await?;
    let tcp_stream: TcpStream = listener.accept().await?.0;


    let mut ws = WebSocket::after_handshake(tcp_stream, Role::Server);
    ws.set_auto_close(true);
    ws.set_auto_pong(true);

    loop {
        let frame = ws.read_frame().await?;
        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => { ws.write_frame(frame).await?; }
            _ => {}
        }
    }



    Ok(())

}