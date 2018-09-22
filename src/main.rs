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
#[macro_use]
extern crate log;
extern crate failure;
#[macro_use]
extern crate failure_derive;

fn main() {
    rocket::ignite()
        .mount("/", routes![])
        .launch();
}
