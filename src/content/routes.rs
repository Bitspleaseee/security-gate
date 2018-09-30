use super::data::CategoryId;
use super::data::CommentId;
use super::data::OptId;
use super::data::SearchQuery;
use super::data::ThreadId;
use super::data::UserId;
use super::requests::CategoryRequest;
use datatypes::content::responses::CategoryPayload;
use datatypes::content::responses::CommentPayload;
use datatypes::content::responses::ContentRequestSuccess;
use datatypes::content::responses::ContentRequestError;
use super::responses::SearchSuccess;
use datatypes::content::responses::ThreadPayload;
use datatypes::content::requests::ContentRequest::{
    self, AddCategory, EditCategory, HideCategory,
    AddThread, EditThread, HideThread,
    AddComment, EditComment, HideComment
    };
use crate::auth::api::{authenticated, Token};
use datatypes::content::requests::{
    AddCategoryPayload, EditCategoryPayload, HideCategoryPayload
};
use crate::JsonResult;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// Search.
// TODO uncomment when a valid implementation for 'FromForm' exists for 'SearchQuery'
#[get("/search?<search_str>")]
fn search<'a>(search_str: SearchQuery<'a>) -> JsonResult<SearchSuccess<'a>, ContentRequestError> {
    //result = controller.search(search_str);
    trace!(
        "sent search request to controller. search-string: {:?}",
        search_str
    );
    Err(ContentRequestError::InvalidId).map(Json).map_err(Json)
}

/// Get a category (name/description), or all categories (limited).
#[get("/category/<opt_id>")]
fn get_category<'a>(opt_id: OptId<CategoryId>) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    match *opt_id {
        Some(id) => {
            // Get a category
            //let result = controller.get_category(id);
            trace!("Getting category with id {:?}", id);
            Err(ContentRequestError::InvalidId)
        }
        None => {
            // Get all categories
            //let result = controller.get_all_category();
            trace!("Getting all categories");
            Err(ContentRequestError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

/// Get a categories threads.
#[get("/category/<id>/threads")]
fn get_threads_category<'a>(id: CategoryId) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    trace!("Getting all threads from category with id {:?}", id);
    //let result = controller.get_threads_in_category(id).map(Json).map_err(Json)
    Err(ContentRequestError::InvalidId).map(Json).map_err(Json)
}

/// Get a thread (name/description), or all categories (limited).
#[get("/thread/<opt_id>")]
fn get_thread<'a>(opt_id: OptId<ThreadId>) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    match *opt_id {
        Some(id) => {
            // Get a thread
            //let result = controller.get_thread(id);
            trace!("Getting thread with id {:?}", id);
            Err(ContentRequestError::InvalidId)
        }
        None => {
            // Get all threads
            //let result = controller.get_all_threads();
            trace!("Getting all threads");
            Err(ContentRequestError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

/// Get a threads comments.
#[get("/thread/<id>/comments")]
fn get_comments_in_thread<'a>(id: ThreadId) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    trace!("Getting all comments from thread with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(ContentRequestError::InvalidId).map(Json).map_err(Json)
}

/// Get a comment or all comments (limited).
#[get("/comments/<opt_id>")]
fn get_comment<'a>(opt_id: OptId<CommentId>) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    match *opt_id {
        Some(id) => {
            // Get a comment
            //let result = controller.get_comment(id);
            trace!("Getting thread with id {:?}", id);
            Err(ContentRequestError::InvalidId)
        }
        None => {
            // Get all comments
            //let result = controller.get_all_comments();
            trace!("Getting all threads");
            Err(ContentRequestError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}

/// Get user info.
#[get("/user/<id>")]
fn get_user<'a>(id: UserId) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    trace!("Getting user with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(ContentRequestError::InvalidId).map(Json).map_err(Json)
}

#[post("/content", format = "application/json", data = "<req>")]
pub fn post_content<'a>(
    token: Token,
    req: Json<ContentRequest<'a>>,
) -> JsonResult<ContentRequestSuccess<'a>, ContentRequestError> {
    let result = authenticated(token);
    if result.is_err() {
        Err(ContentRequestError::InvalidToken).map_err(Json)?;
    }

    match *req {
        AddCategory(AddCategoryPayload {
            ref title,
            ref description,
        }) => {
            // [..] is used to turn &String into &str
            //let title = raw_title[..].try_into().map_err(Json)?;
            //let description = raw_description[..].try_into().map_err(Json)?;

            //new_category(title, description)
            Err(ContentRequestError::InvalidId)
        }
        //CategoryRequest::Edit(Category {
        //    ref id,
        //    ref title,
        //    ref description
        //}) => {
        //    //edit_category(title, description)
        //    Err(GetError::InvalidId)
        //}
        HideCategory(HideCategoryPayload { ref hide }) => {
            //hide_category(title, description)
            Err(ContentRequestError::InvalidId)
        }
    }.map(Json)
    .map_err(Json)
}
