use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Topic {
    pub id: u32,
    pub value: String,
}

pub struct TopicList {
    pub topics: Vec<Topic>,
}
