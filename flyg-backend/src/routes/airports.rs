use rocket::http::Status;
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
#[get("/airport/<icao_code>")]
pub fn get_airport_information(icao_code: String) -> Result<Json<AirportInformation>, Status> {
    // an ICAO code is always for letters/digits long, everything else seems to be an
    // invalid request
    if icao_code.len() != 4 {
        return Err(Status::BadRequest);
    }

    //
    Ok(Json(AirportInformation {
        icao_code,
        country_code: "DE".to_owned(),
        name: "Foo".to_owned(),
        position: Coordinates {
            latitude: 0.0,
            longitude: 0.0,
        },
        runways: vec![],
    }))
}

/// # Add a new airport to the database
///
/// Add information about the name, position and runways of an airport to the database, so
/// it can be used with Flyg.
#[post("/airport", data = "<_airport_information>")]
pub fn add_new_airport(_airport_information: Json<AirportInformation>) -> Status {
    Status::NotImplemented
}
