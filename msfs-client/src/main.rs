use std::time::Duration;

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
    use flyg_msfs_client::simconnect::{Events, Notification, SimConnect};
    use log::info;

    initialize_logging();

    let event = Events::Brakes;
    let simulator_connection = match SimConnect::new() {
        Ok(connection) => connection,
        Err(error_code) => panic!("Could not connect to the simulator: 0x{:x}", error_code),
    };
    simulator_connection
        .register_event(event)
        .expect("No break association!");

    simulator_connection.display_message_to_user(
        "Flyg connected to the simulator".to_string(),
        Duration::from_secs(5),
    );

    loop {
        match simulator_connection.get_next_notification() {
            Some(Notification::Connected) => info!("Connection opened!"),
            Some(Notification::Disconnected) => info!("Connection closed!"),
            Some(Notification::Brakes) => info!("Brakes!"),
            None => {}
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
