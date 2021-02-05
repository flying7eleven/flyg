use chrono::Utc;
use flyg_backend::FlygDatabaseConnection;
use log::LevelFilter;
use rocket::routes;

fn setup_logger() {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        //.level_for("hyper::server", LevelFilter::Info)
        .level_for("hyper", LevelFilter::Info)
        .chain(std::io::stdout())
        .apply();
}

fn main() {
    setup_logger();

    // if we could get the required database url, launch rocket for handling the requests to the backend
    rocket::ignite()
        .attach(FlygDatabaseConnection::fairing())
        .mount(
            "/v1",
            routes![
                flyg_backend::routes::airports::get_airport_information,
                flyg_backend::routes::airports::get_closest_airport_to_position
            ],
        )
        .launch();
}
