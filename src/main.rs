#![feature(plugin)]
#![feature(try_from)]
#![feature(custom_derive)]
#![feature(tool_lints)]
#![plugin(rocket_codegen)]
#![plugin(tarpc_plugins)]
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
#[macro_use]
extern crate tarpc;

use rocket::config::{Config, Environment};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub mod auth;
pub mod banned;
pub mod comms;
pub mod content;
pub mod logging;

/// Convenience wrapper around a `Result` of `Json` values
type JsonResponseResult<T> =
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

    //Getting adress and port from env-variables if possible:
    let address = match std::env::var("SECURITY_GATE_ADDRESS") {
        Ok(value) => value.to_string(),
        Err(_) => {
            warn!("SECURITY_GATE_ADDRESS is not set, using 'localhost'");
            "localhost".to_string()
        }
    };

    let port = match std::env::var("SECURITY_GATE_PORT") {
        Ok(value) => value.to_string(),
        Err(_) => {
            warn!("SECURITY_GATE_PORT is not set, using '9234'");
            "9234".to_string()
        }
    };

    crate::comms::controller::CONTROLLER_IP = match std::env::var("CONTROLLER_ADDRESS") {
        Ok(value) => value.to_string(),
        Err(_) => {
            warn!("CONTROLLER_ADDRESS is not set, using 'localhost:10000'");
            "localhost:10000".to_string()
        }
    };

    crate::comms::auth::AUTH_IP = match std::env::var("AUTH_ADDRESS") {
        Ok(value) => value.to_string(),
        Err(_) => {
            warn!("AUTH_ADDRESS is not set, using 'localhost:10001'");
            "localhost:10001".to_string()
        }
    };

    // Configuring rocket:
    let config = Config::build(Environment::Staging)
        .address(address)                                   // Set address
        .port(port.parse::<u16>().unwrap_or(9234))          // Set port and be sure it is a number
        .finalize()
        .expect("failed to instantiate config");

    info!("igniting rocket");
    rocket::custom(config, false)
        .attach(logging::RocketLogger)
        .attach(banned::BanIpAddrs::default())
        .attach(ModifyResponseHeaders)
        .mount(
            "/",
            routes![content::index, content::static_file, banned::banned_message],
        ).mount(
            "/api/",
            routes![
                banned::post_admin,
                auth::auth,
                content::search,
                content::get_category,
                content::get_categories,
                content::get_thread,
                content::get_threads,
                content::get_comment,
                content::get_comments,
                content::get_threads_category,
                content::get_comments_in_thread,
                content::get_user,
                content::post_content
            ],
        ).launch();
}

pub struct ModifyResponseHeaders;

impl Fairing for ModifyResponseHeaders {
    fn info(&self) -> Info {
        Info {
            name: "alter generic headers (e.g. CSP header)",
            kind: Kind::Response,
        }
    }
    fn on_response(&self, _: &Request, res: &mut Response) {
        res.set_header(
            Header::new(
                "Content-Security-Policy",
                "default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self';"
            )
        );
    }
}
