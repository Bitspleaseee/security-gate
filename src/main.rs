#![feature(plugin)]
#![feature(try_from)]
#![feature(custom_derive)]
#![feature(tool_lints)]
#![plugin(rocket_codegen)]
#![allow(
    clippy::suspicious_else_formatting,
    clippy::needless_pass_by_value,
    clippy::implicit_hasher
)]

extern crate lazy_static;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate clap;
extern crate fern;

use rocket::config::{Config, Environment};

pub mod auth;
pub mod banned;
pub mod content;
pub mod logging;

/// Convenience wrapper around a `Result` of `Json` values
type JsonResult<T> =
    Result<rocket_contrib::Json<T>, rocket_contrib::Json<datatypes::error::ResponseError>>;

fn main() {
    // Logging
    let cmd_arguments = clap::App::new("security-gate")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        ).get_matches();

    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");
    logging::setup_logging(verbosity).expect("failed to initialize logging.");

    // Configuring rocket:
    let config = Config::build(Environment::Staging)
        .address("localhost")
        .port(9234)
        .finalize()
        .expect("failed to instantiate config");

    info!("igniting rocket");
    rocket::custom(config, false)
        .attach(logging::RocketLogger)
        .attach(banned::BanIpAddrs::default())
        .mount(
            "/",
            routes![content::index, content::static_file, banned::banned_message],
        ).mount(
            "/api/",
            routes![
                banned::post_admin,
                auth::routes::auth,
                content::search,
                content::get_category,
                content::get_thread,
                content::get_comment,
                content::get_threads_category,
                content::get_comments_in_thread,
                content::get_user,
                content::post_content
            ],
        ).launch();
}
