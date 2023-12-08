use actix_web::web::Form;
use actix_web::{post, HttpResponse};
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

#[utoipa::path(
    post,
    path = "/v1/auth",
    request_body(content = AuthenticateUserParams, description = "", content_type = "multipart/form-data"),
    responses(
        (status = StatusCode::OK, description = "The user was successfully authorized and a token is returned.", body = AuthenticatedTokenResult, content_type = "application/json"),
        (status = StatusCode::BAD_REQUEST, description = "The supplied data seems to be in an invalid format and cannot be processed."),
        (status = StatusCode::UNAUTHORIZED, description = "The supplied credentials are not valid and the user does not get a token."),
    ),
)]
#[post("/v1/auth")]
pub async fn authenticate_user(_auth_data: Form<AuthenticateUserParams>) -> HttpResponse {
    HttpResponse::NotImplemented().into()
}
