use flyg_msfs_client::bindings;
use flyg_msfs_client::simconnect::Events;
use flyg_msfs_client::simconnect::SimConnect;
use std::convert::TryFrom;
use std::ffi::c_void;

unsafe extern "C" fn callback(
    data: *mut bindings::SIMCONNECT_RECV,
    _cb_data: bindings::DWORD,
    _context: *mut c_void,
) {
    match (*data).dwID as i32 {
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EXCEPTION => {
            let exception = *(data as *const bindings::SIMCONNECT_RECV_EXCEPTION);
            println!(
                "Exception: dwException {}, dwSendID {}, dwIndex {}",
                exception.dwException, exception.dwSendID, exception.dwIndex
            );
        }
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
            println!("Open!");
        }
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => {
            let event = *(data as *const bindings::SIMCONNECT_RECV_EVENT);
            match Events::try_from(event.uEventID) {
                Ok(Events::Brakes) => println!("Breaks!"),
                _ => println!("Unknown event ID"),
            }
        }
        id => {
            println!("Unknown identifier: {}", id);
        }
    }
}

fn main() {
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
