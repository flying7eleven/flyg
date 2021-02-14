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
