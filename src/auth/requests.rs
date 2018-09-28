use crate::auth::responses::AuthError;
use regex::Regex;
use std::convert::TryFrom;
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum AuthRequest {
    Authenticate(#[serde(rename = "payload")] AuthPayload),
    Deauthenticate(#[serde(rename = "payload")] EmptyPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthPayload {
    #[serde(rename = "username")]
    pub raw_username: String,
    #[serde(rename = "password")]
    pub raw_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyPayload {}

/// The regex which vertifies that a username is formatted correctly
const USERNAME_REGEX: &str = "^[a-zA-Z0-9_-]{4,10}$";

/// The regex which vertifies that a password is formatted correctly
const PASSWORD_REGEX: &str = "^[\\w]{8,64}$";

/// A valid (well formatted) username
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Username<'a>(&'a str);

impl<'a> TryFrom<&'a str> for Username<'a> {
    type Error = AuthError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(USERNAME_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(Username(s))
        } else {
            Err(AuthError::InvalidUsername)
        }
    }
}

impl<'a> Display for Username<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct PlainPassword<'a>(&'a str);

impl<'a> TryFrom<&'a str> for PlainPassword<'a> {
    type Error = AuthError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(PASSWORD_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(PlainPassword(s))
        } else {
            Err(AuthError::InvalidPassword)
        }
    }
}