#![feature(plugin)]
#![feature(try_from)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rocket;
#[macro_use]
extern crate log;
extern crate failure;
#[macro_use]
extern crate failure_derive;

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

/*impl<'v> FromFormValue<'v> for Password {
    type Error = &'v RawStr;

    fn from_form_value(value: &'v RawStr) -> Result<Password, Self::Error> {
        value.as_str().try_into().map_err(|_| value)
    }
}
*/

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

/*impl<'v> TryFrom<&'v str> for Password {
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

}*/

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
struct LogIn<'v> {
    username: Result<Username, &'v RawStr>,
    password: String //Result<Password, &'v RawStr>
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
struct User<'v> {
    username: Result<Username, &'v RawStr>,
    password: String, //Result<Password, &'v RawStr>,
    email: String,
    description: String,
    role: String,
    uid: u32
}

#[derive_FromForm]
struct Avatar {
    avatar: String,
    uid: u32
}

/*
#[catch(400)]
fn not_found(req: &Request, remote_addr: SocketAddr) -> String {
   // info!("{}: requested invalid URI {}", remote_addr, req.uri());
    "We encontered an error when processing your request".toString()
}*/

#[get("/")]
fn index(/*remote_addr: SocketAddr*/) -> &'static str {
   // info!("{}: retriving webpage", remote_addr);
    "Hello, world!"
}

/*/// Retrieve the user's ID, if any.
#[post("/login", format = "application/x-www-form-urlencoded", data = "<input>")]
fn login(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = login(input);
        cookies.add_private(Cookie::new("user_token", result.token));
        let ret = OkResponse {
            ok: result.ok,
            message: ""
        };
        info!("{}: logged in as user: {}", remote_addr, input.username);
        Json(ret);
    } else {
        let ret = OkResponse {
            ok: false,
            message: ""
        }
        info!("{}: tried to log in as user: {}", remote_addr, input.username);
        JSON(ret);
  }
}

/// Register user.
#[post("/register", format = "application/x-www-form-urlencoded", data = "<input>")]
fn register(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = register(input);
        info!("{}: sent request to register new user with username: {}", remote_addr, input.username);
        Json(result);
    } else {
        let ret = OkResponse {
            ok: false,,
            message: ""
        }
        info!("{}: failed regestering new user with username: {}", remote_addr, input.username);
        JSON(ret);
  }
}

/// Retrieve the user's profile.
#[get("/user/<username>")]
fn showUserProfile(username: Username, remote_addr: SocketAddr) -> String {
    //result = controller.getUserProfile(username);
    info!("{}: sent request to controller about public profile of user with username: {}", remote_addr, input.username);
    JSON(result);
}

// Retrive other user-information like email and role
#[get("/user/<username>/private")]
fn showUserProfile(cookies: Cookies, username: Username, remote_addr: SocketAddr) -> String {
    //result = auth.getUserProfile(cookies.get_private("user_token"), username);
    info!("{}: sent request to controller about private profile of user with username: {}", remote_addr, input.username);
    JSON(result);
}

/// Search.
#[get("/search/<searchStr>")]
fn search(searchStr: String, remote_addr: SocketAddr) -> String {
    //result = controller.search(searchStr);
    info!("{}: sent search request to controller. search-string: {}", remote_addr, searchstring);
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
    info!("{}: lgged out: took away cookie and told auth-module", remote_addr);
    JSON(ret);
}

/// Get all categories.
#[get("/category")]
fn getAllCategories(remote_addr: SocketAddr) -> String {
    //let result = controller.getAllCategories();
    info!("{}: sent request of getting list of all categories to controller", remote_addr);
    JSON(result);
}

/// Get all threads in category.
#[get("/category/<category>")]
fn getCategory(category: u32, remote_addr: SocketAddr) -> String {
    //let result = controller.getCategory(category);
    info!("{}: sent request of getting category with id {} to controller", remote_addr, category);
    JSON(result);
}

/// Get all comments in a thread.
#[get("/thread/<thread>")]
fn getThread(thread: u32, remote_addr: SocketAddr) -> String {
    //let result = controller.getThread(thread);
    info!("{}: sent request of getting thread with id {} to controller", remote_addr, thread);
    JSON(result);
}

/// Comment on a thread.
#[post("/comment/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn comment(cookies: Cookies, input: Form<Comment>, remote_addr: SocketAddr) -> String {
    //result = authverifyUser(cookies.get_private("user_token"))
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

/// Make a new thread.
#[post("/thread/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addThread(cookies: Cookies, input: Form<Thread>, remote_addr: SocketAddr) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.addThread(input, result);
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

/// Make a new thread.
#[post("/category/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addCategory(cookies: Cookies, input: Form<Category>, remote_addr: SocketAddr) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role > 1 {                                      // If token is correct and role is moderator or above.
        //let ret = controller.addCategory(input, result);
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

/// Edit user
#[post("/user/edit", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editUser(cookies: Cookies, input: Form<User>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
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
        info!("{}: sent request to edit user to controller. editing will be done by user {} on user {}", remote_addr, result.username, input.username);
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

/// Upload a new avatar.
#[post("/user/avatar/upload", format = "multipart/form-data", data = "<input>")]
fn uploadAvatar(cookies: Cookies, input: Form<Avatar>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.uploadAvatar(input, result);
        info!("{}: sent request to upload avatar for user with id {} by user ", remote_addr, input.uid, result.username);
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
fn editCategory(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.editCategory(input, result, cid);
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

/// Edit thread.
#[post("/thread/edit/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editThread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editThread(input, result, tid);
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

/// Edit comment.
#[post("/comment/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editComment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editComment(input, result, cid);
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

/// Hide category.
#[post("/category/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideCategory(cookies: Cookies, input: Form<Category>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.hideCategory(input, result, cid);
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

/// Hide thread.
#[post("/thread/hide/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideThread(cookies: Cookies, input: Form<Thread>, tid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideThread(input, result, tid);
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

/// Hide comment.
#[post("/comment/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideComment(cookies: Cookies, input: Form<Comment>, cid: u32, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideComment(input, result, cid);
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

/// Administer usergroups.
#[post("/admin/edit/usergroups", format = "application/x-www-form-urlencoded", data = "<input>")]
fn adminUsergroups(cookies: Cookies, input: Form<AdminUsergroups>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = auth.administerUsergroups(input, result);
        info!("{}: sent request to auth-module to administer usergroups for user {}", remote_addr, result.username);
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
}*/

fn main() {
    rocket::ignite().mount("/", routes![index/*, login, register, showUserProfile, search, logout, getAllCategories, getCategory, getThread, comment, addCategory, addThread, editUser, uploadAvatar, editCategory, editThread, editComment, hideCategory, hideThread, hideComment, adminUsergroups*/]).launch();
}
