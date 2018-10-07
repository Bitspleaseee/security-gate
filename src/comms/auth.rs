use datatypes::auth::requests::*;
use datatypes::auth::responses::*;
use datatypes::payloads::*;

pub const AUTH_IP: &str = "localhost:10001";

service! {
    rpc authenticate(payload: AuthPayload) -> Token | AuthError;
    rpc deauthenticate(payload: TokenPayload<EmptyPayload>) -> () | AuthError;
    rpc register(payload: RegisterUserPayload) -> () | AuthError;
    rpc get_user_role(payload: TokenPayload<EmptyPayload>) -> Role | AuthError;
}
