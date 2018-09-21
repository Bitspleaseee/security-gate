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
struct NewComment {
    content: String,
    thread: u32
}

#[derive_FromForm]
struct NewThread {
    title: String,
    description: String,
    category: u32
}

#[derive_FromForm]
struct NewCategory {
    title: String,
    description: String,
}

#[derive_FromForm]
struct User {
    username: Result<Username, &'vRawStr>,
    password: Result<Password, &'vRawStr>,
    email: String,
    description: String,
    avatar: String
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
#[post("/login", data = "<input>")]
fn login(input: Form<LogIn>) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = login(input);
        cookies.add_private(Cookie::new("user_token", /*result.token*/));
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
#[post("/register", data = "<input>")]
fn login(input: Form<LogIn>) -> Json<OkResponse> {
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
fn showUserProfile(cookies: Cookies, username: String) -> String {
    //result = getUserProfile(username);
    JSON(result);
}

/// Search.
#[get("/search/<searchStr>")]
fn search(cookies: Cookies, searchStr: String) -> String {
    //result = search(searchStr);
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

/// Get all threads in category.
#[get("/category/<category>")]
fn search(cookies: Cookies, category: String) -> String {
    //let result = getCategory(category);
    JSON(result);
}

/// Search.
#[get("/thread/<thread>")]
fn search(cookies: Cookies, thread: String) -> String {
    //let result = getThread(thread);
    JSON(result);
}

/// Comment on a thread.
#[post("/comment", data = "<input>")]
fn comment(cookies: Cookies, input: Form<NewComment>) -> String {
    //result = verifyUser(cookies.get_private("user_token"))
    if result.ok == true {
        //let ret = comment(input, result.id);
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
#[post("/newThread", data = "<input>")]
fn addThread(cookies: Cookies, input: Form<NewThread>) -> String {
    //result = verifyUser(cookies.get_private("user_token"))
    if result.ok == true {
        //let ret = addThread(input, result.id);
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
#[post("/newCategory", data = "<input>")]
fn addCategory(cookies: Cookies, input: Form<NewCategory>) -> String {
    //result = verifyUser(cookies.get_private("user_token"))
    if result.ok == true {
        //let ret = addCategory(input, result.id);
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
#[post("/editUser", data = "<input>")]
fn editUser(cookies: Cookies, input: Form<User>) -> JSON<OkResponse> {
    //result = verifyUser(cookies.get_private("user_token"))
    if result.ok == true {
        //let ret1 = auth.editUser(input, result.id);
        //let ret2 = controller.editUser(input, result.id);
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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
