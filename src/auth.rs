use rocket::http::Cookies;
use rocket_contrib::Json;

use tarpc::sync::client::{ClientExt, Options};

use datatypes::auth::requests::AuthRequest;
use datatypes::auth::responses::{AuthSuccess, AuthError};
use datatypes::error::ResponseError;
use datatypes::payloads::TokenPayload;
use datatypes::valid::token::USER_TOKEN_NAME;
use std::net::SocketAddr;

use crate::comms::auth::SyncClient as AuthClient;

use crate::JsonResponseResult;


lazy_static! {
    static ref AUTH_IP: SocketAddr =
        match std::env::var("AUTH_ADDRESS") {
            Ok(value) => value.as_str(),
            Err(_) => {
                warn!("AUTH_ADDRESS is not set, using 'localhost:10001'");
                "localhost:10001"
            }.parse().expect("Invalid formatted AUTH_ADDRESS")
    };
}

// Connect to authentication service
pub fn connect_to_auth() -> Result<AuthClient, ResponseError> {
    AuthClient::connect(AUTH_IP, Options::default()).map_err(|e| {
        error!("Unable to connect to authentication-service: {:?}", e);
        ResponseError::InternalServerError
    })
}

/// Authenticate or deauthenticate user
///
/// Uses a strict JSON format to conway actions.
///
/// # Request objects
///
/// ## Authentication request
///
/// A request to authenticate with the system. Returns a special session cookie
/// which will be used
/// for authorization.
///
/// ```json
/// {
///     "type": "AUTHENTICATE",
///     "payload": {
///         "username": "my_username",
///         "password": "secret"
///     }
/// }
/// ```
///
/// ## Deauthentication request
///
/// A request to deauthenticate with the service. Will remove the session cookie.
///
/// ```json
/// {
///     "type": "DEAUTHENTICATE",
///     "payload": {}
/// }
/// ```
///
/// # Response objects
///
/// Returns a similar shaped JSON object which containes the outcome of the request.
///
/// ```json
/// {
///     "type": "AUTHENTICATED",
/// }
/// ```
///
/// The possible types are defined in [`AuthError`](../responses/enum.AuthError.html)
#[post("/auth", format = "application/json", data = "<req>")]
pub fn auth(mut cookies: Cookies, req: Option<Json<AuthRequest>>) -> JsonResponseResult<AuthSuccess> {
    use datatypes::auth::requests::AuthRequest::*;

    let req = req.ok_or(AuthError::InvalidCredentials).map_err(|e| Json(e.into()))?;           // If invalid request query.

    match req.into_inner() {
        Authenticate(p) => {
            let username = p.username.clone();
            connect_to_auth()
                .map_err(Json)?
                .authenticate(p)
                .map(|_| {
                    info!("User '{}' authenticated successfully", &username);
                    Json(AuthSuccess::Authenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        }
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(Json(ResponseError::Unauthenticated))?;

            connect_to_auth()
                .map_err(Json)?
                .deauthenticate(TokenPayload::new(None, cookie.clone()))
                .map(|_| {
                    info!("User deauthenticated successfully");
                    cookies.remove_private(cookie);
                    Json(AuthSuccess::Deauthenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        }
        RegisterUser(p) => {
            let username = p.username.clone();
            connect_to_auth()
                .map_err(Json)?
                .register(p)
                .map(|_| {
                    info!("User '{}' registered successfully", &username);
                    Json(AuthSuccess::UserRegistered)
                }).map_err(|e| {
                    error!("Unable to 'register': {:?}", e);
                    Json(e.into())
                })
        }
    }
}
