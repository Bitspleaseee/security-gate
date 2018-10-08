//! The code to ban, unban and check if ip is banned.

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rocket::State;
use rocket::{Data, Request};
use rocket_contrib::Json;
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::RwLock;

use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;

use datatypes::admin::requests::AdminRequest;
use datatypes::admin::responses::AdminSuccess;
use datatypes::auth::responses::*;
use datatypes::content::responses::*;
use datatypes::error::ResponseError;
use datatypes::valid::token::Token;

use crate::auth::connect_to_auth;
use crate::JsonResponseResult;

const REQUEST_LIMIT: u32 = 40;

pub struct Count {
    pub count: u32,
    pub time: DateTime<Utc>,
}

impl Count {
    pub fn new(time: DateTime<Utc>) -> Count {
        Count { count: 0, time }
    }
    pub fn count(&mut self, time: DateTime<Utc>) -> u32 {
        if time == self.time {
            self.count = self.count + 1;
        } else {
            self.time = time;
            self.count = 1;
        }
        self.count
    }
}

// Define blacklist:
#[derive(Default)]
pub struct BanIpAddrs {
    banned_ips: Arc<RwLock<HashSet<IpAddr>>>,
    pub map: Arc<RwLock<HashMap<IpAddr, Count>>>,
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
        {
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
                req.set_uri("/banned"); // If banned, redirect to banned-page.
                return;
            }
        }

        // Request couter

        let mut map = match self.map.write() {
            Ok(map) => map,
            Err(e) => {
                error!("Error reading map : {}", e);
                return;
            }
        };

        let mut time_now: DateTime<Utc> = Utc::now();
        trace!("time: {}", time_now);

        let sec = time_now.time().second();
        let nano = time_now.time().nanosecond();

        let floor_sec = (sec / 10) * 10;
        let offset = sec - floor_sec;

        time_now = time_now - Duration::seconds(offset.into());
        time_now = time_now - Duration::nanoseconds(nano.into());

        trace!("time: {}", time_now);

        let ip = addr.ip();

        match map.get_mut(&ip) {
            Some(counter) => {
                let c = counter.count(time_now);
                info!("count: {}", c);

                if c > REQUEST_LIMIT {
                    match self.banned_ips.write() {
                        Ok(mut banned_ips) => {
                            banned_ips.insert(ip);
                            ()
                        }
                        Err(e) => error!("Error writing to banned_ips: {}", e),
                    };
                    req.set_uri("/banned");
                }
            }
            None => {
                map.insert(ip, Count::new(time_now));
                ()
            }
        };
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
    req: Option<Json<AdminRequest>>,
    banned_ips: State<Arc<RwLock<HashSet<IpAddr>>>>,
) -> JsonResponseResult<AdminSuccess> {
    let req = req
        .ok_or(ContentError::InvalidContent)
        .map_err(|e| Json(e.into()))?; // If invalid request give error.

    // Check what role the user has (and that a user is valid):
    let role = connect_to_auth()
        .map_err(Json)?
        .get_user_role(token)
        .map_err(|e| Json(e.into()))?;

    // Only admins can do something here (return with error if not admin)
    if role < Role::Admin {
        Err(ResponseError::Unauthorized).map_err(|e| Json(e))?;
    }

    use datatypes::admin::requests::AdminRequest::*;
    match req.into_inner() {
        BanIp(p) => {
            // Use a separate scope to perform insertion
            //
            // This is to minimize the amount of time we store the
            // 'RwLockWriteGuard' to prevent blocking other requests from
            // reading from 'banned_ips'
            let res = {
                banned_ips
                    .write()
                    .map_err(|_| Json(ResponseError::InternalServerError))?
                    .insert(p.ip)
            };

            // true  => IpAddr is now banned
            // false => IpAddr is already banned
            if res {
                info!("banned ip {}", p.ip);
            } else {
                info!("tried to ban already banned ip {}", p.ip);
            }
            Ok(AdminSuccess::IpBanned)
        }
        UnbanIp(p) => {
            // Use a separate scope to perform removal
            let res = {
                banned_ips
                    .write()
                    .map_err(|_| Json(ResponseError::InternalServerError))?
                    .remove(&p.ip)
            };

            // true  => IpAddr is now unbanned
            // false => IpAddr is already unbanned
            if res {
                info!("unbanned ip {}", p.ip);
            } else {
                info!("tried to unban already unbanned ip {}", p.ip);
            }
            Ok(AdminSuccess::IpUnbanned)
        }
    }.map(Json)
}
