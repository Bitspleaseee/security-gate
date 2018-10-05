use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub const CONTROLLER_IP: &'static str = "localhost:1000";

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ResponseError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ResponseError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ResponseError;
    rpc upload_avatar(payload: UploadAvatarPayload) -> UserPayload | ResponseError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ResponseError;
    rpc get_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ResponseError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ResponseError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ResponseError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ResponseError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ResponseError;
    rpc get_threads(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ResponseError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ResponseError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ResponseError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ResponseError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ResponseError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ResponseError;
    rpc get_comments(payload: GetCommentsPayload) -> Vec<CommentPayload> | ResponseError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ResponseError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ResponseError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ResponseError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ResponseError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ResponseError;

}

