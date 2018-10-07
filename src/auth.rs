use rocket::http::Cookies;
use rocket_contrib::Json;

use tarpc::sync::client::{ClientExt, Options};

use datatypes::auth::requests::AuthRequest;
use datatypes::auth::responses::AuthSuccess;
use datatypes::error::ResponseError;
use datatypes::valid::token::{Token, USER_TOKEN_NAME};
use datatypes::payloads::TokenPayload;
use datatypes::payloads::EmptyPayload;

use crate::comms::auth::SyncClient as AuthClient;
use crate::comms::auth::AUTH_IP;

use crate::JsonResponseResult;

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
pub fn auth(mut cookies: Cookies, req: Json<AuthRequest>) -> JsonResponseResult<AuthSuccess> {
    use datatypes::auth::requests::AuthRequest::*;
    match req.into_inner() {
        Authenticate(p) => {
            connect_to_auth()
                .map_err(Json)?
                .authenticate(p)
                .map(|v| {
                    info!("User '{}' authenticated successfully", &p.username);
                    Json(AuthSuccess::Authenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        },
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(Json(ResponseError::Unauthenticated))?;

            connect_to_auth()
                .map_err(Json)?
                .deauthenticate(TokenPayload::new(EmptyPayload, cookie))
                .map(|v| {
                    info!("User deauthenticated successfully");
                    cookies.remove_private(cookie);
                    Json(AuthSuccess::Deauthenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        },
        RegisterUser(p) => {
            connect_to_auth()
                .map_err(Json)?
                .register(p)
                .map(|v| {
                    info!("User '{}' registered successfully", &p.username);
                    Json(AuthSuccess::UserRegistered)
                }).map_err(|e| {
                    error!("Unable to 'register': {:?}", e);
                    Json(e.into())
                })
        }
    }
}
