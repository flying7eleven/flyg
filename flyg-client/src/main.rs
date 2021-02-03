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

    // process the messages we receive from the simulator
    loop {
        match simulator_connection.get_next_notification() {
            Some(Notification::Connected) => info!("Connection opened!"),
            Some(Notification::Disconnected) => info!("Connection closed!"),
            Some(Notification::Position(position)) => {
                info!("Position update. Altitude: {:.0}ft", position.altitude)
            }
            Some(Notification::AircraftAtcId(atc_infos)) => {
                info!("ATC ID update. New tail number {}", atc_infos.atc_id);
            }
            None => {}
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
