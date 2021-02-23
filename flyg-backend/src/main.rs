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
    use rocket::catchers;

    setup_logger();

    // if we could get the required database url, launch rocket for handling the requests to the backend
    rocket::ignite()
        .attach(FlygDatabaseConnection::fairing())
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
                flyg_backend::routes::auth::get_login_token,
                flyg_backend::routes::airports::get_airport_information,
                flyg_backend::routes::airports::get_closest_airport_to_position
            ],
        )
        .launch();
}
