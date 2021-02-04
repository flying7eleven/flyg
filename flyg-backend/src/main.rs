use chrono::Utc;
use dotenv::dotenv;
use log::LevelFilter;
use rocket::routes;
use std::env;

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
        .chain(std::io::stdout())
        .apply();
}

fn main() {
    setup_logger();
    dotenv().ok();

    // if we could get the required database url, launch rocket for handling the requests to the backend
    if let Ok(database_url) = env::var("DATABASE_URL") {
        rocket::ignite()
            .manage(database_url.clone())
            .mount(
                "/v1",
                routes![flyg_backend::routes::airports::get_airport_information],
            )
            .launch();
    } else {
        panic!("Could not find the required DATABASE_URL for launching the backend");
    }
}
