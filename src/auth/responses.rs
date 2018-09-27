use rocket::Outcome;
use rocket::http::Status;
use rocket::http::Cookies;
use crate::auth::api::{authenticated, USER_TOKEN_NAME};
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthSuccess {
    Authenticated,
    Deauthenticated,
}

#[derive(Fail, Debug, Serialize, Deserialize)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthError {
    #[fail(display = "token missing from cookies")]
    MissingToken,
    #[fail(display = "invalid username")]
    InvalidUsername,
    #[fail(display = "invalid password")]
    InvalidPassword,
}

struct Autenticated(u32);

impl<'a, 'r> FromRequest<'a, 'r> for Autenticated {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Autenticated, AuthError> {
        let cookie = request.cookies()
        .get_private(USER_TOKEN_NAME);
        //TODO: Fix errormessage:
        //.ok_or(AuthError::MissingToken)
        //.map_err(Json)?;

        let result = authenticated(&cookie);

        if result.is_ok() {
            return Outcome::Success(Autenticated(result.id));
        } else {
            return Outcome::Failure((Status::BadRequest, AuthError::MissingToken));
        }
    }
}

