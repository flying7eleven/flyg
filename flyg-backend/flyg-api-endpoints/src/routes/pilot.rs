use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// All information stored for the pilot who is currently authenticated.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct PilotInformationResult {
    /// The first name of the pilot.
    #[schema(example = "John")]
    pub first_name: String,
    /// The last name of the pilot.
    #[schema(example = "Doe")]
    pub last_name: String,
}

#[utoipa::path(
    get,
    context_path = "/v1",
    responses(
        (status = StatusCode::OK, description = "The information about the currently authenticated pilot.", body = PilotInformationResult, content_type = "application/json"),
        (status = StatusCode::FORBIDDEN, description = "The token which was used seems not to be valid (anymore)."),
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/pilot")]
pub async fn get_pilot_info() -> Result<Json<PilotInformationResult>, Status> {
    Err(Status::NotImplemented)
}
