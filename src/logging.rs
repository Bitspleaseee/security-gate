use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response, Rocket};
use std::io;

pub struct RocketLogger;

impl Fairing for RocketLogger {
    fn info(&self) -> Info {
        Info {
            name: "logger",
            kind: Kind::Launch | Kind::Request | Kind::Response,
        }
    }

    // Log startup information about rocket
    fn on_launch(&self, rocket: &Rocket) {
        info!("starting security-gate");

        for route in rocket.routes() {
            info!("route {} added", route);
        }

        info!("Rocket launch config: {:?}", rocket.config());
    }

    fn on_request(&self, req: &mut Request, _: &Data) {
        match req.remote() {
            Some(addr) => info!("[{}] {} {}", addr, req.method(), req.uri()),
            None => info!("[-.-.-.-] {} {}", req.method(), req.uri()),
        }
    }

    // Log all relevant information about the response
    fn on_response(&self, req: &Request, _: &mut Response) {
        match req.remote() {
            Some(addr) => {
                info!("[{}] {} {}: Responding", addr, req.method(), req.uri());
            }
            None => {
                info!("[-.-.-.-] {} {}: Responding", req.method(), req.uri());
            }
        }
    }
}

pub fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();
    //let ip = socket.expect("failed to get ip-adress of user.").ip();

    base_config = match verbosity {
        0 => base_config.level(log::LevelFilter::Info),

        1 => base_config
            .level(log::LevelFilter::Debug)
            .level_for("mio", log::LevelFilter::Info)
            .level_for("tarpc", log::LevelFilter::Info)
            .level_for("hyper", log::LevelFilter::Info)
            .level_for("tokio_io", log::LevelFilter::Info)
            .level_for("tokio_core", log::LevelFilter::Info)
            .level_for("tokio_proto", log::LevelFilter::Info)
            .level_for("tokio_reactor", log::LevelFilter::Info)
            .level_for("tokio_threadpool", log::LevelFilter::Info),

        2 => base_config
            .level(log::LevelFilter::Trace)
            .level_for("mio", log::LevelFilter::Info)
            .level_for("tarpc", log::LevelFilter::Info)
            .level_for("hyper", log::LevelFilter::Info)
            .level_for("tokio_io", log::LevelFilter::Info)
            .level_for("tokio_core", log::LevelFilter::Info)
            .level_for("tokio_proto", log::LevelFilter::Info)
            .level_for("tokio_reactor", log::LevelFilter::Info)
            .level_for("tokio_threadpool", log::LevelFilter::Info),

        _3_or_more => base_config.level(log::LevelFilter::Trace),
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
