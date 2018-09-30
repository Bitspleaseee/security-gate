use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use crate::auth::api::{authenticated, Token};
use crate::JsonResult;
use super::content::responses::OkSuccess;
use super::content::data::OkMessage;
use rocket_contrib::Json;
use datatypes::auth::requests::{BanRequest, BanUserPayload};
use datatypes::content::responses::ContentRequestError;
use std::collections::HashSet;
use std::net::IpAddr;
use rocket::response::Redirect;
use rocket::State;
use rocket::Rocket;
use std::sync::RwLock;
use std::sync::Arc;

pub struct NotBanned;

// Define blacklist:
#[derive(Default)]
pub struct BanIpAddrs {
     banned_ips: Arc<RwLock<HashSet<IpAddr>>>
}

impl Fairing for BanIpAddrs {
    fn info(&self) -> Info {
        Info {
            name: "A fairing which check that an ip is not banned",
            kind: Kind::Attach | Kind::Request,
        }
    }

   fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let banned_ips_clone = self.banned_ips.clone();
        rocket.manage(banned_ips_clone);
        Ok(rocket)
   }

   fn on_request(&self, req: &mut Request, _data: &Data) {
        match req.remote() {
            Some(addr) => {
                let ips = self.banned_ips.read().unwrap();
                if ips.contains(&addr.ip()) {
                    info!("[{}] {} {}: IP not blacklisted, accepts request", addr, req.method(), req.uri());
                } else {
                    info!("[{}] {} {}: IP blacklisted, sent to /blocked", addr, req.method(), req.uri());
                    let redirect = Redirect::to("/blocked");
                }
            },
            None => info!("[-.-.-.-] {} {}", req.method(), req.uri()),
        }
    }
}

// Give banned message
#[get("/banned")]
fn bannedMessage() -> &'static str {
    "You are banned from this site."
}

#[post("/admin", format = "application/json", data = "<req>")]
pub fn post_admin<'a>(
    token: Token,
    req: Json<BanRequest>,
    banned_ips: State<Arc<RwLock<HashSet<IpAddr>>>>
) -> JsonResult<OkSuccess<'a>, ContentRequestError> {
    let result = authenticated(token);
    if result.is_err() {
        Err(ContentRequestError::InvalidToken).map_err(Json)?;
    }

    match *req {
        BanRequest::Ban(BanUserPayload {
            ref ip,
        }) => {
            let ip_address = ip;
            let ips = banned_ips.write().unwrap();
            let success = ips.insert(*ip_address);                // Insert ip.
            if success {                                          // Ip don't exist already.
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} blacklisted.", ip_address).as_str()}))
            } else {
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} is already blacklisted.", ip_address).as_str()}))
            }
        }
        BanRequest::Unban(BanUserPayload {
            ref ip,
        }) => {
            let ips = banned_ips.write().unwrap();
            let success = ips.remove(ip);                         // Insert ip.
            if success {                                          // Ip don't exist already.
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} is removed from blacklist.", ip).as_str()}))
            } else {
                Ok(OkSuccess::Ok(OkMessage {ok: false, message: format!("Ip {} is not in blacklist.", ip).as_str()}))
            }
        }
    }.map(Json)
    .map_err(Json)
}