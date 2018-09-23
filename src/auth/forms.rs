use crate::auth::data::{PlainPassword, Username};

#[derive_FromForm]
pub struct LoginForm<'v> {
    username: Option<Username<'v>>,
    password: Option<PlainPassword<'v>>,
}

impl<'v> LoginForm<'v> {
    pub fn username(&self) -> Option<&Username<'v>> {
        self.username.as_ref()
    }
    pub fn password(&self) -> Option<&PlainPassword<'v>> {
        self.password.as_ref()
    }
}

#[derive(Fail, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoginFormError {
    #[fail(display = "invalid username (see USERNAME_REGEX for criteria)")]
    InvalidUsername,
    #[fail(display = "invalid password (see PASSWORD_REGEX for criteria)")]
    InvalidPassword,
}
