use super::schema::{airports, runway_airport_associations, runways};
use super::FlygDatabaseError;
use crate::database::types::GeogPoint;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Clone, Queryable, Identifiable, Associations)]
#[primary_key(id)]
#[table_name = "airports"]
pub struct Airport {
    pub id: i32,
    pub icao_code: String,
    pub last_update: NaiveDateTime,
    pub country: String,
    pub location: GeogPoint,
    pub name: String,
}

#[derive(Clone, Queryable, Identifiable, Associations)]
#[primary_key(id)]
#[table_name = "runways"]
pub struct Runway {
    pub id: i32,
    pub primary_direction: i32,
    pub secondary_direction: i32,
    pub primary_suffix: Option<String>,
    pub runway_length: i32,
    pub runway_width: i32,
}

#[derive(Clone, Queryable, Identifiable, Associations)]
#[primary_key(id)]
#[table_name = "runway_airport_associations"]
#[belongs_to(Airport)]
#[belongs_to(Runway)]
pub struct RunwayAirportAssociations {
    pub id: i32,
    pub airport_id: i32,
    pub runway_id: i32,
}

/// Query the database for all information for a specific airport.
///
/// This method can be used to query the database for all information
/// regarding a specific airport by its assigned ICAO code.
///
/// # Arguments
/// * `database_connection` - The connection to the database servers for the query.
/// * `icao_code_to_query_for` - The four letter ICAO code to query for.
///
/// # Errors
/// Will return `Err` if the requested airport information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
/// * `MoreThanOneResult` - Got more than one airport which should not happen since the ICAO code is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
pub fn get_information_for_icao_code(
    database_connection: &PgConnection,
    icao_code_to_query_for: &String,
) -> Result<Airport, FlygDatabaseError> {
    use super::schema::airports::dsl::{airports, icao_code};

    if let Ok(found_airports) = airports
        .filter(icao_code.eq(icao_code_to_query_for))
        .load::<Airport>(database_connection)
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

    // it seems that we completely failed to query the database for the requested information
    Err(FlygDatabaseError::FailedToQueryDatabase)
}

/// Query the database for the runway information for a specific airport.
///
/// This method can be used to query the database for all information
/// regarding the runways of a specific airport by its assigned ICAO code.
///
/// # Arguments
/// * `database_connection` - The connection to the database servers for the query.
/// * `icao_code_to_query_for` - The four letter ICAO code to query for.
///
/// # Errors
/// Will return `Err` if the requested runway information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
/// * `MoreThanOneResult` - Got more than one airport which should not happen since the ICAO code is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
pub fn get_runway_information_for_icao_code(
    database_connection: &PgConnection,
    icao_code_to_query_for: &String,
) -> Result<Vec<Runway>, FlygDatabaseError> {
    return match get_information_for_icao_code(database_connection, icao_code_to_query_for) {
        Ok(airport_infos) => {
            get_runway_information_for_airport(database_connection, &airport_infos)
        }
        Err(error) => Err(error),
    };
}

#[derive(Queryable)]
pub struct AirportByDistance {
    pub icao_code: String,
    #[allow(dead_code)]
    location: GeogPoint,
    pub distance: f64,
}

/// Query the database for the closest airports to a specific location.
///
/// The closest 3 airports to a specified location will be returned with their ICAO code as well
/// as the distance to the point in meters.
///
/// # Arguments
/// * `database_connection` - The connection to the database servers for the query.
/// * `latitude_reference` - The latitude to which the distance should be calculated.
/// * `longitude_reference` - The longitude to which the distance should be calculated.
///
/// # Errors
/// Will return `Err` if the requested runway information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
pub fn get_closest_airports_for_coordinates(
    database_connection: &PgConnection,
    latitude_reference: f32,
    longitude_reference: f32,
) -> Result<Vec<AirportByDistance>, FlygDatabaseError> {
    use super::schema::airports::dsl::{airports, icao_code, location};
    use diesel::dsl::sql;
    use diesel::sql_types::Double;
    use log::error;

    // try to query the closest airports
    return match airports
        .select((
            icao_code,
            location,
            sql::<Double>(&format!(
                "ST_Distance(location::geography, 'SRID=4326;POINT({long} {lat})'::geometry) AS distance",
                lat = latitude_reference,
                long = longitude_reference
            )),
        ))
        .order(sql::<Double>("distance ASC"))
        .limit(3)
        .load::<AirportByDistance>(database_connection)
    {
        Ok(result) => Ok(result),
        Err(error) => {
            error!("{:?}", error);
            Err(FlygDatabaseError::NoResults)
        }
    };
}

/// Query the database for the runway information for a specific airport.
///
/// This method can be used to query the database for all information
/// regarding the runways of a specific airport by its database entity.
///
/// # Arguments
/// * `database_connection` - The connection to the database servers for the query.
/// * `airport_to_query_for` - The database entity to use for querying the runway information.
///
/// # Errors
/// Will return `Err` if the requested runway information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
/// * `MoreThanOneResult` - Got more than one airport which should not happen since the ICAO code is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
pub fn get_runway_information_for_airport(
    database_connection: &PgConnection,
    airport_to_query_for: &Airport,
) -> Result<Vec<Runway>, FlygDatabaseError> {
    use super::schema::runway_airport_associations::dsl::runway_id;
    use super::schema::runways::dsl::{id, runways};
    use diesel::pg::expression::dsl::any;

    let found_runway_ids =
        RunwayAirportAssociations::belonging_to(airport_to_query_for).select(runway_id);
    if let Ok(found_runways) = runways
        .filter(id.eq(any(found_runway_ids)))
        .load::<Runway>(database_connection)
    {
        return Ok(found_runways);
    }

    // it seems that we completely failed to query the database for the requested information
    Err(FlygDatabaseError::FailedToQueryDatabase)
}
