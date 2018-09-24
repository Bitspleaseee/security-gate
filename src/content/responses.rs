use super::data::Category;
use super::data::Thread;
use super::data::Comment;
use super::data::SearchResult;

pub enum SearchSuccess<'a> {
    Results(Vec<SearchResult<'a>>),
}

pub enum CategorySuccess<'a> {
    MultipleCategories(Vec<Category<'a>>),
    SingleCategory(Category<'a>),
}

pub enum ThreadSuccess<'a> {
    MultipleThreads(Vec<Thread<'a>>),
    SingleThread(Thread<'a>),
}

pub enum CommentSuccess<'a> {
    MultipleComments(Vec<Thread<'a>>),
    SingleThread(Thread<'a>),
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