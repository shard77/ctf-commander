use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Award {
    pub id: u32,
    pub user_id: u32,
    pub team_id: u32,
    pub type_: String,
    pub tname: String,
    pub description: String,
    pub date: String,
    pub value: u32,
    pub category: String,
    pub icon: String,
    pub requirements: Vec<String>,
}

pub struct AwardList {
    pub awards: Vec<Award>,
}
