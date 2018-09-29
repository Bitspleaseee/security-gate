use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use crate::auth::api::{authenticated, Token, USER_TOKEN_NAME};
use crate::JsonResult;
use super::content::responses::OkSuccess;
use rocket_contrib::Json;

pub struct NotBanned;

#[get("/banned")]
fn bannedMessage() -> &'static str {
    "You are banned from this site."
}

#[post("/admin", format = "application/json", data = "<req>")]
pub fn post_admin<'a>(
    token: Token,
    req: Json<BanRequest>,
) -> JsonResult<OkSuccess<'a>, AuthError> {
    let result = authenticated(token);
    if result.is_err() {
        Err(AuthError::TokenNotCorrect).map_err(Json)?;
    }

    match *req {
        CategoryRequest::Add(AddPayload {
            ref raw_title,
            ref raw_description,
        }) => {
            // [..] is used to turn &String into &str
            //let title = raw_title[..].try_into().map_err(Json)?;
            //let description = raw_description[..].try_into().map_err(Json)?;

            //new_category(title, description)
            Err(GetError::InvalidId)
        }
        //CategoryRequest::Edit(Category {
        //    ref id,
        //    ref title,
        //    ref description
        //}) => {
        //    //edit_category(title, description)
        //    Err(GetError::InvalidId)
        //}
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
            Some(addr) => info!("[{}] {} {}", addr, req.method(), req.uri()),
            None => info!("[-.-.-.-] {} {}", req.method(), req.uri()),
        }
    }
}