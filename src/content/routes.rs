use super::data::Category;
use super::data::CategoryId;
use super::data::Comment;
use super::data::CommentId;
use super::data::OptId;
use super::data::Thread;
use super::data::ThreadId;
use super::data::UserId;
use super::data::SearchQuery;
use super::responses::CategorySuccess;
use super::responses::CommentSuccess;
use super::responses::ThreadSuccess;
use super::responses::SearchSuccess;
use super::requests::CategoryRequest;
use super::requests::CommentRequest;
use super::requests::ThreadRequest;
use super::responses::GetError;
use super::responses::OkSuccess;
use crate::auth::api::{authenticated, USER_TOKEN_NAME, Token};
use crate::content::requests::{
    AddPayload, HideCategoryPayload, HideCommentPayload, HideThreadPayload,
};
use crate::JsonResult;
use rocket::http::Cookies;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::convert::TryInto;
use std::io;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/static/<file>")]
fn static_file(file: &RawStr) -> io::Result<NamedFile> {
    NamedFile::open(format!("static/{}", file))
}

/// Search.
// TODO uncomment when a valid implementation for 'FromForm' exists for 'SearchQuery'
#[get("/search?<search_str>")]
fn search<'a>(search_str: SearchQuery<'a>) -> JsonResult<SearchSuccess<'a>, GetError> {
   //result = controller.search(search_str);
   trace!("sent search request to controller. search-string: {:?}", search_str);
   Err(GetError::InvalidId).map(Json).map_err(Json)
}

/// Get a category (name/description), or all categories (limited).
#[get("/category/<opt_id>")]
fn get_category<'a>(opt_id: OptId<CategoryId>) -> JsonResult<CategorySuccess<'a>, GetError> {
    match *opt_id {
        Some(id) => {
            // Get a category
            //let result = controller.get_category(id);
            trace!("Getting category with id {:?}", id);
            Err(GetError::InvalidId)
        }
        None => {
            // Get all categories
            //let result = controller.get_all_category();
            trace!("Getting all categories");
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
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
        Some(id) => {
            // Get a thread
            //let result = controller.get_thread(id);
            trace!("Getting thread with id {:?}", id);
            Err(GetError::InvalidId)
        }
        None => {
            // Get all threads
            //let result = controller.get_all_threads();
            trace!("Getting all threads");
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
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
        Some(id) => {
            // Get a comment
            //let result = controller.get_comment(id);
            trace!("Getting thread with id {:?}", id);
            Err(GetError::InvalidId)
        }
        None => {
            // Get all comments
            //let result = controller.get_all_comments();
            trace!("Getting all threads");
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

/// Get user info.
#[get("/user/<id>")]
fn get_user<'a>(id: UserId) -> JsonResult<ThreadSuccess<'a>, GetError> {
    trace!("Getting user with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(GetError::InvalidId).map(Json).map_err(Json)
}

#[post("/content", format = "application/json", data = "<req>")]
pub fn post_content<'a>(
    token: Token,
    req: Json<CategoryRequest>,
) -> JsonResult<OkSuccess<'a>, GetError> {
    let result = authenticated(token);
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
        CategoryRequest::Hide(HideCategoryPayload { ref id }) => {
            //hide_category(title, description)
            Err(GetError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}


