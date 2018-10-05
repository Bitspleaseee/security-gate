//! API-routes to manage content.
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::path::{Path, PathBuf};
use tarpc::sync::client::{Options, ClientExt};

use crate::auth::api::{authenticated, Token};
use crate::JsonResponseResult;

use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::{ResponseError, ResponseResult};
use datatypes::valid::fields::*;
use datatypes::valid::ids::*;

use crate::comms::controller;
use crate::comms::controller::CONTROLLER_IP;

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
fn search(search_form: SearchForm) -> JsonResponseResult<ContentSuccess> {
    let search_request: SearchPayload = SearchPayload {
        query: search_form.q
    };
    trace!(
        "Sending search request to controller. search-string: {:?}",
        search_request.query
    );

    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;

    match con.search(search_request)
         {
        Ok(v) => {
            trace!("Gotten back search info for query {:?} from controller.", search_request.query);
            Ok(ContentSuccess::SearchResult(v))
        },
        Err(e) => {
            error!("Error when getting user from controller: {}", e);
            Err(ResponseError::InternalServerError)
        }
    }
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
        SearchPayload { query: self.q }
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
fn get_category(opt_id: OptId<CategoryId>) -> JsonResponseResult<ContentSuccess> {
    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;

    match *opt_id {
        Some(raw_id) => {
            // Get a category
            //let result = controller.get_category(id);
            trace!("Getting category with id {:?}", raw_id);

            let category_payload: GetCategoryPayload = GetCategoryPayload {
                id: raw_id
            };
            match con.get_category(category_payload)
                 {
                Ok(v) => {
                    trace!("Gotten back category with id {:?} from controller.", raw_id);
                    Ok(ContentSuccess::Category(v))
                },
                Err(e) => {
                    error!("Error when getting category from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        None => {
            // Get all categories
            //let result = controller.get_all_category();
            let also_hidden: GetHiddenPayload = GetHiddenPayload {
                include_hidden: false,
            };
            trace!("Getting all categories");
            match con.get_categories(also_hidden)
                 {
                Ok(v) => {
                    trace!("Gotten back all categories from controller. Also hidden: {:?}", also_hidden.include_hidden);
                    Ok(ContentSuccess::Categories(v))
                },
                Err(e) => {
                    error!("Error when getting category from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}

/// Get a categories threads.
#[get("/category/<id>/threads")]
fn get_threads_category(id: CategoryId) -> JsonResponseResult<ContentSuccess> {
    trace!("Getting all threads from category with id {:?}", id);
    let category_payload: GetThreadsPayload = GetThreadsPayload {
        id: id
    };

    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;

    match con.get_threads(category_payload)
         {
        Ok(v) => {
            trace!("Gotten back category with id {:?} from controller.", id);
            Ok(ContentSuccess::Threads(v))
        },
        Err(e) => {
            error!("Error when getting category from controller: {}", e);
            Err(ResponseError::InternalServerError)
        }
    }.map_err(ResponseError::from)
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
fn get_thread(opt_id: OptId<ThreadId>) -> JsonResponseResult<ContentSuccess> {
    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;
    
    match *opt_id {
        Some(raw_id) => {
            // Get a thread
            trace!("Getting thread with id {:?}", raw_id);

            let thread_payload: GetThreadPayload = GetThreadPayload {
                id: raw_id
            };

            match con.get_thread(thread_payload)
                 {
                Ok(v) => {
                    trace!("Gotten back thread with id {:?} from controller.", raw_id);
                    Ok(ContentSuccess::Thread(v))
                },
                Err(e) => {
                    error!("Error when getting thread from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        None => {
            // Get all threads
            //let result = controller.get_all_threads();
            trace!("Getting all threads");
            
            let also_hidden: GetHiddenPayload = GetHiddenPayload {
                include_hidden: false,
            };

           match con.get_all_threads(also_hidden)
                 {
                Ok(v) => {
                    trace!("Gotten back all threads from controller.");
                    Ok(ContentSuccess::Threads(v))
                },
                Err(e) => {
                    error!("Error when getting all threads from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
    }.map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}

/// Get a threads comments.
#[get("/thread/<id>/comments")]
fn get_comments_in_thread(id: ThreadId) -> JsonResponseResult<ContentSuccess> {
    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;
    
    trace!("Getting all comments from thread with id {:?}", id);

    let comments_payload: GetCommentsPayload = GetCommentsPayload {
        id: id
    };

    match con.get_comments(comments_payload)
         {
            Ok(v) => {
                trace!("Gotten back all comments in thread with id from controller.", id);
                Ok(ContentSuccess::Comments(v))
            },
            Err(e) => {
                error!("Error when getting all threads from controller: {}", e);
                Err(ResponseError::InternalServerError)
            }
        }
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
fn get_comment(opt_id: OptId<CommentId>) -> JsonResponseResult<ContentSuccess> {
    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;

    match *opt_id {
        Some(raw_id) => {
            // Get a comment
            trace!("Getting comment with id {:?}", raw_id);
            
            let comment_payload: GetCommentPayload = GetCommentPayload {
                id: raw_id
            };

            match con.get_comment(comment_payload) {
                Ok(v) => {
                    trace!("Gotten back comment with id {:?} from controller.", raw_id);
                    Ok(ContentSuccess::Threads(v))
                },
                Err(e) => {
                    error!("Error when getting all threads from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        None => {
            // Get all comments
            trace!("Getting all comments");

            let also_hidden: GetHiddenPayload = GetHiddenPayload {
                include_hidden: false,
            };

            match con.get_comment(also_hidden) {
                Ok(v) => {
                    trace!("Gotten back all comments from controller.");
                    Ok(ContentSuccess::Threads(v))
                },
                Err(e) => {
                    error!("Error when getting all comments from controller: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
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
fn get_user(id: UserId) -> JsonResponseResult<ContentSuccess> {
    trace!("Getting user with id {:?}", id);

    let user_id = GetUserPayload {
        id: id
    };

    let con =
        controller::SyncClient::connect(CONTROLLER_IP, Options::default())
        .map_err(|e| {
            error!("error connecting to controller: {}", e);
            Json(ResponseError::InternalServerError)
        })?;

    match con.get_user(user_id) {
        Ok(v) => {
            trace!("Gotten back user info for user with id {:?} from controller.", id);
            Ok(ContentSuccess::User(v))
        },
        Err(e) => {
            error!("Error when getting user from controller: {}", e);
            Err(ResponseError::InternalServerError)
        }
    }
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
pub fn post_content(token: Token, req: Json<ContentRequest>) -> JsonResponseResult<ContentSuccess> {
    use datatypes::content::requests::ContentRequest::*;
    use datatypes::content::responses::CategoryPayload;

    // Ask auth-module if user can do this (is logged in and has correct role):
    authenticated(token).map_err(|_| Json(ResponseError::Unauthenticated))?;

        let con =
            controller::SyncClient::connect(CONTROLLER_IP, Options::default())
            .map_err(|e| {
                error!("error connecting to controller: {}", e);
                Json(ResponseError::InternalServerError)
            })?;

    match *req {
        AddCategory(ref p) => {
            // Relays what is sent back to the user
            match con.add_category(p) {
                Ok(v) => {
                    trace!("Added new category with title: {}", p.title.clone());
                    Ok(ContentSuccess::Category(v))
                },
                Err(e) => {
                    error!("Error when adding new category with title {}: {}", p.title.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        EditCategory(ref p) => {
            // Relays what is sent back to the user
            match con.edit_category(p) {
                Ok(v) => {
                    trace!("Edited category with id: {}", p.id.clone());
                    Ok(ContentSuccess::Category(v))
                },
                Err(e) => {
                    error!("Error when trying to edit category with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        HideCategory(ref p) => {
            // Relays what is sent back to the user
            match con.hide_category(p) {
                Ok(v) => {
                    trace!("Hided category with id: {}", p.id.clone());
                    Ok(ContentSuccess::Category(v))
                },
                Err(e) => {
                    error!("Error when trying to hide category with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        AddThread(ref p) => {
            // Relays what is sent back to the user
            match con.add_thread(p) {
                Ok(v) => {
                    trace!("Added new thread with title: {}", p.title.clone());
                    Ok(ContentSuccess::Thread(v))
                },
                Err(e) => {
                    error!("Error when adding new thread with title {}: {}", p.title.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        EditThread(ref p) => {
            // Relays what is sent back to the user
            match con.edit_thread(p) {
                Ok(v) => {
                    trace!("Edited thread with id: {}", p.id.clone());
                    Ok(ContentSuccess::Thread(v))
                },
                Err(e) => {
                    error!("Error when trying to edit thread with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        HideThread(ref p) => {
            // Relays what is sent back to the user
            match con.hide_thread(p) {
                Ok(v) => {
                    trace!("Hided thread with id: {}", p.id.clone());
                    Ok(ContentSuccess::Thread(v))
                },
                Err(e) => {
                    error!("Error when trying to hide thread with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        AddComment(ref p) => {
            // Relays what is sent back to the user
            match con.add_comment(p) {
                Ok(v) => {
                    trace!("Added new comment with title: {}", p.title.clone());
                    Ok(ContentSuccess::Comment(v))
                },
                Err(e) => {
                    error!("Error when adding new comment with title {}: {}", p.title.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        EditComment(ref p) => {
            // Relays what is sent back to the user
            match con.edit_comment(p) {
                Ok(v) => {
                    trace!("Edited comment with id: {}", p.id.clone());
                    Ok(ContentSuccess::Comment(v))
                },
                Err(e) => {
                    error!("Error when trying to edit comment with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        HideComment(ref p) => {
            // Relays what is sent back to the user
            match con.hide_comment(p) {
                Ok(v) => {
                    trace!("Hided comment with id: {}", p.id.clone());
                    Ok(ContentSuccess::Category(v))
                },
                Err(e) => {
                    error!("Error when trying to hide comment with id {}: {}", p.id.clone(), e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        UploadAvatar(ref p) => {
            // Relays what is sent back to the user
            match con.upload_avatar(p) {
                Ok(v) => {
                    trace!("Uploaded avatar");
                    Ok(ContentSuccess::Avatar(v))
                },
                Err(e) => {
                    error!("Error when trying to upload avatar: {}", e);
                    Err(ResponseError::InternalServerError)
                }
            }
        }
        _ => unimplemented!(),
    }
    .map_err(ResponseError::from)
    .map(Json)
    .map_err(Json)
}
