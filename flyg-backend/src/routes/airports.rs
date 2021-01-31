use crate::database::airports::{get_information_for_icao_code, FlygDatabaseError};
use rocket::http::Status;
use rocket::{get, post, State};
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
    /// The first (main) direction of the airport (e.g. 25).
    direction_one: u8,
    /// The second direction of the airport (e.g. 7).
    direction_two: u8,
}

/// Information about a specific airport.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirportInformation {
    /// The four-letter ICAO code of the represented airport
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

/// # Get information about a specific airport
///
/// Return information about a requested airport. The airport **must** be specified by using
/// its four-letter ICAO code.
#[get("/airports/<icao_code>")]
pub fn get_airport_information(
    db_url: State<String>,
    icao_code: String,
) -> Result<Json<AirportInformation>, Status> {
    // an ICAO code is always for letters/digits long, everything else seems to be an
    // invalid request
    if icao_code.len() != 4 {
        return Err(Status::BadRequest);
    }

    // try to query the information about the requested airport and return them
    return match get_information_for_icao_code(db_url.inner(), &icao_code.to_uppercase()) {
        Ok(airport_infos) => Ok(Json(AirportInformation {
            icao_code: airport_infos.icao_code,
            country_code: airport_infos.country,
            name: airport_infos.name,
            position: Coordinates {
                latitude: airport_infos.latitude,
                longitude: airport_infos.longitude,
            },
            runways: vec![],
        })),
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
