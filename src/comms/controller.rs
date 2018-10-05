use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};

pub const CONTENT_IP: &'static str = "localhost:1000";

service! {
    rpc get_user(payload: GetUserPayload) -> ResponseResult<UserPayload>;
    rpc add_user(payload: AddUserPayload) -> ResponseResult<UserPayload>;
    rpc edit_user(payload: EditUserPayload) -> ResponseResult<UserPayload>;
    rpc upload_avatar(payload: UploadAvatarPayload) -> ResponseResult<UserPayload>;

    rpc get_category(payload: GetCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc get_categories(payload: GetCategoriesPayload) -> ResponseResult<Vec<CategoryPayload>>;
    rpc add_category(payload: AddCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc edit_category(payload: EditCategoryPayload) -> ResponseResult<CategoryPayload>;
    rpc hide_category(payload: HideCategoryPayload) -> ResponseResult<CategoryPayload>;

    rpc get_thread(payload: GetThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc get_threads(payload: GetThreadsPayload) -> ResponseResult<Vec<ThreadPayload>>;
    rpc add_thread(payload: AddThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc edit_thread(payload: EditThreadPayload) -> ResponseResult<ThreadPayload>;
    rpc hide_thread(payload: HideThreadPayload) -> ResponseResult<ThreadPayload>;

    rpc get_comment(payload: GetCommentPayload) -> ResponseResult<CommentPayload>;
    rpc get_comments(payload: GetCommentsPayload) -> ResponseResult<Vec<CommentPayload>>;
    rpc add_comment(payload: AddCommentPayload) -> ResponseResult<CommentPayload>;
    rpc edit_comment(payload: EditCommentPayload) -> ResponseResult<CommentPayload>;
    rpc hide_comment(payload: HideCommentPayload) -> ResponseResult<CommentPayload>;

    rpc search(payload: SearchPayload) -> ResponseResult<SearchResultsPayload>;
}

