use crate::{FlygDatabaseConnection, FlygSettings};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{post, Request, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
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
    exp: usize,
    iat: usize,
    nbf: usize,
    sub: String,
    admin: bool,
}

/// The representation of an authenticated user. As soon as this is included in the parameters
/// of a route, the call can be just made with an valid token in the header.
pub struct AuthenticatedUser {
    email_address: String,
}

/// TODO
#[derive(Debug)]
pub enum AuthorizationError {
    /// TODO
    MissingAuthorizationHeader,
    /// TODO
    MalformedAuthorizationHeader,
    /// TODO
    InvalidToken,
    ///
    NoDecodingKey,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = AuthorizationError;

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, AuthorizationError> {
        use jsonwebtoken::decode;
        use log::error;

        //
        let maybe_authorization_header = request.headers().get_one("Authorization");
        match maybe_authorization_header {
            Some(maybe_authorization) => {
                // split the token type from the actual token... there have to be two parts
                let authorization_information =
                    maybe_authorization.split(" ").collect::<Vec<&str>>();
                if authorization_information.len() != 2 {
                    error!("It seems that the authorization header is malformed. There were 2 parts expected but we got {}", authorization_information.len());
                    return Outcome::Failure((
                        Status::Forbidden,
                        AuthorizationError::MalformedAuthorizationHeader,
                    ));
                }

                // ensure that the token type is marked as 'bearer' token
                if authorization_information[0].to_lowercase() != "bearer" {
                    error!("It seems that the authorization header is malformed. We expected as token type 'bearer' but got '{}'", authorization_information[0].to_lowercase());
                    return Outcome::Failure((
                        Status::Forbidden,
                        AuthorizationError::MalformedAuthorizationHeader,
                    ));
                }

                // specify the parameter for the validation of the token
                let mut validation_parameter = Validation::new(Algorithm::RS256);
                validation_parameter.leeway = 5; // allow a time difference of max. 5 seconds
                validation_parameter.validate_exp = true;
                validation_parameter.validate_nbf = true;

                // get the current flyg configuration (for the public key)
                let flyg_configuration = match request.guard::<State<FlygSettings>>() {
                    Outcome::Success(state) => state,
                    Outcome::Failure(_) => {
                        error!(
                            "Could not get the current configuration for extracting the toking signing key"
                        );
                        return Outcome::Failure((
                            Status::Forbidden,
                            AuthorizationError::NoDecodingKey,
                        ));
                    }
                    Outcome::Forward(forward) => return Outcome::Forward(forward),
                };

                // get the 'validation' key for the token
                let decoding_key = match DecodingKey::from_rsa_pem(
                    flyg_configuration.public_key.as_bytes(),
                ) {
                    Ok(key) => key,
                    Err(error) => {
                        error!("Could not get the public key for the token validation. The error was: {}", error);
                        return Outcome::Failure((
                            Status::Forbidden,
                            AuthorizationError::NoDecodingKey,
                        ));
                    }
                };

                // verify the validity of the token supplied in the header
                let decoded_token = match decode::<Claims>(
                    authorization_information[1],
                    &decoding_key,
                    &validation_parameter,
                ) {
                    Ok(token) => token,
                    Err(error) => {
                        error!(
                            "The supplied token seems to be invalid. The error was: {}",
                            error
                        );
                        return Outcome::Failure((
                            Status::Forbidden,
                            AuthorizationError::InvalidToken,
                        ));
                    }
                };

                // if we reach this step, the validation was successful, and we can allow the user to
                // call the route
                return Outcome::Success(AuthenticatedUser {
                    email_address: decoded_token.claims.sub,
                });
            }
            _ => {
                error!("No authorization header could be found for an authenticated route!");
                Outcome::Failure((
                    Status::Forbidden,
                    AuthorizationError::MissingAuthorizationHeader,
                ))
            }
        }
    }
}

/// The response for a request for a new token.
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
}

/// # Get an access token to access the API
///
/// This method will return a new access token for the given `subject`. It will *not* check
/// if the subject is authorized to get a token or of the subject is even valid. This has to
/// be done from the calling party!
fn get_token_for_user(subject: &String, is_admin: bool, private_key: &String) -> Option<String> {
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
        exp: token_expires_at,
        iat: token_issued_at,
        nbf: token_issued_at + 1,
        sub: subject.clone(),
        admin: is_admin,
    };

    // get the signing key for the token
    let encoding_key = match EncodingKey::from_rsa_pem(private_key.as_bytes()) {
        Ok(key) => key,
        Err(error) => {
            error!(
                "Could not get the signing key for the token. The error was: {}",
                error
            );
            return None;
        }
    };

    // generate a new JWT for the supplied header and token claims. if we were sucessfull, return
    // the token
    let header = Header::new(Algorithm::RS256);
    if let Ok(token) = encode(&header, &token_claims, &encoding_key) {
        return Some(token);
    }

    // if we fail, return None
    None
}

/// # Request the public key for validating the tokens
///
/// The user (or better the client) can use this method to request the public key
/// which can be used to validate the tokens offered by this backend.
#[get("/auth/signature")]
pub fn get_public_key_for_signature_validation(configuration: State<FlygSettings>) -> String {
    configuration.public_key.clone()
}

/// # Request a new access token
///
/// The user can use this method to request a new access token by supplying
/// the username and password for a user.
#[post("/auth/token", data = "<login_information>")]
pub fn get_login_token(
    database_connection: FlygDatabaseConnection,
    login_information: Json<LoginInformation>,
    configuration: State<FlygSettings>,
) -> Result<Json<TokenResponse>, Status> {
    use crate::database::auth::get_user_record;
    use bcrypt::verify;
    use log::error;

    // try to get the user record for the supplied email address / username
    let user = match get_user_record(&*database_connection, &login_information.username) {
        Ok(user) => user,
        Err(error) => {
            // ensure that we know what happened
            error!(
                "Could not get the user record for '{}'. The error was: {:?}",
                login_information.username, error
            );

            // just slow down the process to prevent easy checking if a user name exists or not
            let _ = verify(
                "some_password",
                "$2y$12$7xMzqvnHyizkumZYpIRXheGMAqDKVo8HKtpmQSn51JUfY0N2VN4ua",
            );

            // finally we can tell teh user that he/she is not authorized
            return Err(Status::Unauthorized);
        }
    };

    // check if the supplied password matches the one we stored in the database using the same bcrypt
    // parameters
    match verify(&login_information.password, user.password.as_str()) {
        Ok(is_password_correct) => {
            if !is_password_correct {
                return Err(Status::Unauthorized);
            }
        }
        Err(error) => {
            error!("Could not verify the supplied password with the one stored in the database. The error was: {}", error);
            return Err(Status::InternalServerError);
        }
    }

    // if we get here, the we ensured that the user is known and that the supplied password
    // was valid, we can generate a new access token and return it to the calling party
    if let Some(token) = get_token_for_user(
        &login_information.username,
        user.is_admin,
        &configuration.private_key,
    ) {
        return Ok(Json(TokenResponse {
            access_token: token,
        }));
    }

    // it seems that we failed to generate a valid token, this should never happen, something
    // seems to be REALLY wrong
    Err(Status::InternalServerError)
}
