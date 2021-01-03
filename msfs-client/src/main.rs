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
    use flyg_msfs_client::simconnect::{Notification, SimConnect};
    use log::info;

    initialize_logging();

    let simulator_connection = match SimConnect::new() {
        Ok(connection) => connection,
        Err(error_message) => panic!("Could not connect to the simulator: {}", error_message),
    };

    //
    simulator_connection
        .request_position_updates()
        .expect("No position update!");

    loop {
        match simulator_connection.get_next_notification() {
            Some(Notification::Connected) => info!("Connection opened!"),
            Some(Notification::Disconnected) => info!("Connection closed!"),
            Some(Notification::Position(position)) => {
                info!("Position update. Altitude: {:.0}ft", position.altitude)
            }
            None => {}
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
