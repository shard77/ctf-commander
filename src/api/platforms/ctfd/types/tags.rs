use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: u32,
    pub challenge_id: u32,
    pub value: String,
}

pub struct TagList {
    pub tags: Vec<Tag>,
}
