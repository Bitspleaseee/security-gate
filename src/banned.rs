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

pub struct NotBanned;

// Define blacklist:
lazy_static! {
    static ref BLACKLIST: HashSet<IpAddr> = {
        let mut m = HashSet::new();
        m
    };
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
            let success = BLACKLIST.insert(*ip_address);                  // Insert ip.
            if success {                                          // Ip don't exist already.
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} blacklisted.", ip_address).as_str()}))
            } else {
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} is already blacklisted.", ip_address).as_str()}))
            }
        }
        BanRequest::Unban(BanUserPayload {
            ref ip,
        }) => {
            let success = BLACKLIST.remove(ip);                  // Insert ip.
            if success {                                          // Ip don't exist already.
                Ok(OkSuccess::Ok(OkMessage {ok: true, message: format!("Ip {} is removed from blacklist.", ip).as_str()}))
            } else {
                Ok(OkSuccess::Ok(OkMessage {ok: false, message: format!("Ip {} is not in blacklist.", ip).as_str()}))
            }
        }
    }.map(Json)
    .map_err(Json)
}

impl Fairing for NotBanned {
    fn info(&self) -> Info {
        Info {
            name: "A fairing which check that an ip is not banned",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, req: &mut Request, _data: &Data) {
        match req.remote() {
            Some(addr) => {
                if !BLACKLIST.contains(&addr.ip()) {
                    info!("[{}] {} {}: IP not blacklisted, accepts request", addr, req.method(), req.uri());
                } else {
                    info!("[{}] {} {}: IP blacklisted, sent to /blocked", addr, req.method(), req.uri());
                }
            },
            None => info!("[-.-.-.-] {} {}", req.method(), req.uri()),
        }
    }
}