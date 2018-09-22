
/*/// Retrieve the user's ID, if any.
#[post("/login", format = "application/x-www-form-urlencoded", data = "<input>")]
fn login(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = login(input);
        cookies.add_private(Cookie::new("user_token", result.token));
        let ret = OkResponse {
            ok: result.ok,
            message: ""
        };
        info!("{}: logged in as user: {}", remote_addr, input.username);
        Json(ret);
    } else {
        let ret = OkResponse {
            ok: false,
            message: ""
        }
        info!("{}: tried to log in as user: {}", remote_addr, input.username);
        JSON(ret);
  }
}

/// Register user.
#[post("/register", format = "application/x-www-form-urlencoded", data = "<input>")]
fn register(input: Form<LogIn>, remote_addr: SocketAddr) -> Json<OkResponse> {
    if let Ok(username) = input.username {
        // result = register(input);
        info!("{}: sent request to register new user with username: {}", remote_addr, input.username);
        Json(result);
    } else {
        let ret = OkResponse {
            ok: false,,
            message: ""
        }
        info!("{}: failed regestering new user with username: {}", remote_addr, input.username);
        JSON(ret);
  }
}

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
