use flyg_core::Notification;

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
    use flyg_msfs2020::simconnect::SimConnect;
    use log::info;
    use std::sync::{Arc, Mutex};
    use std::thread::spawn;

    // initialize the logger and create the instance for the simualtor connection
    initialize_logging();
    let mutex_sim_connection = Mutex::new(SimConnect::new());
    let arc_sim_connection = Arc::new(mutex_sim_connection);

    // establish a connection to the simulator before even trying to process its data
    {
        let mut guard = arc_sim_connection.lock().unwrap();
        match guard.connect() {
            Ok(_) => {}
            Err(error_message) => panic!("Could not connect to the simulator: {}", error_message),
        };
    }

    // process the messages we receive from the simulator in a new thread
    let data_processing_thread;
    {
        let arc_sim_connection = arc_sim_connection.clone();
        data_processing_thread = spawn(move || loop {
            let simulator_connection = match arc_sim_connection.lock() {
                Ok(guard) => guard,
                Err(error) => {
                    info!("It seems that the mutex guard is currently locked ({}), waiting for a while...", error);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }
            };

            // ensure that we have a connection to the simulator. if not, wait some time and try again
            if !simulator_connection.is_connected() {
                std::mem::drop(simulator_connection);
                info!("It seems, that we do not have a connection to the simulator. Waiting for some seconds before continuing...");
                std::thread::sleep(std::time::Duration::from_secs(5));
                continue;
            }

            match simulator_connection.get_next_notification() {
                Some(Notification::Connected) => info!("Connection opened!"),
                Some(Notification::Disconnected) => info!("Connection closed!"),
                Some(Notification::PositionUpdate(position)) => {
                    info!(
                        "Position update. Altitude: {:.0}ft (lat. {}, long. {})",
                        position.altitude, position.latitude, position.longitude
                    );
                }
                None => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        });
    }

    // request position updates for the plane
    {
        let arc_sim_connection = arc_sim_connection.clone();
        let guard = arc_sim_connection.lock().unwrap();
        guard
            .request_position_updates()
            .expect("No position update!");
    }

    // wait until the data processing thread has finished
    data_processing_thread.join().unwrap();
}
