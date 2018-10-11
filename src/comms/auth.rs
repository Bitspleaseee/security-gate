use datatypes::auth::requests::*;
use datatypes::auth::responses::*;
use datatypes::content::requests::*;
use datatypes::valid::ids::UserId;
use datatypes::valid::token::Token;

service! {
    rpc authenticate(payload: AuthPayload) -> Token | AuthError;
    rpc deauthenticate(payload: Token) -> () | AuthError;
    rpc register(payload: RegisterUserPayload) -> AddUserPayload | AuthError;
    rpc get_user(payload: Token) -> (UserId, Role) | AuthError;
    rpc set_user_role(payload: SetUserRolePayload) -> () | AuthError;
}
