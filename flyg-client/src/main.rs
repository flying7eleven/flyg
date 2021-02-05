fn initialize_logging() {
    use chrono::Local;
    use log::LevelFilter;

    let logging_framework = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply();

    // ensure the logging framework was successfully initialized
    if logging_framework.is_err() {
        panic!("Could not initialize the logging framework. Terminating!");
    }
}

fn main() {
    use flyg::simconnect::{Notification, SimConnect};
    use log::info;

    // initialize the logger and create the instance for the simualtor connection
    initialize_logging();
    let mut simulator_connection = SimConnect::new();

    // establish a connection to the simulator
    match simulator_connection.connect() {
        Ok(_) => {}
        Err(error_message) => panic!("Could not connect to the simulator: {}", error_message),
    };

    // request position updates for the plane
    simulator_connection
        .request_position_updates()
        .expect("No position update!");

    // request ATC ID updates for the plane
    simulator_connection
        .request_atc_id_updates()
        .expect("No ATC ID update!");

    // request updates for the title of the plane
    simulator_connection
        .request_plane_title_updates()
        .expect("No plane title update!");

    // request updates for the performance parameter of the plane
    simulator_connection
        .request_plane_performance_parameter_updates()
        .expect("No plane title update!");

    // process the messages we receive from the simulator
    loop {
        match simulator_connection.get_next_notification() {
            Some(Notification::Connected) => info!("Connection opened!"),
            Some(Notification::Disconnected) => info!("Connection closed!"),
            Some(Notification::Position(position)) => {
                info!(
                    "Position update. Altitude: {:.0}ft (lat. {}, long. {})",
                    position.altitude, position.latitude, position.longitude
                );
            }
            Some(Notification::AircraftAtcId(atc_infos)) => {
                info!("ATC ID update. Tail number: {}", atc_infos.tail_number);
                info!("ATC ID update. Callsign: {}", atc_infos.callsign);
                info!("ATC ID update. Flight number: {}", atc_infos.flight_number);
            }
            Some(Notification::AircraftTitle(title)) => {
                info!("Aircraft title update. Title: {}", title);
            }
            Some(Notification::AircraftParameters(parameters)) => {
                info!("Aircraft parameters received!");
                info!(
                    "  - Number of engines         : {}",
                    parameters.number_of_engines
                );
                info!(
                    "  - Type of engines           : {:?}",
                    parameters.engine_type
                );
                info!(
                    "  - Engine RPM       (1,2,3,4): {}, {}, {}, {}",
                    parameters.engine_rpm[0],
                    parameters.engine_rpm[1],
                    parameters.engine_rpm[2],
                    parameters.engine_rpm[3]
                );
                info!(
                    "  - Engine FF [Lb/h] (1,2,3,4): {}, {}, {}, {}",
                    parameters.fuel_flow[0],
                    parameters.fuel_flow[1],
                    parameters.fuel_flow[2],
                    parameters.fuel_flow[3]
                );
            }
            None => {}
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
