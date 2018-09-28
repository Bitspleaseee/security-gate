use rocket::Outcome;
use rocket::http::Status;
use rocket::http::Cookies;
use crate::auth::api::{Token, USER_TOKEN_NAME};
use rocket::request::{self, Request, FromRequest};
use std::convert::Into;
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

impl<'a, 'r> FromRequest<'a, 'r> for Token<'a> {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token<'a>, AuthError> {
        let cookie = request.cookies().get_private(USER_TOKEN_NAME);

         match cookie {
            Some(cookie_content) => {
                // Found a token
                info!("Getting request with token {:?}", cookie_content);
                Outcome::Success(Token::new(cookie_content.value().to_owned()))
            }
            None => {
                // Did not found any token
                info!("Did not found any token.");
                Outcome::Failure((Status::BadRequest, AuthError::MissingToken))
            }
        }
    }
}
