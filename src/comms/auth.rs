use datatypes::auth::requests::*;
use datatypes::auth::responses::*;
use datatypes::payloads::*;
use datatypes::valid::token::Token;

service! {
    rpc authenticate(payload: AuthPayload) -> Token | AuthError;
    rpc deauthenticate(payload: TokenPayload<EmptyPayload>) -> () | AuthError;
    rpc register(payload: RegisterUserPayload) -> () | AuthError;
    rpc get_user_role(payload: TokenPayload<EmptyPayload>) -> Role | AuthError;
}
