use rocket::Outcome;
use rocket::http::Status;
use rocket::http::Cookies;
use crate::auth::api::{Token, USER_TOKEN_NAME};
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

impl<'a, 'r> FromRequest<'a, 'r> for Token<'a> {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Token<'a>, AuthError> {
        let cookie = request.cookies()
        .get_private(USER_TOKEN_NAME);


         match cookie {
            Some(token) => {
                // Get a comment
                //let result = controller.get_comment(id);
                info!("Getting request with token {:?}", token);
                return Outcome::Success(Token(cookie));
            }
            None => {
                // Get all comments
                //let result = controller.get_all_comments();
                trace!("Getting all threads");
                return Outcome::Failure((Status::BadRequest, AuthError::MissingToken));
            }
        }
    }
}