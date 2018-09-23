use crate::auth::forms::{PlainPassword, Token, Username};

pub fn authenticate<'a>(_username: &Username<'a>, _password: &PlainPassword<'a>) -> Option<Token> {
    Some(Token::new("placeholder"))
}
