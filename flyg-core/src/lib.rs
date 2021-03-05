pub enum Notification {
    Connected,
    Disconnected,
    PositionUpdate(AircraftPosition),
}

/// Store the position of the aircraft at the time of the recording.
#[derive(Copy, Clone)]
pub struct AircraftPosition {
    /// The current latitude of the aircraft.
    pub latitude: f64,
    /// The current longitude of the aircraft.
    pub longitude: f64,
    /// The current altitude (above ground) of the aircraft.
    pub altitude: f64,
}

/// A single recording (for a specific time point) of the aircraft information.
pub struct AircraftInformationRecord {
    /// The current position of the aircraft.
    pub position: AircraftPosition,
}
