use super::schema::users;
use crate::database::FlygDatabaseError;
use diesel::prelude::*;

#[derive(Clone, Queryable, Identifiable)]
#[primary_key(id)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub password: String,
}

/// Get the user record for a specific email address.
///
/// Query the database for the user record associated with a specific email address. The
/// email address is marked as unique in the database, so it can be used as the username
/// to identify the user with.
///
/// # Arguments
/// * `database_connection` - The connection to the database servers for the query.
/// * `email_address_query` - The email address associated with the user to query.
///
/// # Errors
/// Will return `Err` if the requested runway information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the user with the given email address.
/// * `MoreThanOneResult` - Got more than one user which should not happen since the email address is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
pub fn get_user_record(
    database_connection: &PgConnection,
    email_address_query: &String,
) -> Result<User, FlygDatabaseError> {
    use super::schema::users::dsl::{email_address, users};

    // try to query the user record for the supplied email address
    if let Ok(found_users) = users
        .filter(email_address.eq(email_address_query))
        .load::<User>(database_connection)
    {
        // check if there is at least one user record for the supplied email address, if not, return
        // an error immediately
        if found_users.len() < 1 {
            return Err(FlygDatabaseError::NoResults);
        }

        // ensure that we did not find more than one result, if some, something is REALLY wrong
        if found_users.len() > 1 {
            return Err(FlygDatabaseError::MoreThanOneResult);
        }

        // return the found user record
        return Ok(found_users[0].clone());
    }

    // it seems that we completely failed to query the database for the requested information
    Err(FlygDatabaseError::FailedToQueryDatabase)
}
