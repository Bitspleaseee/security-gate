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
    role: String
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

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Retrieve the user's ID, if any.
#[post("/login", format = "application/x-www-form-urlencoded", data = "<input>")]
fn login(input: Form<LogIn>) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = login(input);
        cookies.add_private(Cookie::new("user_token", result.token));
        let ret = OkResponse {
            ok: result.ok,
            message: ""
        };
        Json(ret);
    } else {
        let ret = OkResponse {
            ok: false,
            message: ""
        }
        JSON(ret);
  }
}

/// Register user.
#[post("/register", format = "application/x-www-form-urlencoded", data = "<input>")]
fn register(input: Form<LogIn>) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = register(input);
        let ret = OkResponse {
            ok: result.ok
        };
        Json(ret);
    } else {
        let ret = OkResponse {
            ok: false,,
            message: ""
        }
        JSON(ret);
  }
}

/// Retrieve the user's profile.
#[get("/user/<username>")]
fn showUserProfile(username: String) -> String {
    //result = controller.getUserProfile(username);
    JSON(result);
}

// Retrive other user-information like email and role
#[get("/user/<username>/private")]
fn showUserProfile(cookies: Cookies, username: String) -> String {
    //result = auth.getUserProfile(cookies.get_private("user_token"), username);
    JSON(result);
}

/// Search.
#[get("/search/<searchStr>")]
fn search(cookies: Cookies, searchStr: String) -> String {
    //result = controller.search(searchStr);
    JSON(result);
}

/// Remove the `user_token` cookie and tell the authorization module to delete the token.
#[post("/logout")]
fn logout(mut cookies: Cookies) -> Json<OkResponse> {
    //auth.logout(cookies.get_private("user_token"))
    cookies.remove_private(Cookie::named("user_token"));
    let ret = OkResponse {
        ok: true
    };
    JSON(ret);
}

/// Get all categories.
#[get("/category")]
fn getAllCategories() -> String {
    //let result = controller.getAllCategories();
    JSON(result);
}

/// Get all threads in category.
#[get("/category/<category>")]
fn getCategory(category: u32) -> String {
    //let result = controller.getCategory(category);
    JSON(result);
}

/// Get all comments in a thread.
#[get("/thread/<thread>")]
fn getThread(thread: u32) -> String {
    //let result = controller.getThread(thread);
    JSON(result);
}

/// Comment on a thread.
#[post("/comment/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn comment(cookies: Cookies, input: Form<Comment>) -> String {
    //result = authverifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.comment(input, result);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Make a new thread.
#[post("/thread/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addThread(cookies: Cookies, input: Form<Thread>) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.addThread(input, result);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Make a new thread.
#[post("/category/new", format = "application/x-www-form-urlencoded", data = "<input>")]
fn addCategory(cookies: Cookies, input: Form<Category>) -> String {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.addCategory(input, result);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Edit user
#[post("/user/edit", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editUser(cookies: Cookies, input: Form<User>) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
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
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Upload a new avatar.
#[post("/user/avatar/upload", format = "multipart/form-data", data = "<input>")]
fn uploadAvatar(cookies: Cookies, input: Form<Avatar>) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.uploadAvatar(input, result);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Edit category.
#[post("/category/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editCategory(cookies: Cookies, input: Form<Category>, cid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.editCategory(input, result, cid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Edit thread.
#[post("/thread/edit/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editThread(cookies: Cookies, input: Form<Thread>, tid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editThread(input, result, tid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Edit comment.
#[post("/comment/edit/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editComment(cookies: Cookies, input: Form<Comment>, cid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.editComment(input, result, cid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Hide category.
#[post("/category/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editCategory(cookies: Cookies, input: Form<Category>, cid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = controller.hideCategory(input, result, cid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Hide thread.
#[post("/thread/hide/<tid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideThread(cookies: Cookies, input: Form<Thread>, tid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideThread(input, result, tid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Hide comment.
#[post("/comment/hide/<cid>", format = "application/x-www-form-urlencoded", data = "<input>")]
fn hideComment(cookies: Cookies, input: Form<Comment>, cid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.hideComment(input, result, cid);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

/// Administer usergroups.
#[post("/admin/edit/usergroups", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editCategory(cookies: Cookies, input: Form<AdminUsergroups>, cid: u32) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"))
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = auth.administerUsergroups(input, result);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        JSON(ret);
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, login, register, showUserProfile, search, logout, 
        getAllCategories, getCategory, getThread, comment, addCategory, addThread, editUser, uploadAvatar,
        editCategory, editThread, editComment]).launch();
}
