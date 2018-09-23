use regex::Regex;
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Display};

/// The regex which vertifies that a username is formatted correctly
const USERNAME_REGEX: &str = "^[a-zA-Z0-9_-]{4,10}$";

/// The regex which vertifies that a password is formatted correctly
const PASSWORD_REGEX: &str = "^[\\w]{8,64}$";

/// A valid (well formatted) username
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Username<'a>(&'a str);

impl<'v> FromFormValue<'v> for Username<'v> {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Username<'v>, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}

impl<'v> TryFrom<&'v str> for Username<'v> {
    type Error = &'v str;

    fn try_from(s: &'v str) -> Result<Username<'v>, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(USERNAME_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(Username(s))
        } else {
            Err(s)
        }
    }
}

impl<'v> Display for Username<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A valid (well formatted) plaintext password
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct PlainPassword<'a>(&'a str);

impl<'v> FromFormValue<'v> for PlainPassword<'v> {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<PlainPassword<'v>, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}

impl<'v> TryFrom<&'v str> for PlainPassword<'v> {
    type Error = &'v str;

    fn try_from(s: &'v str) -> Result<PlainPassword<'v>, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(PASSWORD_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(PlainPassword(s))
        } else {
            Err(s)
        }
    }
}
