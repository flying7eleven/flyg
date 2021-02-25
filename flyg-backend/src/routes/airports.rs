use crate::database::airports::{
    get_closest_airports_for_coordinates, get_information_for_icao_code,
    get_runway_information_for_icao_code,
};
use crate::database::FlygDatabaseError;
use crate::routes::auth::AuthenticatedUser;
use crate::FlygDatabaseConnection;
use rocket::http::{RawStr, Status};
use rocket::{get, post};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

/// Representation of a position on the earth.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    /// The longitudinal component of the coordinates.
    longitude: f32,
    /// The latitudinal component of the coordinates.
    latitude: f32,
}

/// Information about a single (both directions) runway.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunwayInformation {
    /// The length of the runway in meters.
    length: u16,
    /// The width of the runway in meters.
    width: u8,
    /// The primary direction of the runway in degree (magnetic north, e.g. 252).
    primary_direction: u16,
    /// The second direction of the runway in degree (magnetic north, e.g. 7).
    secondary_direction: u16,
    /// The suffix of the primary direction of the runway (if applicable, e.g. L, R, C, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_suffix: Option<String>,
}

/// Information about a specific airport.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirportInformation {
    /// The four-letter ICAO code of the represented airport.
    icao_code: String,
    /// The two-letter country code in which the airport is in.
    country_code: String,
    /// The official English name of the airport.
    name: String,
    /// The position of the airport in earth.
    position: Coordinates,
    /// A list of all runways of the airport.
    runways: Vec<RunwayInformation>,
}

/// A data tuple describing the identifier of an airport combined with the distance to some reference
/// point in nautical miles.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirportDistance {
    /// The four-letter ICAO code of the represented airport.
    icao_code: String,
    /// The distance to the airport in nautical miles.
    distance: f32,
}

/// TODO
///
/// # Arguments
/// * `database_connection` - TODO
/// * `latitude` - TODO
/// * `longitude` - TODO
///
/// # Errors
/// Will return `Err` if the requested runway information could not be found. The result
/// might be one of the following:
/// * `NoResults` - Could not find the airport with the given ICAO code.
/// * `MoreThanOneResult` - Got more than one airport which should not happen since the ICAO code is unique.
/// * `FailedToQueryDatabase` - Completely failed to query the database for the requested information.
#[get("/airports/closest?<latitude>&<longitude>")]
pub fn get_closest_airport_to_position(
    database_connection: FlygDatabaseConnection,
    latitude: &RawStr,
    longitude: &RawStr,
) -> Result<Json<Vec<AirportDistance>>, Status> {
    use log::error;

    // convert the latitude to a float value
    let latitude_as_float = match latitude.parse::<f32>() {
        Ok(value) => value,
        Err(error) => {
            error!(
                "Failed to convert the passed latitude ({}) to a float. The error was: {}",
                latitude, error
            );
            return Err(Status::BadRequest);
        }
    };

    // convert the longitude to a float value
    let longitude_as_float = match longitude.parse::<f32>() {
        Ok(value) => value,
        Err(error) => {
            error!(
                "Failed to convert the passed longitude ({}) to a float. The error was: {}",
                longitude, error
            );
            return Err(Status::BadRequest);
        }
    };

    // ensure the latitude and the longitude are in the correct range
    if latitude_as_float < -90.0
        || latitude_as_float > 90.0
        || longitude_as_float < -180.0
        || longitude_as_float > 180.0
    {
        error!(
            "The latitude ({}) and/or longitude ({}) are out of a valid range",
            latitude_as_float, longitude_as_float
        );
        return Err(Status::BadRequest);
    }

    let closest_airports = match get_closest_airports_for_coordinates(
        &*database_connection,
        latitude_as_float,
        longitude_as_float,
    ) {
        Ok(airports) => {
            let mut prepared_airports = vec![];
            for current_airport_tuple in airports {
                prepared_airports.push(AirportDistance {
                    icao_code: current_airport_tuple.0,
                    distance: current_airport_tuple.1,
                })
            }
            prepared_airports
        }
        Err(_) => {
            return Err(Status::BadRequest);
        }
    };

    //
    Ok(Json(closest_airports))
}

/// # Get information about a specific airport
///
/// Return information about a requested airport. The airport **must** be specified by using
/// its four-letter ICAO code.
#[get("/airports/<icao_code>")]
pub fn get_airport_information(
    database_connection: FlygDatabaseConnection,
    icao_code: String,
    _authenticated_user: AuthenticatedUser,
) -> Result<Json<AirportInformation>, Status> {
    use log::error;

    // an ICAO code is always for letters/digits long, everything else seems to be an
    // invalid request
    if icao_code.len() != 4 {
        error!("Invalid (too short) ICAO code provided: '{}'", icao_code);
        return Err(Status::BadRequest);
    }

    // try to query the information about the requested airport and return them
    return match get_information_for_icao_code(&*database_connection, &icao_code.to_uppercase()) {
        Ok(airport_infos) => {
            // since the airport was found, we now can query the airports which are associated
            // to it
            let raw_runway_information = match get_runway_information_for_icao_code(
                &*database_connection,
                &icao_code.to_uppercase(),
            ) {
                Ok(runways) => runways,
                Err(error) => {
                    return match error {
                        FlygDatabaseError::FailedToQueryDatabase => {
                            Err(Status::InternalServerError)
                        }
                        FlygDatabaseError::MoreThanOneResult => Err(Status::InternalServerError),
                        FlygDatabaseError::NoResults => Err(Status::NotFound),
                    };
                }
            };

            // all information where available and we can format the information, ...
            let runways = raw_runway_information
                .iter()
                .map(|input_element| RunwayInformation {
                    length: input_element.runway_length as u16,
                    width: input_element.runway_width as u8,
                    primary_direction: input_element.primary_direction as u16,
                    secondary_direction: input_element.secondary_direction as u16,
                    primary_suffix: input_element.primary_suffix.clone(),
                })
                .collect();

            // ... and return them to the requesting party
            return Ok(Json(AirportInformation {
                icao_code: airport_infos.icao_code,
                country_code: airport_infos.country,
                name: airport_infos.name,
                position: Coordinates {
                    latitude: airport_infos.latitude,
                    longitude: airport_infos.longitude,
                },
                runways,
            }));
        }
        Err(error) => match error {
            FlygDatabaseError::FailedToQueryDatabase => Err(Status::InternalServerError),
            FlygDatabaseError::MoreThanOneResult => Err(Status::InternalServerError),
            FlygDatabaseError::NoResults => Err(Status::NotFound),
        },
    };
}

/// # Add a new airport to the database
///
/// Add information about the name, position and runways of an airport to the database, so
/// it can be used with Flyg.
#[post("/airport", data = "<_airport_information>")]
pub fn add_new_airport(_airport_information: Json<AirportInformation>) -> Status {
    Status::NotImplemented
}
