use crate::auth::api::{authenticate, deauthenticate, USER_TOKEN_NAME};
use crate::auth::requests::{AuthPayload, AuthRequest};
use crate::auth::responses::{AuthError, AuthSuccess};
use crate::JsonResult;
use rocket::http::Cookies;
use rocket_contrib::Json;
use std::convert::TryInto;

/// Authenticate or deauthenticate user
///
/// Uses a strict JSON format to conway actions.
///
/// # Request objects
///
/// ## Authentication request
///
/// A request to authenticate with the system. Returns a special session cookie which will be used
/// for authorization.
///
/// ```json
/// {
///     "type": "AUTHENTICATE",
///     "payload": {
///         "username": "my_username",
///         "password": "secret"
///     }
/// }
/// ```
///
/// ## Deauthentication request
///
/// A request to deauthenticate with the service. Will remove the session cookie.
///
/// ```json
/// {
///     "type": "DEAUTHENTICATE",
///     "payload": {}
/// }
/// ```
///
/// # Response objects
///
/// Returns a similar shaped JSON object which containes the outcome of the request.
///
/// ```json
/// {
///     "type": "AUTHENTICATED",
/// }
/// ```
///
/// The possible types are defined in [`AuthError`](../responses/enum.AuthError.html)
#[post("/auth", format = "application/json", data = "<req>")]
pub fn auth(mut cookies: Cookies, req: Json<AuthRequest>) -> JsonResult<AuthSuccess, AuthError> {
    use super::requests::AuthRequest::{Authenticate, Deauthenticate};

    match *req {
        Authenticate(AuthPayload {
            ref raw_username,
            ref raw_password,
        }) => {
            // [..] is used to turn &String into &str
            let username = raw_username[..].try_into().map_err(Json)?;
            let password = raw_password[..].try_into().map_err(Json)?;

            authenticate(&username, &password).map(|token| {
                cookies.add_private(token.into());
                info!("user '{}' authenticated successfully", username);
                AuthSuccess::Authenticated
            })
        }
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(AuthError::MissingToken)
                .map_err(Json)?;

            deauthenticate(&cookie).map(|_| {
                info!("user deauthenticated successfully");
                cookies.remove_private(cookie);
                AuthSuccess::Deauthenticated
            })
        }
    }.map(Json)
    .map_err(Json)
}


/*/// Register user.
#[post("/register", format = "application/json", data = "<req>")]
fn register(req: RegisterPayload) -> JsonResult<AuthSuccess, AuthError> { {
    if let Ok(username) = req.username {
        // result = auth.register(req);
        trace!("sent request to register new user with username: {}", req.username);
        Err(AuthError::InvalidUsername).map(Json).map_err(Json)
    } else {
        trace!("failed regestering new user with username: {}", req.username);
        Err(AuthError::InvalidUsername).map(Json).map_err(Json)
  }
}*/

/*
/// Retrieve the user's profile.
#[get("/user/<username>")]
fn showUserProfile(username: Username, remote_addr: SocketAddr) -> String {
    //result = controller.getUserProfile(username);
    info!("{}: sent request to controller about public profile of user with username: {}", remote_addr, input.username);
    JSON(result);
}

// Retrive other user-information like email and role
#[get("/user/<username>/private")]
fn showUserProfile(cookies: Cookies, username: Username, remote_addr: SocketAddr) -> String {
    //result = auth.getUserProfile(cookies.get_private("user_token"), username);
    info!("{}: sent request to controller about private profile of user with username: {}", remote_addr, input.username);
    JSON(result);
}

/// Remove the `user_token` cookie and tell the authorization module to delete the token.
#[post("/logout")]
fn logout(mut cookies: Cookies, remote_addr: SocketAddr) -> Redirect {
    //auth.logout(cookies.get_private("user_token"))
    cookies.remove_private(Cookie::named("user_token"));
    info!("{}: user logged out", remote_addr, );
    // Redirect user to homepage on logout
    Redirect::to("/")
}

/// Edit user
#[post("/user/edit", format = "application/x-www-form-urlencoded", data = "<input>")]
fn editUser(cookies: Cookies, input: Form<User>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret1 = auth.editUser(input, result);
        //let ret2 = controller.editUser(input, result);
        let ret = OkResponse {
            ok: false,
            message: "Wrong input"
        }
        if ret1.ok == true && ret2.ok == true {           // Check that both functions worked correctly
            ret.ok = true;
            ret.message = "";
        }
        info!("{}: sent request to edit user to controller. editing will be done by user {} on user {}", remote_addr, result.username, input.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Upload a new avatar.
#[post("/user/avatar/upload", format = "multipart/form-data", data = "<input>")]
fn uploadAvatar(cookies: Cookies, input: Form<Avatar>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true {                                      // If token is correct.
        //let ret = controller.uploadAvatar(input, result);
        info!("{}: sent request to upload avatar for user with id {} by user ", remote_addr, input.uid, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
}

/// Administer usergroups.
#[post("/admin/edit/usergroups", format = "application/x-www-form-urlencoded", data = "<input>")]
fn adminUsergroups(cookies: Cookies, input: Form<AdminUsergroups>, remote_addr: SocketAddr) -> JSON<OkResponse> {
    //result = auth.verifyUser(cookies.get_private("user_token"));
    info!("{}: sent request to verify user to auth-module", remote_addr);
    if result.ok == true && result.role == 3 {                     // If token is correct and user is admin.
        //let ret = auth.administerUsergroups(input, result);
        info!("{}: sent request to auth-module to administer usergroups for user {}", remote_addr, result.username);
        JSON(ret);
    }
    else {
        let ret = OkResponse {
            ok: false,
            message: "Wrong input."
        }
        info!("{}: verify-request rejected", remote_addr);
        JSON(ret);
    }
*/
