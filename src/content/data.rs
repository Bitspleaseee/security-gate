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

/// Marker trait to simplify implementations of actions on any id-type
trait Id {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CategoryId(u32);
impl_from_str!(CategoryId, u32);
impl Id for CategoryId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct ThreadId(u32);
impl_from_str!(ThreadId, u32);
impl Id for ThreadId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct CommentId(u32);
impl_from_str!(CommentId, u32);
impl Id for CommentId {}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct UserId(u32);
impl_from_str!(UserId, u32);
impl Id for UserId {}



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
        param.as_ref().try_into().map_err(|_| GetError::InvalidId)
    }
}


macro_rules! impl_from_str {
    ($id:ty, $from_ty:ty) => {
        impl<'a> FromStr<'a> for $id {
            type Err = <$id as FromStr>::Err;
            fn from_str(s: &'a str) -> Result<Self, Self::Err> {
                s.parse::<$from_ty>().map($id)
            }
        }
    };
}
