pub mod airports;
pub mod auth;
pub mod schema;

#[derive(Debug)]
pub enum FlygDatabaseError {
    /// Failed to query the database for the requested information.
    FailedToQueryDatabase,
    /// Got more than one results which was not expected.
    MoreThanOneResult,
    /// Could not find the requested information.
    NoResults,
}

#[cfg(debug_assertions)]
embed_migrations!("migrations/");

#[cfg(not(debug_assertions))]
pub fn run_migrations(database_url: &str) {
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use log::info;

    //
    info!("Running database migrations to ensure the scheme is up to date!");

    // try to establish a connection to the local database
    let db_connection = match PgConnection::establish(database_url) {
        Ok(connection) => connection,
        Err(error) => {
            panic!(
                "Could not connect to the database. The error was: {}",
                error
            );
        }
    };

    // if the migration failed, we have to terminate since we cannot interact with the
    // database
    match embedded_migrations::run(&db_connection) {
        Ok(_) => info!("Successfully, ran the database migrations"),
        Err(_) => panic!("Failed to run the database migrations, terminating..."),
    }
}

#[cfg(debug_assertions)]
pub fn run_migrations(_: &str) {
    use log::info;
    info!("Not running database migrations since we are in dev mode!");
}
