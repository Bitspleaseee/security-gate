use std::ops::Deref;
use std::convert::TryFrom;
use std::str::FromStr;
use std::convert::TryInto;
use rocket::request::FromParam;
use rocket::http::RawStr;
use super::responses::GetError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment<'a> {
    content: &'a str,
    thread: CommentId // TODO make own ID type
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thread<'a> {
    id: ThreadId, // TODO make own ID type
    category_id: u32, // TODO make own ID type
    title: &'a str,
    description: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category<'a> {
    id: CategoryId, // TODO make own ID type
    title: &'a str,
    description: &'a str
}

// TODO: Make this function correctly
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult<'a> {
    id: CategoryId, // TODO make own ID type
    title: &'a str,
    description: &'a str
}

/// Marker trait to simplify implementations of actions on any id-type
trait Id {}

macro_rules! impl_from_str {
    ($ty:ty, $exp:expr, $from_ty:ty) => {
        impl FromStr for $ty {
            type Err = <$from_ty as FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<$from_ty>().map($exp)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CategoryId(u32);
impl_from_str!(CategoryId, CategoryId, u32);
impl Id for CategoryId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ThreadId(u32);
impl_from_str!(ThreadId, ThreadId, u32);
impl Id for ThreadId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CommentId(u32);
impl_from_str!(CommentId, CommentId, u32);
impl Id for CommentId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct UserId(u32);
impl_from_str!(UserId, UserId, u32);
impl Id for UserId {}

impl<'a, I: Id + FromStr> FromParam<'a> for I {
    type Error = GetError;
    fn from_param(param: &'a RawStr) ->Result<Self, Self::Error> {
        let s: &'a str = param.as_ref();
        s.parse().map_err(|_| GetError::InvalidId)
    }
}


/// Optional wrapper for any type which implements Id
pub struct OptId<I: Id>(Option<I>);

impl<'v, I: Id + FromStr> TryFrom<&'v str> for OptId<I> {
    type Error = <I as FromStr>::Err;

    fn try_from(s: &'v str) -> Result<Self, Self::Error> {

        if s.is_empty() {
            return Ok(OptId(None)); // string = ""
        }

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
