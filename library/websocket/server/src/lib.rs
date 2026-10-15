use anyhow::Result;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex}
use std::sync::atomic::AtomicBool;
use uuid::Uuid;


struct WebsocketServer {
    clients: Arc<Mutex<HashMap<Uuid, String>>,
    runtime: Option<JoinHandle<()>>,
    stop_flag: Arc<AtomicBool>, 

}

impl WebsocketServer {
    fn new() -> Self {
        WebsocketServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
            stop_flag: Arc::new(AtomicBool::new(false)),
            runtime: None,
        }
    }

    fn serve(&self) {

        let clients_clone = Arc::clone(&self.clients);
        let stop_flag_clone = Arc::clone(&self.stop_flag);

        self.runtime = Some(tokio::spawn(async move {
            websocket_lifetime(clients_clone, stop_flag_clone).await;
        }));
        
    }

    fn stop(&self) {
        
    }
}

async fn websocket_lifetime(
    clients: Arc<Mutex<HashMap<Uuid, String>>>, 
    stop_flag: Arc<AtomicBool>) -> Result<()>
{
    loop {
        if stop_flag.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }


    Ok(())

}