use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::path::{Path, PathBuf};

use crate::auth::api::{authenticated, Token};
use crate::JsonResult;

use datatypes::content::requests::{ContentRequest, SearchPayload};
use datatypes::content::responses::{ContentError, ContentSuccess};
use datatypes::error::ResponseError;
use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
#[get("/search?<search_form>")]
fn search(search_form: SearchForm) -> JsonResult<ContentSuccess> {
    let search_request: ContentRequest = ContentRequest::Search(search_form.into());
    //result = controller.search(search_request);
    trace!(
        "sent search request to controller. search-string: {:?}",
        search_request
    );
    Err(ContentError::InvalidId)
        .map_err(ResponseError::from)
        .map(Json)
        .map_err(Json)
}

/// A search form used to make all searches done in the URL
#[derive(FromForm, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchForm {
    q: QueryStr,
}

impl Into<SearchPayload> for SearchForm {
    fn into(self) -> SearchPayload {
        SearchPayload::new(self.q)
    }
}

/// Get a category (name/description), or all categories (limited).
#[get("/category/<opt_id>")]
fn get_category(opt_id: OptId<CategoryId>) -> JsonResult<ContentSuccess> {
    match *opt_id {
        Some(id) => {
            // Get a category
            //let result = controller.get_category(id);
            trace!("Getting category with id {:?}", id);
            Err(ContentError::InvalidId)
        }
        None => {
            // Get all categories
            //let result = controller.get_all_category();
            trace!("Getting all categories");
            Err(ContentError::InvalidId)
        }
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}

/// Get a categories threads.
#[get("/category/<id>/threads")]
fn get_threads_category(id: CategoryId) -> JsonResult<ContentSuccess> {
    trace!("Getting all threads from category with id {:?}", id);
    //let result = controller.get_threads_in_category(id).map(Json).map_err(Json)
    Err(ContentError::InvalidId)
        .map_err(ResponseError::from)
        .map(Json)
        .map_err(Json)
}

/// Get a thread (name/description), or all categories (limited).
#[get("/thread/<opt_id>")]
fn get_thread(opt_id: OptId<ThreadId>) -> JsonResult<ContentSuccess> {
    match *opt_id {
        Some(id) => {
            // Get a thread
            //let result = controller.get_thread(id);
            trace!("Getting thread with id {:?}", id);
            Err(ContentError::InvalidId)
        }
        None => {
            // Get all threads
            //let result = controller.get_all_threads();
            trace!("Getting all threads");
            Err(ContentError::InvalidId)
        }
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}

/// Get a threads comments.
#[get("/thread/<id>/comments")]
fn get_comments_in_thread(id: ThreadId) -> JsonResult<ContentSuccess> {
    trace!("Getting all comments from thread with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(ContentError::InvalidId)
        .map_err(ResponseError::from)
        .map(Json)
        .map_err(Json)
}

/// Get a comment or all comments (limited).
#[get("/comments/<opt_id>")]
fn get_comment(opt_id: OptId<CommentId>) -> JsonResult<ContentSuccess> {
    match *opt_id {
        Some(id) => {
            // Get a comment
            //let result = controller.get_comment(id);
            trace!("Getting thread with id {:?}", id);
            Err(ContentError::InvalidId)
        }
        None => {
            // Get all comments
            //let result = controller.get_all_comments();
            trace!("Getting all threads");
            Err(ContentError::InvalidId)
        }
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}

/// Get user info.
#[get("/user/<id>")]
fn get_user(id: UserId) -> JsonResult<ContentSuccess> {
    trace!("Getting user with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(ContentError::InvalidId)
        .map_err(ResponseError::from)
        .map(Json)
        .map_err(Json)
}

#[post("/content", format = "application/json", data = "<req>")]
pub fn post_content(token: Token, req: Json<ContentRequest>) -> JsonResult<ContentSuccess> {
    use datatypes::content::requests::ContentRequest::*;
    use datatypes::content::responses::CategoryPayload;
    use datatypes::content::responses::ContentError;

    authenticated(token).map_err(|_| Json(ResponseError::Unauthenticated))?;

    match *req {
        AddCategory(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(1, title, description);
            Ok(ContentSuccess::Category(payload))
            //new_category(title, description)
            //Err(ContentError::InvalidId)
        }
        //CategoryRequest::Edit(Category {
        //    ref id,
        //    ref title,
        //    ref description
        //}) => {
        //    //edit_category(title, description)
        //    Err(GetError::InvalidId)
        //}
        HideCategory(ref _p) => {
            //hide_category(title, description)
            Err(ContentError::InvalidId)
        }
        _ => unimplemented!(),
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}
