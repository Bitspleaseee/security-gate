//! The code to ban, unban and check if ip is banned.

use crate::auth::api::{authenticated, Token};
use crate::JsonResult;
use datatypes::admin::requests::AdminRequest;
use datatypes::admin::responses::AdminSuccess;
use datatypes::error::ResponseError;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rocket::State;
use rocket::{Data, Request};
use rocket_contrib::Json;
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::RwLock;

// Define blacklist:
#[derive(Default)]
pub struct BanIpAddrs {
    banned_ips: Arc<RwLock<HashSet<IpAddr>>>,
}

// Be sure blacklist is added "globally" (on_attach) and is checked on every response (on_request).
impl Fairing for BanIpAddrs {
    fn info(&self) -> Info {
        Info {
            name: "ban ip-addresses",
            kind: Kind::Attach | Kind::Request,
        }
    }

    // Add blacklist "globally" availible.
    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let banned_ips_clone = self.banned_ips.clone();
        Ok(rocket.manage(banned_ips_clone))
    }

    // Check client ip against blacklist.
    fn on_request(&self, req: &mut Request, _: &Data) {
        let addr = match req.remote() {
            Some(addr) => addr,
            // Ban any user where we cannot see their IP-address
            None => {
                info!("user without a ip-address tried to access the service");
                req.set_uri("/banned");
                return;
            }
        };

        let banned_ips = match self.banned_ips.read() {
            Ok(banned_ips) => banned_ips,
            Err(e) => {
                error!(
                    "internal error occured when trying to read
                       'banned_ips': {}",
                    e
                );
                return;
            }
        };

        if banned_ips.contains(&addr.ip()) {
            info!(
                "[{}] {} {}: IP banned, sent to /banned",
                addr,
                req.method(),
                req.uri()
            );
            req.set_uri("/banned");         // If banned, redirect to banned-page.
        }
    }
}

/// Give banned message
#[get("/banned")]
fn banned_message() -> &'static str {
    "You are banned from this site."
}


/// Ban or unban users.
///
/// If you are admin, you can ban or unban users.
/// Types you can send in: 'BAN', 'UNBAN'.
/// Types I can get back: 'IPBANNED', 'IPUNBANNED'.
/// 
/// # Example
///
/// Send this json to 'api/admin' (need to first log in as admin).
/// 
///´´´json
///{
///  "type": "BAN"
///  "payload": {
///      "ip": 195.168.1.2
///  }
///}
/// ´´´
/// 
/// Result:
///
///´´´json
///{
///  "type": "IPBANNED",
///  "payload": {
///      "ip": 195.168.1.2
///  }
///}
/// ´´´
#[post("/admin", format = "application/json", data = "<req>")]
pub fn post_admin(
    token: Token,
    req: Json<AdminRequest>,
    banned_ips: State<Arc<RwLock<HashSet<IpAddr>>>>,
) -> JsonResult<AdminSuccess> {
    authenticated(token).map_err(|_| Json(ResponseError::Unauthenticated))?;

    match *req {
        AdminRequest::BanIp(ref p) => {
            let mut banned_ips = banned_ips
                .write()
                .map_err(|_| Json(ResponseError::InternalServerError))?;

            // TODO should we tell the user about this indifference?
            // true  => IpAddr is now banned
            // false => IpAddr is already banned
            if banned_ips.insert(*p.ip()) {
                info!("banned ip {}", p.ip());
            } else {
                info!("tried to ban already banned ip {}", p.ip());
            }
            Ok(AdminSuccess::IpBanned)
        }
        AdminRequest::UnbanIp(ref p) => {
            let mut banned_ips = banned_ips
                .write()
                .map_err(|_| Json(ResponseError::InternalServerError))?;

            // TODO should we tell the user about this indifference?
            // true  => IpAddr is now unbanned
            // false => IpAddr is already unbanned
            if banned_ips.remove(p.ip()) {
                info!("unbanned ip {}", p.ip());
            } else {
                info!("tried to unban already unbanned ip {}", p.ip());
            }
            Ok(AdminSuccess::IpUnbanned)
        }
    }.map(Json)
}
