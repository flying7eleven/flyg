use super::schema::airports;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Clone, Queryable, Identifiable)]
#[primary_key(id)]
#[table_name = "airports"]
pub struct Airport {
    pub id: i32,
    pub icao_code: String,
    pub last_update: NaiveDateTime,
    pub country: String,
    pub longitude: f32,
    pub latitude: f32,
    pub name: String,
}

pub enum FlygDatabaseError {
    /// Failed to query the database for the requested information.
    FailedToQueryDatabase,
    /// Got more than one results which was not expected.
    MoreThanOneResult,
    /// Could not find the requested information.
    NoResults,
}

/// Query the database for all information for a specific airport.
///
/// This method can be used to query the database for all information
/// regarding a specific airport by its assigned ICAO code.
///
/// # Arguments
/// * `db_url` - The URL use to connect to the database server.
/// * `icao_code_to_query_for` - The four letter ICAO code to query for.
///
/// # Errors
/// Will return `Err` if the requested airport information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
/// * `MoreThanOneResult` - Got more than one airport which should not happen since the ICAO code is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
pub fn get_information_for_icao_code(
    db_url: &String,
    icao_code_to_query_for: &String,
) -> Result<Airport, FlygDatabaseError> {
    use super::schema::airports::dsl::{airports, icao_code};

    if let Ok(database_connection) = PgConnection::establish(&db_url) {
        if let Ok(found_airports) = airports
            .filter(icao_code.eq(icao_code_to_query_for))
            .load::<Airport>(&database_connection)
        {
            // return an matching error if we got non or too many results
            if found_airports.len() == 0 {
                return Err(FlygDatabaseError::NoResults);
            } else if found_airports.len() > 1 {
                return Err(FlygDatabaseError::MoreThanOneResult);
            }

            // return the information about the airport which where requested
            return Ok(found_airports[0].clone());
        }
    }

    // it seems that we completely failed to query the database for the requested information
    Err(FlygDatabaseError::FailedToQueryDatabase)
}
