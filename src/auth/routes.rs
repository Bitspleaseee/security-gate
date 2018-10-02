use crate::auth::api::{authenticate, register, deauthenticate, USER_TOKEN_NAME};
use crate::JsonResult;
use datatypes::auth::requests::{AuthRequest};
use datatypes::auth::responses::{AuthSuccess, AuthError};
use datatypes::error::ResponseError;
use rocket::http::Cookies;
use rocket_contrib::Json;

/// Authenticate or deauthenticate user
///
/// Uses a strict JSON format to conway actions.
///
/// # Request objects
///
/// ## Authentication request
///
/// A request to authenticate with the system. Returns a special session cookie which will be used
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
pub fn auth(mut cookies: Cookies, req: Json<AuthRequest>) -> JsonResult<AuthSuccess> {
    use datatypes::auth::requests::AuthRequest::*;
    match *req {
        Authenticate(ref p) => authenticate(p.username(), p.password()).map(|token| {
            cookies.add_private(token.into());
            info!("user '{}' authenticated successfully", p.username());
            AuthSuccess::Authenticated
        }),
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(Json(ResponseError::Unauthenticated))?;

            deauthenticate(&cookie).map(|_| {
                info!("user deauthenticated successfully");
                cookies.remove_private(cookie);
                AuthSuccess::Deauthenticated
            })
        },
        RegisterUser(ref p) => register(p.username(), p.password(), p.email()).map(|token| {
            info!("user '{}' authenticated successfully", p.username());
            AuthSuccess::UserRegistered
        }),
        _ => unimplemented!(),
    }.map(Json)
    .map_err(Json)
}

/*
/// Register user.
#[post("/register", format = "application/json", data = "<req>")]
fn register(req: Json<AuthRequest>) -> JsonResult<AuthSuccess> {
    if let Ok(username) = req.username {
        // result = auth.register(req);
        trace!("sent request to register new user with username: {}", req.username);
        Err(AuthError::InvalidUsername).map(Json).map_err(Json)
    } else {
        trace!("failed regestering new user with username: {}", req.username);
        Err(AuthError::InvalidUsername).map(Json).map_err(Json)
  }
}*/