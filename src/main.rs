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
extern crate failure;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate failure_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate clap;

mod auth;
mod content;
pub mod logging;

use rocket::fairing::AdHoc;
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;


/// Convenience wrapper around a `Result` of `Json` values
type JsonResult<T, E> = Result<rocket_contrib::Json<T>, rocket_contrib::Json<E>>;

fn main() {
    // Logging
    let cmd_arguments = clap::App::new("cmd-program")
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increases logging verbosity each use for up to 3 times"),
        )
        .get_matches();

    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");

    logging::setup_logging(verbosity).expect("failed to initialize logging.");

    info!("Starting program");

    rocket::ignite()
        // .attach(AdHoc::on_request(|req, _| {
        //     Builder::new()
        //         .format(|buf, record| {
        //             writeln!(buf,
        //                 "{} [{}] - IP {:?}: {}",
        //                 Local::now().format("%Y-%m-%dT%H:%M:%S"),
        //                 record.level(),
        //                 req.remote(),                    // Returns an ip or None if nothing is found.
        //                 record.args()
        //             )
        //         })
        //         .filter(None, LevelFilter::Info)
        //         .init();
        // }))
        .mount("/", routes![content::routes::index])
        .mount("/", routes![content::routes::static_file])
        .mount("/api/", routes![auth::routes::auth])
        .mount("/api", routes![content::routes::search])
        .mount("/api", routes![content::routes::get_category])
        .mount("/api", routes![content::routes::get_thread])
        .mount("/api", routes![content::routes::get_comment])
        .mount("/api", routes![content::routes::get_threads_category])
        .mount("/api", routes![content::routes::get_comments_in_thread])
        .mount("/api", routes![content::routes::get_user])
        .mount("/api", routes![content::routes::post_content])
        .launch();

        info!("Set up routes.");
}
