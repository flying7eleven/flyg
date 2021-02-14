pub enum Notification {
    Connected,
    Disconnected,
    PositionUpdate(AircraftPosition),
}

#[derive(Copy, Clone)]
pub struct AircraftPosition {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}
