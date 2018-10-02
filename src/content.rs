//! API-routes to manage content.
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

/// Get the main webpage.
/// 
/// This function returns the content of the webpage as html/css/javascript.
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

/// Get the other webpages.
/// 
/// This function returns the content of the webpage given in file as html/css/javascript.
#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// Search after some content.
/// 
/// # Example
/// 
/// ## Query
/// 
/// ´´´text
/// localhost:9000/api/search?q=hello%20world
/// ´´´
/// 
/// ## Result
/// ´´´json
/// {
///     "type": "SEARCHRESULT",
///     "payload": {
///         "categories": {},
///         "threads": {
///             {
///                 "id": 23,
///                 "category_id": 3,
///                 "user_id": 12,
///                 "title": "How to make a hello world app in javascript",
///                 "description": "How can I do that?",
///                 "timestamp": 201820121200
///             }
///         },
///         "comments": {[
///             {
///                 "id": 56,
///                 "thread_id": 23,
///                 "parent_id": 54,
///                 "user_id": 4,
///                 "title": "SV: How to make a hello world app in javascript",
///                 "description": "See on http://w3schools.com",
///                 "timestamp": 201820121206
///             }
///         ]},
///         users: {}
///     }
/// }
/// ´´´
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
/// 
/// If you don't give an id, all categories will be returned.
/// 
/// # Example
/// 
/// ## Query
/// 
/// ´´´text
/// localhost:9000/api/category/3
/// ´´´
/// 
/// ## Result
/// ´´´json
/// {
///     "type": "CATEGORY",
///     "payload": {
///         "id": 3,
///         "user_id": 4,
///         "title": "Javascript",
///         "description": "All questions regarding javascript.",
///         "timestamp": 201820031206
///     }
/// }
/// ´´´
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
///
/// If you don't give an id, all threads will be returned.
/// 
/// # Example
/// 
/// ## Query
/// 
/// ´´´text
/// localhost:9000/api/thread/6
/// ´´´
/// 
/// ## Result
/// ´´´json
/// {
///     "type": "THREAD",
///     "payload": {
///         "id": 6,
///         "category_id": 2,
///         "user_id": 4,
///         "title": "Hello",
///         "description": "If you want to say hello, do it here.",
///         "timestamp": 201820131206
///     }
/// }
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
///
/// If you don't give an id, all comments will be returned.
/// 
/// # Example
/// 
/// ## Query
/// 
/// ´´´text
/// localhost:9000/api/comment/98
/// ´´´
/// 
/// ## Result
/// ´´´json
/// {
///     "type": "COMMENT",
///     "payload": {
///         "id": 98,
///         "thread_id": 6,
///         "user_id": 22,
///         "title": "Hello",
///         "description": "Hello everybody.",
///         "timestamp": 201820901206
///     }
/// }
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

/// Get user info by id.
/// 
/// # Example
/// 
/// ## Query
/// 
/// ´´´text
/// localhost:9000/api/user/22
/// ´´´
/// 
/// ## Result
/// ´´´json
/// {
///     "type": "USER",
///     "payload":
///         {
///         "id": 22,
///         "username": "FT45",
///         "description": "Hello Everyone. I like programming",
///         "avatar": "pictures/FT45.png"
///         }
/// } 
#[get("/user/<id>")]
fn get_user(id: UserId) -> JsonResult<ContentSuccess> {
    trace!("Getting user with id {:?}", id);
    //let result = controller.get_comment_in_thread(id).map(Json).map_err(Json)
    Err(ContentError::InvalidId)
        .map_err(ResponseError::from)
        .map(Json)
        .map_err(Json)
}


/// Add some content.
///
/// If you are admin, you can ban or unban users.
/// Types you can send in:
/// 'ADDCATEGORY', 'EDITCATEGORY', 'HIDECATEGORY',
/// 'ADDTHREAD', 'EDITTHREAD', 'HIDETREAD',
/// 'ADDCOMMENT', 'EDITCOMMENT', 'HIDECOMMENT',
/// 'UPLOADAVATAR'.
/// 
/// Types I can get back: 'CATEGORY', 'THREAD', 'COMMENT'.
/// 
/// # Example
///
/// Send this json to 'api/content' (need to first be logged in)
/// 
///´´´json
///{
///  "type": "ADDCOMMENT"
///  "payload": {
///      "thread_id": 6,
///      "user_id": 22,
///      "title": "Hello",
///      "description": "Hello everybody.",
///      "timestamp": 201820901206
///  }
///}
/// ´´´
/// 
/// Result:
///
///´´´json
///{
///  "type": "COMMENT"
///  "payload": {
///      "id": 98,
///      "thread_id": 6,
///      "user_id": 22,
///      "title": "Hello",
///      "description": "Hello everybody.",
///      "timestamp": 201820901206
///  }
///}
/// ´´´
#[post("/content", format = "application/json", data = "<req>")]
pub fn post_content(token: Token, req: Json<ContentRequest>) -> JsonResult<ContentSuccess> {
    use datatypes::content::requests::ContentRequest::*;
    use datatypes::content::responses::CategoryPayload;
    use datatypes::content::responses::ContentError;

    // Ask auth-module if user can do this (is logged in and has correct role):
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
        /*EditCategory(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            ////let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(1, title, description);
            Ok(ContentSuccess::Category(payload))
            //edit_category(id, title, description)
            //Err(ContentError::InvalidId)
        }
        /*HideCategory(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //hide_category(id, title, description)
            //Err(ContentError::InvalidId)
        }
        AddThread(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let category_id = p.category_id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(1, title, description);
            Ok(ContentSuccess::Category(payload))
            //new_thread(title, description)
            //Err(ContentError::InvalidId)
        }
        EditThread(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let category_id = p.category_id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //edit_thread(id, title, description)
            //Err(ContentError::InvalidId)
        }
        HideThread(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //hide_thread(id, title, description)
            //Err(ContentError::InvalidId)
        }
        AddComment(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(1, title, description);
            Ok(ContentSuccess::Category(payload))
            //new_comment(title, description)
            //Err(ContentError::InvalidId)
        }
        EditComment(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //edit_comment(id, title, description)
            //Err(ContentError::InvalidId)
        }
        HideComment(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //hide_comment(id, title, description)
            //Err(ContentError::InvalidId)
        }
        UploadAvatar(ref p) => {
            // Relays what is sent back to the user
            // TODO must be changed, added for testing
            let id = p.id().clone();
            let title = p.title().clone();
            let description = p.description().clone();

            let payload = CategoryPayload::new(id, title, description);
            Ok(ContentSuccess::Category(payload))
            //upload_avatar(id, title, description)
            //Err(ContentError::InvalidId)
        }*/
        _ => unimplemented!(),
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}*/
