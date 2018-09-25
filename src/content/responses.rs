use super::data::Category;
use super::data::Comment;
use super::data::OkMessage;
use super::data::SearchResult;
use super::data::Thread;
use super::data::User;

#[derive(Serialize, Deserialize, Debug)]
pub enum SearchSuccess<'a> {
    Results(#[serde(borrow)] Vec<SearchResult<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CategorySuccess<'a> {
    MultipleCategories(#[serde(borrow)] Vec<Category<'a>>),
    SingleCategory(#[serde(borrow)] Category<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThreadSuccess<'a> {
    MultipleThreads(#[serde(borrow)] Vec<Thread<'a>>),
    SingleThread(#[serde(borrow)] Thread<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommentSuccess<'a> {
    MultipleComments(#[serde(borrow)] Vec<Comment<'a>>),
    SingleComment(#[serde(borrow)] Comment<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserSuccess<'a> {
    SingleUser(#[serde(borrow)] User<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OkSuccess<'a> {
    Ok(#[serde(borrow)] OkMessage<'a>),
}

#[derive(Fail, Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum GetError {
    #[fail(display = "content is hidden")]
    Hidden,
    #[fail(display = "passed invalid id")]
    InvalidId,
    #[fail(display = "token missing from cookies")]
    MissingToken,
    #[fail(display = "token not correct")]
    TokenNotCorrect,
    #[fail(display = "query was wrong")]
    InvalidQuery,
}
