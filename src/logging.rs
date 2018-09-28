use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response, Rocket};
use std::io;

pub struct RocketLogger;

impl Fairing for RocketLogger {
    fn info(&self) -> Info {
        Info {
            name: "A fairing which logs all events",
            kind: Kind::Launch | Kind::Request | Kind::Response,
        }
    }

    fn on_launch(&self, rocket: &Rocket) {
        // TODO log startup information about rocket
        info!("starting security-gate");
    }

    fn on_request(&self, req: &mut Request, _data: &Data) {
        match req.remote() {
            Some(addr) => info!("[{}] {} {}", addr, req.method(), req.uri()),
            None => info!("[-.-.-.-] {} {}", req.method(), req.uri()),
        }
    }

    fn on_response(&self, req: &Request, res: &mut Response) {
        // TODO log all relevant information about the response
    }
}

pub fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();
    //let ip = socket.expect("failed to get ip-adress of user.").ip();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Info),
        1 => base_config.level(log::LevelFilter::Debug),
        _2_or_more => base_config.level(log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        }).chain(fern::log_file("security-gate.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            // special format for debug messages coming from our own crate.
            if record.level() > log::LevelFilter::Info && record.target() == "security-gate" {
                out.finish(format_args!(
                    "---\nDEBUG: {}: {}\n---",
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    chrono::Local::now().format("%H:%M"),
                    record.target(),
                    record.level(),
                    message
                ))
            }
        }).chain(io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}
