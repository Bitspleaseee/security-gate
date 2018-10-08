use datatypes::auth::requests::*;
use datatypes::auth::responses::*;
use datatypes::content::requests::*;
use datatypes::valid::token::Token;

service! {
    rpc authenticate(payload: AuthPayload) -> Token | AuthError;
    rpc deauthenticate(payload: Token) -> () | AuthError;
    rpc register(payload: RegisterUserPayload) -> AddUserPayload | AuthError;
    rpc get_user_role(payload: Token) -> Role | AuthError;
}
