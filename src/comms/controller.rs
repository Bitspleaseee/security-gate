use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;

service! {
    rpc add_user(user: AddUserPayload) -> Result<UserPayload, ResponseError>;
    rpc content_request(request: ContentRequest) -> Result<ContentSuccess, ResponseError>;
}

