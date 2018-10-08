use datatypes::content::requests::*;
use datatypes::content::responses::*;

pub static mut CONTROLLER_IP: &str = "localhost:10000";

service! {
    rpc get_user(payload: GetUserPayload) -> UserPayload | ContentError;
    rpc add_user(payload: AddUserPayload) -> UserPayload | ContentError;
    rpc edit_user(payload: EditUserPayload) -> UserPayload | ContentError;

    rpc get_category(payload: GetCategoryPayload) -> CategoryPayload | ContentError;
    rpc get_all_categories(payload: GetHiddenPayload) -> Vec<CategoryPayload> | ContentError;
    rpc add_category(payload: AddCategoryPayload) -> CategoryPayload | ContentError;
    rpc edit_category(payload: EditCategoryPayload) -> CategoryPayload | ContentError;
    rpc hide_category(payload: HideCategoryPayload) -> CategoryPayload | ContentError;

    rpc get_thread(payload: GetThreadPayload) -> ThreadPayload | ContentError;
    rpc get_threads_in_category(payload: GetThreadsPayload) -> Vec<ThreadPayload> | ContentError;
    rpc get_all_threads(payload: GetHiddenPayload) -> Vec<ThreadPayload> | ContentError;
    rpc add_thread(payload: AddThreadPayload) -> ThreadPayload | ContentError;
    rpc edit_thread(payload: EditThreadPayload) -> ThreadPayload | ContentError;
    rpc hide_thread(payload: HideThreadPayload) -> ThreadPayload | ContentError;

    rpc get_comment(payload: GetCommentPayload) -> CommentPayload | ContentError;
    rpc get_comments_in_thread(payload: GetCommentsPayload) -> Vec<CommentPayload> | ContentError;
    rpc get_all_comments(payload: GetHiddenPayload) -> Vec<CommentPayload> | ContentError;
    rpc add_comment(payload: AddCommentPayload) -> CommentPayload | ContentError;
    rpc edit_comment(payload: EditCommentPayload) -> CommentPayload | ContentError;
    rpc hide_comment(payload: HideCommentPayload) -> CommentPayload | ContentError;

    rpc search(payload: SearchPayload) -> SearchResultsPayload | ContentError;
}
