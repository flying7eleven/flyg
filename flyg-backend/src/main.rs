use flyg_backend::get_configuration;
use std::time::Duration;

fn setup_logger() {
    use chrono::Utc;
    use log::LevelFilter;

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

fn wait_for_database(_: &str) {
    use std::thread;
    thread::sleep(Duration::from_secs(5)); // TODO: do actually check the database and retry several times
}

fn main() {
    use flyg_backend::database::run_migrations;
    use flyg_backend::FlygDatabaseConnection;
    use rocket::{catchers, routes};

    // the first step is to initialize the logging
    setup_logger();

    // read the configuration
    let config = get_configuration();

    // if everything is ran in docker containers, it can take a while until the container
    // for the database is up and running. Just wait for it before continuing
    wait_for_database(&config.database_url);

    // ensure that the migrations on the database are ran before trying to
    // start the API
    run_migrations(&config.database_url);

    // if we could get the required database url, launch rocket for handling the requests to the backend
    rocket::ignite()
        .attach(FlygDatabaseConnection::fairing())
        .manage(config)
        .register(catchers![
            flyg_backend::routes::catcher_bad_request,
            flyg_backend::routes::catcher_unauthorized,
            flyg_backend::routes::catcher_forbidden,
            flyg_backend::routes::catcher_not_found,
            flyg_backend::routes::catcher_method_not_allowed,
            flyg_backend::routes::catcher_internal_server_error,
        ])
        .mount(
            "/v1",
            routes![
                flyg_backend::routes::health::get_health_status,
                flyg_backend::routes::auth::get_public_key_for_signature_validation,
                flyg_backend::routes::auth::get_login_token,
                flyg_backend::routes::airports::get_airport_information,
                flyg_backend::routes::airports::get_closest_airport_to_position
            ],
        )
        .launch();
}
