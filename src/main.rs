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

#[derive_FromForm]
struct LogIn {
    username: Username,
    password: String,
}

#[derive_FromForm]
struct CommentThread {
    content: String,
    thread: i32
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Retrieve the user's ID, if any.
#[post("/login", data = "<input>")]
fn login(input: Form<LogIn>) -> String {
    // result = login(input);
    cookies.add_private(Cookie::new("user_token", /*result.token*/));
}

/// Retrieve the user's profile.
#[get("/user/<username>")]
fn showUserProfile(cookies: Cookies, username: String) -> String {
    //getUserProfile(username);
    Ok("Username: {}", username);
}

/// Search.
#[get("/search/<searchStr>")]
fn search(cookies: Cookies, searchStr: String) -> String {
    //search(searchStr);
    Ok("Search string: {}", searchStr);
}

/// Remove the `user_token` cookie.
#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    //auth.logout(cookies.get_private("user_token"))
    cookies.remove_private(Cookie::named("user_token"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

/// Get all threads in category.
#[get("/category/<category>")]
fn search(cookies: Cookies, category: String) -> String {
    //getCategory(category);
}

/// Search.
#[get("/thread/<thread>")]
fn search(cookies: Cookies, thread: String) -> String {
    //getThread(thread);
}

/// Comment on a thread.
#[post("/comment", data = "<input>")]
fn comment(cookies: Cookies, input: Form<CommentThread>) -> String {
    //result = verifyUser(cookie.get_private("user_token"))
    if (result.ok == true) {
        //comment(input, result.id);
    }
    else {
        Err("Bad request");
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
