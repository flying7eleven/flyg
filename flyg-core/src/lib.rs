/// Notifications which can be received while waiting for simulator events.
pub enum Notification {
    /// The simulator is connected to the client.
    Connected,
    /// The simulator disconnected from the client.
    Disconnected,
    /// The simulator send a position update for the plane.
    PositionUpdate(AircraftPosition),
}

/// Store the position of the aircraft at the time of the recording.
///
/// # Example
/// ```
/// use flyg_core::AircraftPosition;
///
/// let current_position = AircraftPosition {
///     latitude: 51.225372,
///     longitude: 6.778913,
///     altitude: 0.0, // on the ground, so 0.0
/// };
/// ```
#[derive(Copy, Clone)]
pub struct AircraftPosition {
    /// The current latitude of the aircraft.
    pub latitude: f32,
    /// The current longitude of the aircraft.
    pub longitude: f32,
    /// The current altitude (above ground) of the aircraft.
    pub altitude: f32,
}

/// The type of the engines of the plane.
pub enum EngineType {
    /// No engines (e.g. glider).
    NoEngine,
    /// Propellers (e.g. C172).
    Propeller,
    /// Turbo-prop(e.g. TBM).
    TurboProp,
    /// Jet engines (e.g. A320).
    Jet,
}

/// Information about the aircraft (brand, model, engine types, etc.).
///
/// # Example
/// ```
/// use flyg_core::AircraftInformation;
/// use flyg_core::EngineType::Propeller;
///
/// let aircraft_info = AircraftInformation {
///     registration: "D-GOIA".to_string(),
///     engine_type: Propeller,
///     engine_count: 2,
/// };
/// ```
pub struct AircraftInformation {
    /// The registration of the plane (e.g. D-GOIA).
    pub registration: String,
    /// The type of the equipped engines.
    pub engine_type: EngineType,
    /// The number of the engines of the plane.
    pub engine_count: u8,
}

/// A single recording (for a specific time point) of the aircraft information.
///
/// # Example
/// ```
/// use flyg_core::{AircraftInformationRecord, AircraftPosition};
///
/// let aircraft_info_record = AircraftInformationRecord {
///     time: 1614949813,
///     position: AircraftPosition {
///         latitude: 51.225372,
///         longitude: 6.778913,
///         altitude: 0.0, // on the ground, so 0.0
///     },
/// };
/// ```
pub struct AircraftInformationRecord {
    /// The time of the represented measurement as a UNIX timestamp.
    pub time: i64,
    /// The current position of the aircraft.
    pub position: AircraftPosition,
}
