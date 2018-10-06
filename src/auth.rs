use rocket::http::Cookies;
use rocket_contrib::Json;

use datatypes::auth::requests::AuthRequest;
use datatypes::auth::responses::AuthSuccess;
use datatypes::error::ResponseError;
use datatypes::valid::token::{Token, USER_TOKEN_NAME};

use crate::JsonResponseResult;

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
            let token = Token::new("testt");
            cookies.add_private(token.into());
            info!("User '{}' authenticated successfully", &p.username);
            Ok(AuthSuccess::Authenticated)
        }
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(Json(ResponseError::Unauthenticated))?;

            info!("User deauthenticated successfully");
            cookies.remove_private(cookie);
            Ok(AuthSuccess::Deauthenticated)
        }
        RegisterUser(p) => {
            info!("User '{}' authenticated successfully", &p.username);
            Ok(AuthSuccess::UserRegistered)
        }
    }.map(Json)
    .map_err(Json)
}
