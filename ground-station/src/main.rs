use anyhow::Result;
use ground_station::ground_station;

#[tokio::main]
async fn main() -> Result<()>{
    
    ground_station().await?;    

    Ok(())
}
