use crate::auth::requests::{PlainPassword, Username};
use crate::auth::responses::AuthError;
use rocket::http::Cookie;
use std::borrow::Cow;
use std::convert::AsRef;
use std::convert::From;

pub const USER_TOKEN_NAME: &str = "user_token";

pub fn authenticate<'a>(
    _username: &Username,
    _password: &PlainPassword,
) -> Result<Token<'a>, AuthError> {
    Ok(Token::new("placeholder"))
}

pub fn deauthenticate<'a>(_token: impl Into<Token<'a>>) -> Result<(), AuthError> {
    Ok(())
}

pub fn authenticated<'a>(_token: impl Into<Token<'a>>) -> Result<(), AuthError> {
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
