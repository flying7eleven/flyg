use crate::FlygDatabaseConnection;
use jsonwebtoken::Algorithm;
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::post;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

/// Wrap the login information (username and password) provided by the user
/// when requesting a new token.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInformation {
    /// The username (the email address) of the user.
    username: String,
    /// The password for the login request.
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: Vec<String>,
    exp: usize,
    iat: usize,
    iss: String,
    nbf: usize,
    sub: String,
}

/// TODO
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// The access token to use for API requests.
    access_token: String,
}

// define some static values temporary required in this module
lazy_static! {
    /// The time in seconds a token is valid.
    static ref TOKEN_LIFETIME_IN_SECONDS: usize = 60 * 60;

    /// The secret used for signing the token.
    static ref TOKEN_SECRET: &'static str = "secret"; // TODO: move that into a configuration file before going live!!!
}

/// # Get an access token to access the API
///
/// This method will return a new access token for the given `subject`. It will *not* check
/// if the subject is authorized to get a token or of the subject is even valid. This has to
/// be done from the calling party!
fn get_token_for_user(subject: &String) -> Option<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use log::error;

    // get the issuing time for the token
    let token_issued_at = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as usize,
        Err(error) => {
            error!(
                "Could not get the issuing time for the token. The error was: {}",
                error
            );
            return None;
        }
    };

    // calculate the time when the token expires
    let token_expires_at = token_issued_at + 1 + *TOKEN_LIFETIME_IN_SECONDS;

    // define the content of the actual token
    let token_claims = Claims {
        aud: vec!["https://www.flyg.link".to_string()],
        exp: token_expires_at,
        iat: token_issued_at,
        iss: "flyg-backend".to_string(),
        nbf: token_issued_at + 1,
        sub: subject.clone(),
    };

    // generate a new JWT for the supplied header and token claims. if we were sucessfull, return
    // the token
    let header = Header::new(Algorithm::HS512);
    if let Ok(token) = encode(
        &header,
        &token_claims,
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    ) {
        return Some(token);
    }

    // if we fail, return None
    None
}

/// # Request a new access token
///
/// The user can use this method to request a new access token by supplying
/// the username and password for a user.
#[post("/auth/token", data = "<login_information>")]
pub fn get_login_token(
    login_information: Json<LoginInformation>,
) -> Result<Json<TokenResponse>, Status> {
    if let Some(token) = get_token_for_user(&login_information.username) {
        return Ok(Json(TokenResponse {
            access_token: token,
        }));
    }
    Err(Status::InternalServerError)
}
