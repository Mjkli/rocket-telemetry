use serde::{Serialize, Deserialize};


pub const TELEMETRY_DATA_SIZE = 38;

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryData {
    pub roll: f64,
    pub pitch: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub flight_state: FlightState,
    pub count: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlightState {
    Preflight,
    Launch,
    Apogee,
    Descent,
    Landed,
}