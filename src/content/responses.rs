use super::data::Category;
use super::data::Thread;
use super::data::Comment;
use super::data::SearchResult;

#[derive(Serialize, Deserialize, Debug)]
pub enum SearchSuccess<'a> {
    Results(#[serde(borrow)]Vec<SearchResult<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CategorySuccess<'a> {
    MultipleCategories(#[serde(borrow)]Vec<Category<'a>>),
    SingleCategory(#[serde(borrow)]Category<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ThreadSuccess<'a> {
    MultipleThreads(#[serde(borrow)]Vec<Thread<'a>>),
    SingleThread(#[serde(borrow)]Thread<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommentSuccess<'a> {
    MultipleComments(#[serde(borrow)]Vec<Thread<'a>>),
    SingleThread(#[serde(borrow)]Thread<'a>),
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
    InvalidId
}