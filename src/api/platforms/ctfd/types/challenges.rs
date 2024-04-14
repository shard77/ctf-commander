use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub connection_info: String,
    pub next_id: u32,
    pub max_attempts: u32,
    pub value: u32,
    pub category: String,
    pub type_: String,
    pub state: String,
    pub requirements: Vec<String>,
    pub solves: u32,
    pub solved_by_me: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChallengeList {
    pub challenges: Vec<Challenge>,
}
