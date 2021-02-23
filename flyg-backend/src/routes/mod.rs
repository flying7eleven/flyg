use rocket::http::Status;
use rocket::response::content::Json;
use rocket::Request;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub mod airports;
pub mod auth;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

// TODO: add the default catcher as soon as Rocket supports it. It's currently (2021-02-23) in the
//       master and will be added in 0.5 I guess
// #[catch(default)]
// pub fn default_catcher(status: Status, _request: &Request) -> Json<ErrorResponse> {
//    Json(ErrorResponse{ code: status.code, message: status.reason.to_string() })
// }

#[catch(400)]
pub fn catcher_bad_request(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 400,
        message: "Bad request".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}

#[catch(401)]
pub fn catcher_unauthorized(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 401,
        message: "Unauthorized".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}

#[catch(403)]
pub fn catcher_forbidden(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 403,
        message: "Forbidden".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}

#[catch(404)]
pub fn catcher_not_found(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 404,
        message: "Not found".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}

#[catch(405)]
pub fn catcher_method_not_allowed(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 405,
        message: "Method not allowed".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}

#[catch(500)]
pub fn catcher_internal_server_error(_: &Request) -> Json<String> {
    let error_object = ErrorResponse {
        code: 500,
        message: "Internal server error".to_string(),
    };
    Json(to_string(&error_object).unwrap())
}
