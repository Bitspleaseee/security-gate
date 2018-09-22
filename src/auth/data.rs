
use std::convert::{TryFrom, TryInto};

struct User<'v> {
    id: u32, // TODO make a ID type
    username: Username,
    password: Password,
}

/// The regex which vertifies that a username is formatted correctly
const USERNAME_REGEX: &'static str = "^[a-zA-Z0-9_-]{4,10}$";

/// The regex which vertifies that a password is formatted correctly
///
/// NB! Should perhaps be changed to accomodate a hashed password instead?
const PASSWORD_REGEX: &'static str = "^[\w]{8,64}$";

/// A valid username based on a regex
struct Username<'a>(&'a str);

impl<'v> FromFormValue<'v> for Username<'v> {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Username<'v>, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}

impl<'v> TryFrom<&'v str> for Username {
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

/// A valid password based on a regex
struct Password<'a>(&'a str);

impl<'v> FromFormValue<'v> for Password<'v> {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Password<'v>, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}

impl<'v> TryFrom<&'v str> for Password<'v> {
    type Error = &'v str;

    fn try_from(s: &'v str) -> Result<Password<'v>, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(PASSWORD_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(Password(s))
        } else {
            Err(s)
        }
    }

}

