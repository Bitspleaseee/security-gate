use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;

use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr};
use tarpc::sync::client::{ClientExt, Options};

use datatypes::auth::requests::AuthRequest;
use datatypes::auth::responses::{AuthError, AuthSuccess};
use datatypes::content::responses::*;
use datatypes::error::ResponseError;
use datatypes::valid::token::USER_TOKEN_NAME;

use crate::comms::auth::SyncClient as AuthClient;
use crate::content::connect_to_controller;
use crate::JsonResponseResult;

lazy_static! {
    static ref AUTH_IP: SocketAddr = match std::env::var("AUTH_ADDRESS") {
        Ok(value) => value.parse().expect("Invalid formatted AUTH_ADDRESS"),
        Err(_) => {
            warn!("AUTH_ADDRESS is not set, using 'localhost:10001'");
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 10001)
        }
    };
}

// Connect to authentication service
pub fn connect_to_auth() -> Result<AuthClient, ResponseError> {
    AuthClient::connect(*AUTH_IP, Options::default()).map_err(|e| {
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
pub fn auth(
    mut cookies: Cookies,
    req: Option<Json<AuthRequest>>,
) -> JsonResponseResult<AuthSuccess> {
    use datatypes::auth::requests::AuthRequest::*;

    let req = req
        .ok_or(AuthError::InvalidCredentials)
        .map_err(|e| Json(e.into()))?; // If invalid request query.

    match req.into_inner() {
        Authenticate(p) => {
            let username = p.username.clone();
            connect_to_auth()
                .map_err(Json)?
                .authenticate(p)
                .map(|token| {
                    info!("User '{}' authenticated successfully", &username);

                    cookies.add_private(token.into());

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
                .deauthenticate(cookie.clone().into())
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
            let user = connect_to_auth().map_err(Json)?.register(p).map_err(|e| {
                error!("Auth: Unable to 'register': {:?}", e);
                Json(e.into())
            })?;
            debug!("Auth: user registerd successfully");

            let _user = connect_to_controller()
                .map_err(Json)?
                .add_user(user)
                .map_err(|e| {
                    error!("Controller: Unable to add user: {:?}", e);
                    Json(e.into())
                })?;

            debug!("Controller: Returning success from 'add_user' request");
            Ok(Json(AuthSuccess::UserRegistered))
        }
    }
}
