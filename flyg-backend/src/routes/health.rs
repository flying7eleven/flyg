use crate::FlygDatabaseConnection;
use rocket::get;
use rocket::http::Status;

/// # Route for the health check of the server
///
/// The route returns 201 if the system is healthy and any other error status
/// if not.
#[get("/health")]
pub fn get_health_status(_database_connection: FlygDatabaseConnection) -> Status {
    Status::NoContent // TODO: add a DB check
}
