#![feature(plugin)]
#![feature(try_from)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rocket;

use std::convert::{TryFrom, TryInto};
use rocket::http::{Cookies, RawStr};
use rocket::request::FromFormValue;
use regex::Regex;
use rocket::response::content::Content;
use rocket::http::ContentType;


/// The regex which vertifies that a username is formatted correctly
const USERNAME_REGEX: &'static str = "^[a-zA-Z0-9_-]{4,10}$";

/// A valid username based on a regex
struct Username(String);

impl<'v> FromFormValue<'v> for Username {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Username, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}

impl<'v> FromFormValue<'v> for Password {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Password, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}


impl<'v> TryFrom<&'v str> for Username {
    type Error = &'v str;

    fn try_from(s: &'v str) -> Result<Username, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(USERNAME_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(Username(s.into()))
        } else {
            Err(s)
        }
    }

}

impl<'v> TryFrom<&'v str> for Password {
    type Error = &'v str;

    fn try_from(s: &'v str) -> Result<Password, Self::Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(USERNAME_REGEX).unwrap();
        }
        if RE.is_match(s) {
            Ok(Password(s.into()))
        } else {
            Err(s)
        }
    }

}

struct LogInResponse {
    ok: bool,
    id: u32,
    token: String
}

struct OkResponse {
    ok: bool,
    message: String
}

#[derive_FromForm]
struct LogIn {
    username: Result<Username, &'vRawStr>,
    password: Result<Password, &'vRawStr>
}

#[derive_FromForm]
struct Comment {
    content: String,
    thread: u32
}

#[derive_FromForm]
struct Thread {
    title: String,
    description: String,
    category: u32
}

#[derive_FromForm]
struct Category {
    title: String,
    description: String
}

#[derive_FromForm]
struct AdminUsergroups {
    uid: u32,
    role: u32
}

#[derive_FromForm]
struct User {
    username: Result<Username, &'vRawStr>,
    password: Result<Password, &'vRawStr>,
    email: String,
    description: String,
    uid: u32
}

#[derive_FromForm]
struct Avatar {
    avatar: String
    uid: u32
}

#[catch(404)]
fn not_found(req: &Request) -> String { 
    "The website was not found"
}

#[catch(400)]
fn not_found(req: &Request) -> String { 
    "We encontered an error when processing your request"
}

fn log(message: String, remote_addr: String) {
    format!("Remote Address: {:?}", remote_addr)
}

#[get("/")]
fn index(remote_addr: SocketAddr) -> &'static str {
    log("Retriving webpage", remote_addr);                  // Log action.
    "Hello, world!"
}

/// Retrieve the user's ID, if any.
#[post("/login", format = "application/x-www-form-urlencoded", data = "<input>")]
fn login(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = login(input);
        cookies.add_private(Cookie::new("user_token", result.token));
        let ret = OkResponse {
            ok: result.ok,
            message: ""
        };
        log(format!("Logged in as user: {}", input.username), remote_addr);                  // Log action.
        Json(ret);
    } else {
        let ret = OkResponse {
            ok: false,
            message: ""
        }
        log(format!("Tried to log in as user: {}", input.username), remote_addr);                  // Log action.
        JSON(ret);
  }
}

/// Register user.
#[post("/register", format = "application/x-www-form-urlencoded", data = "<input>")]
fn register(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = register(input);
        log(format!("Sent request to register new user with username: {}", input.username), remote_addr);                  // Log action.
        Json(result);
    } else {
        let ret = OkResponse {
            ok: false,,
            message: ""
        }
        log(format!("Failed regestering new user with username: {}", input.username), remote_addr);                  // Log action.
        JSON(ret);
  }
}

/// Retrieve the user's profile.
#[get("/user/<username>")]
fn showUserProfile(username: Username, remote_addr: SocketAddr) -> String {
    //result = controller.getUserProfile(username);
    log(format!("Sent request to controller about public profile of user with username: {}", input.username), remote_addr);                  // Log action.
    JSON(result);
}

// Retrive other user-information like email and role
#[get("/user/<username>/private")]
fn showUserProfile(cookies: Cookies, username: Username, remote_addr: SocketAddr) -> String {
    //result = auth.getUserProfile(cookies.get_private("user_token"), username);
    log(format!("Sent request to controller about private profile of user with username: {}", input.username), remote_addr);                  // Log action.
    JSON(result);
}

/// Search.
#[get("/search/<searchStr>")]
fn search(searchStr: String, remote_addr: SocketAddr) -> String {
    //result = controller.search(searchStr);
    log(format!("Sent search request to controller. Search-String: {}", searchString), remote_addr);                  // Log action.
    JSON(result);
}

/// Remove the `user_token` cookie and tell the authorization module to delete the token.
#[post("/logout")]
fn logout(mut cookies: Cookies, remote_addr: SocketAddr) -> Json<OkResponse> {
    //auth.logout(cookies.get_private("user_token"))
    cookies.remove_private(Cookie::named("user_token"));
    let ret = OkResponse {
        ok: true
    };
    log(format!("Lgged out: Took away cookie and told auth-module"), remote_addr);                  // Log action.
    JSON(ret);
}

/// Get all categories.
#[get("/category")]
fn getAllCategories(remote_addr: SocketAddr) -> String {
    //let result = controller.getAllCategories();
    log(format!("Sent request of getting list of all categories to controller"), remote_addr);                  // Log action.
    JSON(result);
}

/// Get all threads in category.
#[get("/category/<category>")]
fn getCategory(category: u32, remote_addr: SocketAddr) -> String {
    //let result = controller.getCategory(category);
    log(format!("Sent request of getting category with id {} to controller", category), remote_addr);                  // Log action.
    JSON(result);
}

/// Get all comments in a thread.
#[get("/thread/<thread>")]
fn getThread(thread: u32, remote_addr: SocketAddr) -> String {
    //let result = controller.getThread(thread);
    log(format!("Sent request of getting thread with id {} to controller", thread), remote_addr);                  // Log action.
    JSON(result);
}

/// Comment on a thread.
#[post("/comment/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn comment(cookies: Cookies, input: Form<Comment>, remote_addr: SocketAddr) -> String {
    //result = authverifyUser(cookies.get_private("user_token"))
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.comment(input, result);
        log(format!("Sent request to let user {} comment thread {} to controller", result.username, input.thread), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Make a new thread.
#[post("/thread/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addThread(cookies: Cookies, input: Form<Thread>, remote_addr: SocketAddr) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.addThread(input, result);
        log(format!("Sent request to add new thread. Will be added by user {}", result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Make a new thread.
#[post("/category/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addCategory(cookies: Cookies, input: Form<Category>, remote_addr: SocketAddr) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true && result.role > 1 {                                      // If token is correct and role is moderator or above.
        //let ret = controller.addCategory(input, result);
        log(format!("Sent request to add new thread. Will be added by user {}", result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Edit user
#[post("/user/edit", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editUser(cookies: Cookies, input: Form<User>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret1 = auth.editUser(input, result);
        //let ret2 = controller.editUser(input, result);
        let ret = OkResponse {
            ok: false,
            message: "Wrong input"
        }
        if ret1.ok == true && ret2.ok == true {           // Check that both functions worked correctly
            ret.ok = true;
            ret.message = "";
        }
        log(format!("Sent request to edit user to controller. Editing will be done by user {} on user {}", result.username, input.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Upload a new avatar.
#[post("/user/avatar/upload", format = "multipart/form-data", data = "<input>")]
fn uploadAvatar(cookies: Cookies, input: Form<Avatar>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.uploadAvatar(input, result);
        log(format!("Sent request to upload avatar for user with id {} by user ", input.uid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Edit category.
#[post("/category/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editCategory(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.editCategory(input, result, cid);
        log(format!("Sent request to edit category with id {}, for user {} to controller", cid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Edit thread.
#[post("/thread/edit/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editThread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editThread(input, result, tid);
        log(format!("Sent request to edit thread with id {}, for user {} to controller", tid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Edit comment.
#[post("/comment/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editComment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editComment(input, result, cid);
        log(format!("Sent request to edit comment with id {}, for user {} to controller", cid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Hide category.
#[post("/category/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideCategory(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.hideCategory(input, result, cid);
        log(format!("Sent request to hide category with id {}, for user {} to controller", cid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Hide thread.
#[post("/thread/hide/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideThread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideThread(input, result, tid);
        log(format!("Sent request to hide thread with id {}, for user {} to controller", tid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Hide comment.
#[post("/comment/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideComment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideComment(input, result, cid);
        log(format!("Sent request to hide comment with id {}, for user {} to controller", cid, result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

/// Administer usergroups.
#[post("/admin/edit/usergroups", format = "application/x-www-form-urlencoded", data = "<input>")]
fn adminUsergroups(cookies: Cookies, input: Form<AdminUsergroups>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    log(format!("Sent request to verify user to auth-module"), remote_addr);                  // Log action.
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = auth.administerUsergroups(input, result);
        log(format!("Sent request to auth-module to administer usergroups for user {}", result.username), remote_addr);                  // Log action.
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        log(format!("Verify-request rejected"), remote_addr);                  // Log action.
        JSON(ret);
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, login, register, showUserProfile, search, logout, 
        getAllCategories, getCategory, getThread, comment, addCategory, addThread, editUser, uploadAvatar,
        editCategory, editThread, editComment, hideCategory, hideThread, hideComment, adminUsergroups]).launch();
}
