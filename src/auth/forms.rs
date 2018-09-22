
#[derive_FromForm]
struct LogIn<'v> {
    username: Result<Username<'v>, &'v RawStr>,
    password: Result<Password<'v>, &'v RawStr>
}

#[derive_FromForm]
struct Avatar {
    avatar: String,
    uid: u32
}


