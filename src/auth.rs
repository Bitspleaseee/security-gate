use rocket::http::Cookies;
use rocket_contrib::Json;

use std::convert::TryInto;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use tarpc::sync::client::{ClientExt, Options};

use datatypes::auth::requests::RegisterUserPayload;
use datatypes::auth::requests::{AuthRequest, SetUserRolePayload};
use datatypes::auth::responses::{AuthError, AuthSuccess, Role};
use datatypes::error::ResponseError;
use datatypes::valid::token::USER_TOKEN_NAME;

use crate::comms::auth::SyncClient as AuthClient;
use crate::content::connect_to_controller;
use crate::JsonResponseResult;

lazy_static! {
    static ref AUTH_IP: SocketAddr = match std::env::var("AUTH_ADDRESS") {
        Ok(value) => value
            .to_socket_addrs()
            .expect("Unable to perform AUTH_ADDRESS resolving")
            .next()
            .expect(&format!("Unable to resolve '{}'", value)),
        Err(_) => {
            warn!("AUTH_ADDRESS is not set, using '127.0.0.1:10001'");
            SocketAddr::from(([127, 0, 0, 1], 10001))
        }
    };
}

// Connect to authentication service
pub fn connect_to_auth() -> Result<AuthClient, ResponseError> {
    trace!("Connecting to '{}' to perform auth request", *AUTH_IP);
    AuthClient::connect(*AUTH_IP, Options::default()).map_err(|e| {
        error!("Unable to connect to authentication-service: {:?}", e);
        ResponseError::InternalServerError
    })
}

/// Authenticate or deauthenticate user
///
/// Uses a strict JSON format to conway actions.
///
/// # Request objects
///
/// ## Authentication request
///
/// A request to authenticate with the system. Returns a special session cookie
/// which will be used
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
pub fn auth(
    mut cookies: Cookies,
    req: Option<Json<AuthRequest>>,
) -> JsonResponseResult<AuthSuccess> {
    use datatypes::auth::requests::AuthRequest::*;

    let req = req
        .ok_or(AuthError::InvalidCredentials)
        .map_err(|e| Json(e.into()))?; // If invalid request query.

    match req.into_inner() {
        Authenticate(p) => {
            let username = p.username.clone();
            connect_to_auth()
                .map_err(Json)?
                .authenticate(p)
                .map(|token| {
                    info!("User '{}' authenticated successfully", &username);

                    cookies.add_private(token.into());

                    Json(AuthSuccess::Authenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        }
        Deauthenticate(_) => {
            let cookie = cookies
                .get_private(USER_TOKEN_NAME)
                .ok_or(Json(ResponseError::Unauthenticated))?;

            connect_to_auth()
                .map_err(Json)?
                .deauthenticate(cookie.clone().into())
                .map(|_| {
                    info!("User deauthenticated successfully");
                    cookies.remove_private(cookie);
                    Json(AuthSuccess::Deauthenticated)
                }).map_err(|e| {
                    error!("Unable to 'authenticate': {:?}", e);
                    Json(e.into())
                })
        }
        RegisterUser(p) => {
            let user = connect_to_auth().map_err(Json)?.register(p).map_err(|e| {
                error!("Auth: Unable to 'register': {:?}", e);
                Json(e.into())
            })?;
            debug!("Auth: user registerd successfully");

            let _user = connect_to_controller()
                .map_err(Json)?
                .add_user(user)
                .map_err(|e| {
                    error!("Controller: Unable to add user: {:?}", e);
                    Json(e.into())
                })?;

            debug!("Controller: Returning success from 'add_user' request");
            Ok(Json(AuthSuccess::UserRegistered))
        }
    }
}

pub fn create_admin() {
    let auth = connect_to_auth().expect("Unable to connect to the auth service");
    let controller = connect_to_controller().expect("Unable to connect to the controller");

    println!("Creating admin account");

    let username;
    let email;
    let password;

    println!("Username: ");
    loop {
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();

        match string.trim().to_string().try_into() {
            Ok(value) => {
                username = value;
                break;
            }
            Err(_) => println!("Invalid username"),
        };
    }

    println!("Email: ");
    loop {
        let mut string = String::new();
        io::stdin().read_line(&mut string).unwrap();

        match string.trim().to_string().try_into() {
            Ok(value) => {
                email = value;
                break;
            }
            Err(_) => println!("Invalid email"),
        };
    }

    loop {
        let string = rpassword::prompt_password_stdout("Password: ").unwrap();

        match string.trim().to_string().try_into() {
            Ok(value) => {
                password = value;
                break;
            }
            Err(_) => println!("Invalid password"),
        };
    }

    let p = RegisterUserPayload {
        username,
        email,
        password,
    };

    let user = auth
        .register(p)
        .expect("Failed to register user in the auth service");
    let user = controller
        .add_user(user)
        .expect("Failed to register user in the controller");

    let p = SetUserRolePayload {
        id: user.id,
        role: Role::Admin,
    };
    auth.set_user_role(p).expect("Failed to set user role");

    println!("Account created successfully\n{:#?}", user);
}
