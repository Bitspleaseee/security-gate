#![feature(plugin)]
#![feature(try_from)]
#![plugin(rocket_codegen)]
// These clippy lints are unreasonable, hence they are disabled
#![feature(tool_lints)]
#![allow(clippy::suspicious_else_formatting)]
#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate log;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod auth;
mod content;

fn main() {
    rocket::ignite()
        .mount("/", routes![auth::routes::login, content::routes::index])
        .launch();
}
