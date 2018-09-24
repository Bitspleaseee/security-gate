use super::data::Category;

pub enum CategorySuccess<'a> {
    MultipleCategories(Vec<Category<'a>>),
    SingleCategory(Category<'a>),
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