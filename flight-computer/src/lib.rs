
pub struct TelemetryData {
    pub roll: f64,
    pub pitch: f64,
    pub altitude: f64,
    pub temperature: f64,
    pub flight_state: FlightState,
}

pub enum FlightState {
    Preflight,
    Launch,
    Apogee,
    Descent,
    Landed,
}