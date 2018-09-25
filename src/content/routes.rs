use super::data::Category;
use super::data::Thread;
use super::data::Comment;
use super::data::CategoryId;
use super::data::ThreadId;
use super::data::CommentId;
use super::data::OptId;
use super::data::UserId;
// TODO uncomment when valid impl for 'SearchQuery'
//use super::data::SearchQuery;
use super::responses::CategorySuccess;
use super::responses::ThreadSuccess;
use super::responses::CommentSuccess;
// TODO uncomment when valid impl for 'SearchQuery'
//use super::responses::SearchSuccess;
use super::responses::OkSuccess;
use super::responses::GetError;
use super::requests::CategoryRequest;
use super::requests::ThreadRequest;
use super::requests::CommentRequest;
use crate::JsonResult;
use crate::content::requests::{AddPayload, HideCategoryPayload, HideThreadPayload, HideCommentPayload};
use crate::auth::api::{authenticated, USER_TOKEN_NAME};
use rocket_contrib::Json;
use rocket::http::Cookies;
use std::convert::TryInto;

#[get("/")]
fn index() -> &'static str {
    "Homepage"
}


/// Search.
// TODO uncomment when a valid implementation for 'FromForm' exists for 'SearchQuery'
//#[get("/search?<search_str>")]
//fn search<'a>(search_str: SearchQuery<'a>) -> JsonResult<SearchSuccess<'a>, GetError> {
//    //result = controller.search(search_str);
//    trace!("sent search request to controller. search-string: {:?}", search_str);
//    Err(GetError::InvalidId).map(Json).map_err(Json)
//}


/// Get a category (name/description), or all categories (limited).
#[get("/category/<opt_id>")]
fn get_category<'a>(opt_id: OptId<CategoryId>) -> JsonResult<CategorySuccess<'a>, GetError> {
    match *opt_id {
        Some(id) => {           // Get a category
            //let result = controller.get_category(id);
            trace!("Getting category with id {:?}", id);
            "{\"response\": \"hello\"}";
            Err(GetError::InvalidId)
        },
        None => {               // Get all categories
            //let result = controller.get_all_category();
            trace!("Getting all categories");
            "{\"response\": \"hello\"}";
            Err(GetError::InvalidId)
        }
    }.map(Json).map_err(Json)
}

/// Get a categories threads.
#[get("/category/<id>/threads")]
fn get_threads_category<'a>(id: CategoryId) -> JsonResult<ThreadSuccess<'a>, GetError> {
    trace!("Getting all threads from category with id {:?}", id);
    //let result = controller.get_threads_in_category(id).map(Json).map_err(Json)
    Err(GetError::InvalidId).map(Json).map_err(Json)
}

/// Get a thread (name/description), or all categories (limited).
#[get("/thread/<opt_id>")]
fn get_thread<'a>(opt_id: OptId<ThreadId>) -> JsonResult<ThreadSuccess<'a>, GetError> {
    match *opt_id {
        Some(id) => {           // Get a thread
            //let result = controller.get_thread(id);
            trace!("Getting thread with id {:?}", id);
            Err(GetError::InvalidId)
        },
        None => {               // Get all threads
            //let result = controller.get_all_threads();
            trace!("Getting all threads");
            Err(GetError::InvalidId)
        }
    }.map(Json).map_err(Json)
}

/// Get a threads comments.
#[get("/thread/<id>/comments")]
fn get_comments_in_thread<'a>(id: ThreadId) -> JsonResult<CommentSuccess<'a>, GetError> {
    trace!("Getting all comments from thread with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(GetError::InvalidId).map(Json).map_err(Json)
}

/// Get a comment or all comments (limited).
#[get("/comments/<opt_id>")]
fn get_comment<'a>(opt_id: OptId<CommentId>) -> JsonResult<CommentSuccess<'a>, GetError> {
    match *opt_id {
        Some(id) => {           // Get a comment
            //let result = controller.get_comment(id);
            trace!("Getting thread with id {:?}", id);
            Err(GetError::InvalidId)
        },
        None => {               // Get all comments
            //let result = controller.get_all_comments();
            trace!("Getting all threads");
            Err(GetError::InvalidId)
        }
    }.map(Json).map_err(Json)
}

/// Get user info.
#[get("/user/<id>")]
fn get_user<'a>(id: UserId) -> JsonResult<ThreadSuccess<'a>, GetError> {
    trace!("Getting user with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(GetError::InvalidId).map(Json).map_err(Json)
}

#[post("/category", format = "application/json", data = "<req>")]
pub fn post_category<'a>(mut cookies: Cookies, req: Json<CategoryRequest>) -> JsonResult<OkSuccess<'a>, GetError> {

    let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(GetError::MissingToken)
                .map_err(Json)?;

    let result = authenticated(&cookie);
    if result.is_err() {
    Err(GetError::TokenNotCorrect).map_err(Json)?;
    }

    match *req {
        CategoryRequest::Add(AddPayload {
            ref raw_title,
            ref raw_description,
        }) => {
            // [..] is used to turn &String into &str
            //let title = raw_title[..].try_into().map_err(Json)?;
            //let description = raw_description[..].try_into().map_err(Json)?;

            //new_category(title, description)
            Err(GetError::InvalidId)
        }
        //CategoryRequest::Edit(Category {
        //    ref id,
        //    ref title,
        //    ref description
        //}) => {
        //    //edit_category(title, description)
        //    Err(GetError::InvalidId)
        //}
        CategoryRequest::Hide(HideCategoryPayload {
            ref id,
        }) => {
           //hide_category(title, description)
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

#[post("/thread", format = "application/json", data = "<req>")]
pub fn post_thread<'a>(mut cookies: Cookies, req: Json<ThreadRequest>) -> JsonResult<OkSuccess<'a>, GetError> {

    let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(GetError::MissingToken)
                .map_err(Json)?;

    let result = authenticated(&cookie);
    if result.is_err() {
    Err(GetError::TokenNotCorrect).map_err(Json)?;
    }

    match *req {
        ThreadRequest::Add(AddPayload {
            ref raw_title,
            ref raw_description,
        }) => {
            // [..] is used to turn &String into &str
            //let title = raw_title[..].try_into().map_err(Json)?;
            //let description = raw_description[..].try_into().map_err(Json)?;

            //new_thread(title, description, result.id)      // Send title and description + user-id to controller
            Err(GetError::InvalidId)
        }
        //ThreadRequest::Edit(Thread {
        //    ref id,
        //    ref title,
        //    ref description,
        //    ref category_id
        //}) => {
        //    //edit_thread(title, description, id)
        //    Err(GetError::InvalidId)
        //}
        ThreadRequest::Hide(HideThreadPayload {
            ref id,
        }) => {
           //hide_thread(id)
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

#[post("/comment", format = "application/json", data = "<req>")]
pub fn post_comment<'a>(mut cookies: Cookies, req: Json<CommentRequest>) -> JsonResult<OkSuccess<'a>, GetError> {

    let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(GetError::MissingToken)
                .map_err(Json)?;

    let result = authenticated(&cookie);
    if result.is_err() {
    Err(GetError::TokenNotCorrect).map_err(Json)?;
    }

    match *req {
        CommentRequest::Add(AddPayload {
            ref raw_title,
            ref raw_description,
        }) => {
            // [..] is used to turn &String into &str
            //let title = raw_title[..].try_into().map_err(Json)?;
            //let description = raw_description[..].try_into().map_err(Json)?;

            //new_comment(title, description, result.id)      // Send title and description + user-id to controller
            Err(GetError::InvalidId)
        }
        //CommentRequest::Edit(Comment {
        //    ref id,
        //    ref uid,
        //    ref content,
        //    ref thread
        //}) => {
        //    //edit_comment(title, description, id)
        //    Err(GetError::InvalidId)
        //}
        CommentRequest::Hide(HideCommentPayload {
            ref id,
        }) => {
           //hide_comment(id)
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}


/*
/// Make a new category
#[post("/category/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn add_category(cookies: Cookies, input: Form<Category>, remote_addr: SocketAddr) -> String {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role > 1 {                                      // If token is correct and role is moderator or above.
        //let ret = controller.add_category(input, result);
        info!("{}: sent request to add new thread. will be added by user {}", remote_addr, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Edit category.
#[post("/category/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn edit_category(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.edit_category(input, result, cid);
        info!("{}: sent request to edit category with id {}, for user {} to controller", remote_addr, cid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Hide category.
#[post("/category/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hide_category(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.hide_category(input, result, cid);
        info!("{}: sent request to hide category with id {}, for user {} to controller", remote_addr, cid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}


/// Make a new thread.
#[post("/thread/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn add_thread(cookies: Cookies, input: Form<Thread>, remote_addr: SocketAddr) -> String {
    //result = auth.verify_user(cookies.get_private("user_token"))
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.add_thread(input, result);
        info!("{}: sent request to add new thread. will be added by user {}", remote_addr, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Edit thread.
#[post("/thread/edit/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn edit_thread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.edit_thread(input, result, tid);
        info!("{}: sent request to edit thread with id {}, for user {} to controller", remote_addr, tid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Hide thread.
#[post("/thread/hide/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hide_thread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hide_thread(input, result, tid);
        info!("{}: sent request to hide thread with id {}, for user {} to controller", remote_addr, tid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Get all comments in a thread.
#[get("/thread/<thread>")]
fn get_thread(thread: u32, remote_addr: SocketAddr) -> String {
    //let result = controller.get_thread(thread);
    info!("{}: sent request of getting thread with id {} to controller", remote_addr, thread);
    JSON(result);
}

/// Comment on a thread.
#[post("/comment/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn comment(cookies: Cookies, input: Form<Comment>, remote_addr: SocketAddr) -> String {
    //result = authverify_user(cookies.get_private("user_token"))
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.comment(input, result);
        info!("{}: sent request to let user {} comment thread {} to controller", remote_addr, result.username, input.thread);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}


/// Edit comment.
#[post("/comment/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn edit_comment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.edit_comment(input, result, cid);
        info!("{}: sent request to edit comment with id {}, for user {} to controller", remote_addr, cid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Hide comment.
#[post("/comment/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hide_comment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verify_user(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hide_comment(input, result, cid);
        info!("{}: sent request to hide comment with id {}, for user {} to controller", remote_addr, cid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

}*/
