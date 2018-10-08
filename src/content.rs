//! API-routes to manage content.
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tarpc::sync::client::{ClientExt, Options};
use tarpc::util::FirstSocketAddr;

use datatypes::auth::responses::*;
use datatypes::content::requests::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;
use datatypes::payloads::TokenPayload;
use datatypes::valid::fields::*;
use datatypes::valid::ids::*;
use datatypes::valid::token::Token;

use crate::auth::connect_to_auth;
use crate::comms::controller::SyncClient as ControllerClient;
use crate::JsonResponseResult;

lazy_static! {
    static ref CONTROLLER_IP: SocketAddr = match std::env::var("CONTROLLER_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            warn!("CONTROLLER_ADDRESS is not set, using 'localhost:10000'");
            "localhost:10000".to_string()
        }
    }.try_first_socket_addr()
    .expect("Invalid formatted CONTROLLER_ADDRESS");
}

fn connect_to_controller() -> Result<ControllerClient, ResponseError> {
    ControllerClient::connect(*CONTROLLER_IP, Options::default()).map_err(|e| {
        error!("Unable to connect to controller: {:?}", e);
        ResponseError::InternalServerError
    })
}

// Check if user is admin or moderator
fn is_admin_or_mod(token: Option<Token>) -> Result<bool, ResponseError> {
    token.map_or(Ok(false), |t| {
        Ok(connect_to_auth()
            .map_err(|e| {
                error!("Failed to connect to auth service: {:?}", e);
                e
            })?.get_user_role(TokenPayload::new(None, t))?
            >= Role::Moderator)
    })
}

/// Get the main webpage
///
/// This function returns the content of the webpage as html/css/javascript.
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

/// Get static files (.html, .css, .js, images etc.)
///
/// This function returns the content of the webpage given in file as html/css/javascript.
#[get("/static/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/// Search database based on a search query
///
/// # Example
///
/// ## Query
///
/// ´´´text
/// localhost:9234/api/search?q=hello%20world
/// ´´´
///
/// ## Result
/// ´´´json
/// {
///     "type": "SEARCH_RESULT",
///     "payload": {
///         "categories": {},
///         "threads": [
///             {
///                 "id": 23,
///                 "category_id": 3,
///                 "user_id": 12,
///                 "title": "How to make a hello world app in javascript",
///                 "description": "How can I do that?",
///                 "timestamp": 201820121200
///             }
///         ],
///         "comments": [
///             {
///                 "id": 56,
///                 "thread_id": 23,
///                 "parent_id": 54,
///                 "user_id": 4,
///                 "description": "See on http://w3schools.com for how to make a hello world app",
///                 "timestamp": 201820121206
///             }
///         ],
///         users: {}
///     }
/// }
/// ´´´
#[get("/search?<search_form>")]
fn search(
    search_form: Option<SearchForm>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let search_form = search_form
        .ok_or(ContentError::InvalidSearchQuery)
        .map_err(|e| Json(e.into()))?; // If invalid query.

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let search_req: SearchPayload = SearchPayload {
        query: search_form.q,
        include_hidden,
    };

    info!("Requesting search query '{:?}'", search_req.query);

    connect_to_controller()
        .map_err(Json)?
        .search(search_req)
        .map(|v| {
            info!("Returning success from 'search' request");
            Json(ContentSuccess::SearchResult(v))
        }).map_err(|e| {
            error!("Unable to 'search': {:?}", e);
            Json(e.into())
        })
}

/// A search form used to make all searches done in the URL
#[derive(FromForm, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchForm {
    q: QueryStr,
    hidden: Option<bool>,
}

impl Into<SearchPayload> for SearchForm {
    fn into(self) -> SearchPayload {
        SearchPayload {
            query: self.q,
            include_hidden: self.hidden.unwrap_or(false),
        }
    }
}

/// Get a category based on id
///
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
///
/// # Example
///
/// ## Query
///
/// ´´´text
/// localhost:9234/api/category/3
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
#[get("/category/<id>")]
fn get_category(
    id: Option<CategoryId>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id.

    info!("Requesting category with id {}", id);

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.

    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let category_payload: GetCategoryPayload = GetCategoryPayload { id, include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_category(category_payload)
        .map(|v| {
            info!("Returning success from 'get-category' request");
            Json(ContentSuccess::Category(v))
        }).map_err(|e| {
            error!("Unable to 'get-category': {:?}", e);
            Json(e.into())
        })
}

/// Get all categories (limited)
///
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
///
/// # Example
///
/// ## Query
///
/// ´´´text
/// localhost:9234/api/categories
/// ´´´
///
/// ## Result
/// ´´´json
/// {
///     "type": "CATEGORIES",
///     "payload": [{
///         "id": 1,
///         "user_id": 1,
///         "title": "Rust",
///         "description": "All questions regarding Rust.",
///         "timestamp": 201820033206
///     },
///     {
///         "id": 2,
///         "user_id": 1,
///         "title": "PHP",
///         "description": "All questions regarding PHP.",
///         "timestamp": 201820033206
///     },
///     {
///         "id": 3,
///         "user_id": 4,
///         "title": "Javascript",
///         "description": "All questions regarding javascript.",
///         "timestamp": 201820031206
///     },
///     {
///         "id": 4,
///         "user_id": 4,
///         "title": "HTML",
///         "description": "All questions regarding HTML.",
///         "timestamp": 201820033206
///     }]
/// }
/// ´´´
#[get("/categories")]
fn get_categories(opt_token: Option<Token>) -> JsonResponseResult<ContentSuccess> {
    info!("Requesting all categories");

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let hidden_payload: GetHiddenPayload = GetHiddenPayload { include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_all_categories(hidden_payload)
        .map(|v| {
            info!("Returning success from 'get-categories' request");
            Json(ContentSuccess::Categories(v))
        }).map_err(|e| {
            error!("Unable to 'get-categories': {:?}", e);
            Json(e.into())
        })
}

/// Get the threads of a specific category
#[get("/category/<id>/threads")]
fn get_threads_category(
    id: Option<CategoryId>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id give error.

    info!("Requesting all threads from category with id {:?}", id);

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let threads_payload: GetThreadsPayload = GetThreadsPayload { id, include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_threads_in_category(threads_payload)
        .map(|v| {
            info!("Returning success from 'get-threads-of-category' request");
            Json(ContentSuccess::Threads(v))
        }).map_err(|e| {
            error!("Unable to 'get-threads-of-category': {:?}", e);
            Json(e.into())
        })
}

/// Get a thread based on id
///
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
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
///
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
#[get("/thread/<id>")]
fn get_thread(
    id: Option<ThreadId>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id, give error.

    info!("Getting thread with id {:?}", id);

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let thread_payload: GetThreadPayload = GetThreadPayload { id, include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_thread(thread_payload)
        .map(|v| {
            info!("Returning success from 'get-thread' request");
            Json(ContentSuccess::Thread(v))
        }).map_err(|e| {
            error!("Unable to 'get-thread': {:?}", e);
            Json(e.into())
        })
}

/// Get all threads (limited)
#[get("/threads")]
fn get_threads(opt_token: Option<Token>) -> JsonResponseResult<ContentSuccess> {
    info!("Requesting all threads");

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let hidden_payload: GetHiddenPayload = GetHiddenPayload { include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_all_threads(hidden_payload)
        .map(|v| {
            info!("Returning success from 'get-threads' request");
            Json(ContentSuccess::Threads(v))
        }).map_err(|e| {
            error!("Unable to 'get-threads': {:?}", e);
            Json(e.into())
        })
}

/// Get a threads comments.
#[get("/thread/<id>/comments")]
fn get_comments_in_thread(
    id: Option<ThreadId>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id, give error.

    info!("Requesting all comments from thread with id {:?}", id);

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let comments_payload: GetCommentsPayload = GetCommentsPayload { id, include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_comments_in_thread(comments_payload)
        .map(|v| {
            info!("Returning success from 'get-comments-of-thread' request");
            Json(ContentSuccess::Comments(v))
        }).map_err(|e| {
            error!("Unable to 'get-comments-of-thread': {:?}", e);
            Json(e.into())
        })
}

/// Get a comment
///
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
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
///         "description": "Hello everybody.",
///         "timestamp": 201820901206
///     }
/// }
#[get("/comment/<id>")]
fn get_comment(
    id: Option<CommentId>,
    opt_token: Option<Token>,
) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id, give error.

    info!("Requesting comment with id {:?}", id);

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let comment_payload: GetCommentPayload = GetCommentPayload { id, include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_comment(comment_payload)
        .map(|v| {
            info!("Returning success from 'get-comment' request");
            Json(ContentSuccess::Comment(v))
        }).map_err(|e| {
            error!("Unable to 'get-comment': {:?}", e);
            Json(e.into())
        })
}

/// Get all comment (limited)
#[get("/comments")]
fn get_comments(opt_token: Option<Token>) -> JsonResponseResult<ContentSuccess> {
    info!("Requesting all comments");

    // If logged in as admin/mod, then include hidden elements in result, if not exclude hidden elements.
    let include_hidden: bool = is_admin_or_mod(opt_token).map_err(|e| Json(e))?;
    let hidden_payload: GetHiddenPayload = GetHiddenPayload { include_hidden };

    connect_to_controller()
        .map_err(Json)?
        .get_all_comments(hidden_payload)
        .map(|v| {
            info!("Returning success from 'get-comments' request");
            Json(ContentSuccess::Comments(v))
        }).map_err(|e| {
            error!("Unable to 'get-comments': {:?}", e);
            Json(e.into())
        })
}

/// Get user info based id
///
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
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
///
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
fn get_user(id: Option<UserId>) -> JsonResponseResult<ContentSuccess> {
    let id = id
        .ok_or(ContentError::InvalidId)
        .map_err(|e| Json(e.into()))?; // If invalid id give error.

    info!("Requesting user with id {:?}", id);

    let user_payload = GetUserPayload { id };

    connect_to_controller()
        .map_err(Json)?
        .get_user(user_payload)
        .map(|v| {
            info!("Returning success from 'get-user' request");
            Json(ContentSuccess::User(v))
        }).map_err(|e| {
            error!("Unable to 'get-user': {:?}", e);
            Json(e.into())
        })
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
/// # Error
///
/// You get back the error as a type.
///
/// ## Example
///
/// ´´´json
/// {
///     type: "INTERNAL_SERVER_ERROR"
/// }
/// ´´´
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
pub fn post_content(
    token: Token,
    req: Option<Json<ContentRequest>>,
) -> JsonResponseResult<ContentSuccess> {
    use datatypes::content::requests::ContentRequest::*;

    let req = req
        .ok_or(ContentError::InvalidContent)
        .map_err(|e| Json(e.into()))?; // If invalid request give error.

    // Check what role the user has (and that a user is valid):
    let role = connect_to_auth()
        .map_err(Json)?
        .get_user_role(TokenPayload::new(None, token))
        .map_err(|e| Json(e.into()))?;

    match req.into_inner() {
        AddCategory(p) => {
            // Relays what is sent back to the user
            // If not allowed to do this, return errormessage:
            if role < Role::Moderator {
                Err(ResponseError::Unauthorized).map_err(|e| Json(e))?;
            }

            info!("Forwarding a 'add-category' request");
            connect_to_controller()
                .map_err(Json)?
                .add_category(p)
                .map(|v| {
                    info!("Returning success from 'add-category' request");
                    Json(ContentSuccess::Category(v))
                }).map_err(|e| {
                    error!("Unable to 'add-category': {:?}", e);
                    Json(e.into())
                })
        }
        EditCategory(p) => {
            // Relays what is sent back to the user
            // If not allowed to do this, return errormessage:
            if role < Role::Moderator {
                Err(ResponseError::Unauthorized).map_err(|e| Json(e))?;
            }

            info!("Forwarding a 'edit-category' request");
            connect_to_controller()
                .map_err(Json)?
                .edit_category(p)
                .map(|v| {
                    info!("Returning success from 'edit-category' request");
                    Json(ContentSuccess::Category(v))
                }).map_err(|e| {
                    error!("Unable to 'edit-category': {:?}", e);
                    Json(e.into())
                })
        }
        HideCategory(p) => {
            // Relays what is sent back to the user
            // If not allowed to do this, return errormessage:
            if role < Role::Admin {
                Err(ResponseError::Unauthorized).map_err(|e| Json(e))?;
            }

            info!("Forwarding a 'hide-category' request");
            connect_to_controller()
                .map_err(Json)?
                .hide_category(p)
                .map(|v| {
                    info!("Returning success from 'hide-category' request");
                    Json(ContentSuccess::Category(v))
                }).map_err(|e| {
                    error!("Unable to 'hide-category': {:?}", e);
                    Json(e.into())
                })
        }
        AddThread(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'add-thread' request");
            connect_to_controller()
                .map_err(Json)?
                .add_thread(p)
                .map(|v| {
                    info!("Returning success from 'add-thread' request");
                    Json(ContentSuccess::Thread(v))
                }).map_err(|e| {
                    error!("Unable to 'add-thread': {:?}", e);
                    Json(e.into())
                })
        }
        EditThread(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'edit-thread' request");
            connect_to_controller()
                .map_err(Json)?
                .edit_thread(p)
                .map(|v| {
                    info!("Returning success from 'edit-thread' request");
                    Json(ContentSuccess::Thread(v))
                }).map_err(|e| {
                    error!("Unable to 'edit-thread': {:?}", e);
                    Json(e.into())
                })
        }
        HideThread(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'hide-thread' request");
            connect_to_controller()
                .map_err(Json)?
                .hide_thread(p)
                .map(|v| {
                    info!("Returning success from 'hide-thread' request");
                    Json(ContentSuccess::Thread(v))
                }).map_err(|e| {
                    error!("Unable to 'hide-thread': {:?}", e);
                    Json(e.into())
                })
        }
        AddComment(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'add-comment' request");
            connect_to_controller()
                .map_err(Json)?
                .add_comment(p)
                .map(|v| {
                    info!("Returning success from 'add-comment' request");
                    Json(ContentSuccess::Comment(v))
                }).map_err(|e| {
                    error!("Unable to 'add-comment': {:?}", e);
                    Json(e.into())
                })
        }
        EditComment(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'edit-comment' request");
            connect_to_controller()
                .map_err(Json)?
                .edit_comment(p)
                .map(|v| {
                    info!("Returning success from 'edit-comment' request");
                    Json(ContentSuccess::Comment(v))
                }).map_err(|e| {
                    error!("Unable to 'edit-comment': {:?}", e);
                    Json(e.into())
                })
        }
        HideComment(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'hide-comment' request");
            connect_to_controller()
                .map_err(Json)?
                .hide_comment(p)
                .map(|v| {
                    info!("Returning success from 'hide-comment' request");
                    Json(ContentSuccess::Comment(v))
                }).map_err(|e| {
                    error!("Unable to 'hide-comment': {:?}", e);
                    Json(e.into())
                })
        }
        AddUser(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'add-user' request");
            connect_to_controller()
                .map_err(Json)?
                .add_user(p)
                .map(|v| {
                    info!("Returning success from 'add-user' request");
                    Json(ContentSuccess::User(v))
                }).map_err(|e| {
                    error!("Unable to 'add-user': {:?}", e);
                    Json(e.into())
                })
        }
        EditUser(p) => {
            // Relays what is sent back to the user
            info!("Forwarding a 'edit-user' request");
            connect_to_controller()
                .map_err(Json)?
                .edit_user(p)
                .map(|v| {
                    info!("Returning success from 'edit-user' request");
                    Json(ContentSuccess::User(v))
                }).map_err(|e| {
                    error!("Unable to 'edit-user': {:?}", e);
                    Json(e.into())
                })
        }
    }
}
