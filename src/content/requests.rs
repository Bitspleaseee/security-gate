use super::data::Category;
use super::data::Thread;
use super::data::Comment;
use super::data::CategoryId;
use super::data::ThreadId;
use super::data::CommentId;

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum CategoryRequest<'a> {
    Add(#[serde(rename = "payload")] AddPayload),
    Edit(#[serde(rename = "payload", borrow)] Category<'a>),
    Hide(#[serde(rename = "payload")] HideCategoryPayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ThreadRequest<'a> {
    Add(#[serde(rename = "payload")] AddPayload),
    Edit(#[serde(rename = "payload", borrow)] Thread<'a>),
    Hide(#[serde(rename = "payload")] HideThreadPayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum CommentRequest<'a> {
    Add(#[serde(rename = "payload")] AddPayload),
    Edit(#[serde(rename = "payload", borrow)] Comment<'a>),
    Hide(#[serde(rename = "payload")] HideCommentPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddPayload {
    #[serde(rename = "title")]
    pub raw_title: String,
    #[serde(rename = "description")]
    pub raw_description: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HideCategoryPayload {
    #[serde(rename = "id")]
    pub id: CategoryId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HideThreadPayload {
    #[serde(rename = "id")]
    pub id: ThreadId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HideCommentPayload {
    #[serde(rename = "id")]
    pub id: CommentId,
}
