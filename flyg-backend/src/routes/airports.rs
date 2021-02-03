use crate::database::airports::{
    get_information_for_icao_code, get_runway_information_for_icao_code, FlygDatabaseError,
};
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
        Ok(airport_infos) => {
            // since the airport was found, we now can query the airports which are associated
            // to it
            let raw_runway_information = match get_runway_information_for_icao_code(
                db_url.inner(),
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
                    }
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
