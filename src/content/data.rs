use std::ops::Deref;
use std::convert::TryFrom;
use std::str::FromStr;
use std::convert::TryInto;
use std::fmt::{self, Display};
use rocket::request::FromParam;
// TODO uncomment this when its needed for a valid implementation for QueryStr
//use rocket::request::FromFormValue;
use rocket::http::RawStr;
use super::responses::GetError;
use crate::auth::requests::Username;

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment<'a> {
    id: CommentId,
    content: &'a str,
    thread: ThreadId,
    uid: UserId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thread<'a> {
    id: ThreadId,
    category_id: u32,
    title: &'a str,
    description: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category<'a> {
    id: CategoryId,
    title: &'a str,
    description: &'a str
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User<'a> {
    id: UserId,
    username: Username<'a>,
    description: &'a str,
    avatar: &'a str
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OkMessage<'a> {
    ok: bool,
    message: &'a str
}

// TODO: Make this function correctly
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult<'a> {
    id: CategoryId,
    title: &'a str,
    description: &'a str
}

// TODO uncomment when a valid implementation for `QueryStr` exists
//#[derive_FromForm]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchQuery<'a> {
    q: QueryStr<'a>
}

impl Display for SearchQuery<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.q)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueryStr<'a>(&'a str);

impl Display for QueryStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<'v> FromFormValue<'v> for QueryStr<'v> {
//     type Error = &'v RawStr;
//
//     fn from_form_value(search: &'v RawStr) -> Result<QueryStr, &'v RawStr> {
//         // TODO a raw string is always a correct str, hence you can use `search.as_ref()` which will
//         // return a `&str`. Parse is used to convert between different values, e.g from str to
//         // number. See [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
//         match search.parse::<&str>() {
//             // TODO stronger vertification (only ascii chars and limited length?). You can see how
//             // this was done for `Username` and `Password` in the `auth` module.
//             Ok(search) if search != "" => Ok(QueryStr(search)),
//             _ => Err(search),
//         }
//     }
// }

/// Marker trait to simplify implementations of actions on any id-type
pub trait Id {}

macro_rules! id_impls {
    ($ty:ty, $exp:expr, $from_ty:ty) => {
        impl FromStr for $ty {
            type Err = <$from_ty as FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$from_ty>().map($exp)
            }
        }

        impl<'a> FromParam<'a> for $ty {
            type Error = GetError;
            fn from_param(param: &'a RawStr) ->Result<Self, Self::Error> {
                let s: &'a str = param.as_ref();
                s.parse().map_err(|_| GetError::InvalidId)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CategoryId(u32);
id_impls!(CategoryId, CategoryId, u32);
impl Id for CategoryId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ThreadId(u32);
id_impls!(ThreadId, ThreadId, u32);
impl Id for ThreadId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CommentId(u32);
id_impls!(CommentId, CommentId, u32);
impl Id for CommentId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct UserId(u32);
id_impls!(UserId, UserId, u32);
impl Id for UserId {}

/// Optional wrapper for any type which implements Id
///
/// The reason for this wrapper is to be able to implement a custom `TryFrom` and `FromParam` for
/// `OptId` which takes into account if the value is empty.
pub struct OptId<I: Id>(Option<I>);

impl<'v, I: Id + FromStr> TryFrom<&'v str> for OptId<I> {
    type Error = <I as FromStr>::Err;

    fn try_from(s: &'v str) -> Result<Self, Self::Error> {
        if s.is_empty() { return Ok(OptId(None)); }
        s.parse().map(|id| OptId(Some(id)))
    }
}

impl<I: Id> Deref for OptId<I> {
    type Target = Option<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, I: Id + FromStr> FromParam<'a> for OptId<I> {
    type Error = GetError;
    fn from_param(param: &'a RawStr) ->Result<Self, Self::Error> {
        let s: &'a str = param.as_ref();
        s.try_into().map_err(|_| GetError::InvalidId)
    }
}
