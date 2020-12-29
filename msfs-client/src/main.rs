use chrono::Local;
use flyg_msfs_client::bindings;
use flyg_msfs_client::simconnect::Events;
use flyg_msfs_client::simconnect::SimConnect;
use log::{debug, error, info, LevelFilter};
use std::convert::TryFrom;
use std::ffi::c_void;

fn initialize_logging() {
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
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply();

    // ensure the logging framework was successfully initialized
    if logging_framework.is_err() {
        panic!("Could not initialize the logging framework. Terminating!");
    }
}

unsafe extern "C" fn callback(
    data: *mut bindings::SIMCONNECT_RECV,
    _cb_data: bindings::DWORD,
    _context: *mut c_void,
) {
    match (*data).dwID as i32 {
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
            let exception = *(data as *const bindings::SIMCONNECT_RECV_EXCEPTION);
            error!(
                "Exception: dwException {}, dwSendID {}, dwIndex {}",
                exception.dwException, exception.dwSendID, exception.dwIndex
            );
        }
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
            debug!("Open!");
        }
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
            let event = *(data as *const bindings::SIMCONNECT_RECV_EVENT);
            match Events::try_from(event.uEventID) {
                Ok(Events::Brakes) => info!("Breaks!"),
                _ => debug!("Unknown event ID"),
            }
        }
        id => {
            debug!("Unknown identifier: {}", id);
        }
    }
}

fn main() {
    initialize_logging();
    let simulator_connection = match SimConnect::new() {
        Ok(connection) => connection,
        Err(_) => panic!("Could not connect to the simulator"),
    };
    simulator_connection
        .associate_breaks()
        .expect("No break association!");

    loop {
        unsafe {
            bindings::SimConnect_CallDispatch(
                simulator_connection.handle.as_ptr(),
                Some(callback),
                std::ptr::null_mut(),
            );
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
