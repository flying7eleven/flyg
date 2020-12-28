use flyg_msfs_client::simconnect::SimConnect;

fn main() {
    let _simulator_connection = match SimConnect::new() {
        Ok(connection) => connection,
        Err(_) => panic!("Could not connect to the simulator"),
    };
}
