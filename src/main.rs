#![feature(plugin)]
#![feature(try_from)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
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
extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fern;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;

use rocket::config::{Config, Environment};

pub mod auth;
pub mod content;
pub mod logging;
pub mod banned;

/// Convenience wrapper around a `Result` of `Json` values
type JsonResult<T, E> = Result<rocket_contrib::Json<T>, rocket_contrib::Json<E>>;

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
    //rocket::ignite()
    rocket::custom(config, false)
        .attach(logging::RocketLogger)
        .attach(banned::BanIpAddrs::default())
        .mount(
            "/",
            routes![content::routes::index, content::routes::static_file, banned::bannedMessage],
        ).mount(
            "/api/",
            routes![
                auth::routes::auth,
                content::routes::search,
                content::routes::get_category,
                content::routes::get_thread,
                content::routes::get_comment,
                content::routes::get_threads_category,
                content::routes::get_comments_in_thread,
                content::routes::get_user,
                content::routes::post_content
            ],
        ).launch();
    // TODO change from `launch` to `custom` with a custom config (disable
    // default logging? + set IP and port from environment variables)
    //
    // See https://api.rocket.rs/rocket/struct.Rocket.html#method.custom
}
