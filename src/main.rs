#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::Cookies;

#[derive(FromForm)]
struct LogIn {
    username: String,
    password: String,
}

#[derive(FromForm)]
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