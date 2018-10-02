use datatypes::error::ResponseError;
use datatypes::auth::responses::AuthSuccess;
use datatypes::valid::fields::{PlainPassword, Username, Email};
use rocket::http::Cookie;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use std::borrow::Cow;
use std::convert::AsRef;
use std::convert::From;

pub const USER_TOKEN_NAME: &str = "user_token";

pub fn authenticate<'a>(
    _username: &Username,
    _password: &PlainPassword,
) -> Result<Token<'a>, ResponseError> {
    Ok(Token::new("placeholder"))
}

pub fn register<'a>(
    _username: &Username,
    _password: &PlainPassword,
    _email: &Email,
) -> Result<AuthSuccess, ResponseError> {
    Ok(AuthSuccess::UserRegistered)
}

pub fn deauthenticate<'a>(_token: impl Into<Token<'a>>) -> Result<(), ResponseError> {
    Ok(())
}

pub fn authenticated<'a>(_token: impl Into<Token<'a>>) -> Result<(), ResponseError> {
    Ok(())
}

pub struct Token<'a>(Cow<'a, str>);

impl<'a> Token<'a> {
    pub fn new(s: impl Into<Cow<'a, str>>) -> Token<'a> {
        Token(s.into())
    }
}

impl AsRef<str> for Token<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a Cookie<'a>> for Token<'a> {
    fn from(c: &'a Cookie<'a>) -> Self {
        Token::new(c.value())
    }
}

impl<'a> Into<Cookie<'a>> for Token<'a> {
    fn into(self) -> Cookie<'a> {
        Cookie::new(USER_TOKEN_NAME, self.0.into_owned())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Token<'a> {
    type Error = ResponseError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
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
                Outcome::Failure((Status::BadRequest, ResponseError::Unauthenticated))
            }
        }
    }
}
