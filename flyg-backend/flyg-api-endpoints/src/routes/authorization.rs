use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The expected form data which has to be send to an API endpoint to authenticate a user
/// against the API of the corresponding Flyg instance.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct AuthenticateUserParams {
    /// The user name which identifies the user on the corresponding Flyg instance.
    #[schema(example = "johndoe")]
    pub username: String,
    /// The password associated with the given user name on the corresponding Flyg instance.
    #[schema(example = "simplePassword")]
    pub password: String,
}

/// The information returned by the authentication endpoints which contain the required information
/// to access all API endpoints which require authentication.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct AuthenticatedTokenResult {
    /// The access token is used to access all API endpoints by the user.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5...")]
    pub access_token: String,
    /// The refresh token can be used to get a new `access_token` before the `access_token` loses its validity.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5...")]
    pub refresh_token: String,
}

/// The required information for refreshing the `access_token` and getting a new `refresh_token`.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RefreshTokenRequest {
    /// The refresh token which should be used to create a new token pair.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5...")]
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    context_path = "/v1",
    request_body(content = AuthenticateUserParams, description = "The username and the passwort which should be used for authentication.", content_type = "application/json"),
    responses(
        (status = StatusCode::OK, description = "The user was successfully authorized and a token is returned.", body = AuthenticatedTokenResult, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "The supplied data seems to be in an invalid format and cannot be processed."),
        (status = StatusCode::FORBIDDEN, description = "The supplied credentials are not valid and the user does not get a token."),
    ),
)]
#[post("/authorization/login", data = "<auth_data>")]
pub async fn authenticate_user(
    auth_data: Json<AuthenticateUserParams>,
) -> Result<Json<AuthenticatedTokenResult>, Status> {
    Err(Status::NotImplemented)
}

#[utoipa::path(
    post,
    context_path = "/v1",
    request_body(content = RefreshTokenRequest, description = "The refresh token which should be used to generate a new token pair.", content_type = "application/json"),
    responses(
        (status = StatusCode::OK, description = "The refresh token was used to create a new token pair which can be used for all further requests.", body = AuthenticatedTokenResult, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "The supplied data seems to be in an invalid format and cannot be processed."),
        (status = StatusCode::FORBIDDEN, description = "The supplied refresh token was not valid (anymore)."),
    ),
)]
#[post("/authorization/token", data = "<refresh_data>")]
pub async fn refresh_token(
    refresh_data: Json<RefreshTokenRequest>,
) -> Result<Json<AuthenticatedTokenResult>, Status> {
    Err(Status::NotImplemented)
}
