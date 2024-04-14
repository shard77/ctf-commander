use crate::api::{AuthMethod, Platform};
use anyhow::Error;
use reqwest::Url;

pub use self::types::{awards, challenges, data, tags, topics};

mod types;

pub struct CTFd {
    platform: Platform,
}

// TODO: once all types are implemented, change for example `challenge_flags`
// to return `Result<data::Data<flags::Flag>, Error>` instead of
// `Result<data::Data<String>, Error>`.

impl CTFd {
    pub fn new(base_url: &str, token: &str) -> CTFd {
        let base_url = Url::parse(base_url).unwrap();
        let auth_method = AuthMethod::Token(token.to_string());

        CTFd {
            platform: Platform::new(base_url, auth_method),
        }
    }

    pub fn challenge(
        &self,
        challenge_id: u32,
    ) -> Result<data::Data<challenges::Challenge>, anyhow::Error> {
        let endpoint = String::from("challenges/") + challenge_id.to_string();
        self.platform.get(&endpoint, None)
    }

    pub fn challenge_list(
        &self,
        name: Option<&str>,
        max_attempts: Option<u32>,
        value: Option<u32>,
        category: Option<&str>,
        type_: Option<&str>,
        state: Option<&str>,
        q: Option<&str>,
    ) -> Result<data::Data<challenges::ChallengeList>, anyhow::Error> {
        let mut endpoint = String::from("challenges/");
        let mut query = Vec::new();

        macro_rules! add_query_param {
            ($param:expr, $value:expr) => {
                if let Some(value) = $value {
                    query.push(($param, value));
                }
            };
        }

        add_query_param!("name", name);
        add_query_param!("max_attempts", max_attempts);
        add_query_param!("value", value);
        add_query_param!("category", category);
        add_query_param!("type", type_);
        add_query_param!("state", state);
        add_query_param!("q", q);

        self.platform.get(&endpoint, Some(&query))
    }

    fn challenge_generic(
        &self,
        endpoint: &str,
        challenge_id: u32,
    ) -> Result<data::Data<String>, anyhow::Error> {
        let endpoint = String::from("challenges/") + endpoint.to_string();
        self.platform.get(
            &endpoint,
            Some(&vec![("challenge_id", challenge_id.to_string())]),
        )
    }

    pub fn challenge_types(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("types", challenge_id)
    }

    pub fn challenge_files(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("files", challenge_id)
    }

    pub fn challenge_flags(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("flags", challenge_id)
    }

    pub fn challenge_hints(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("hints", challenge_id)
    }

    pub fn challenge_requirements(
        &self,
        challenge_id: u32,
    ) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("requirements", challenge_id)
    }

    pub fn challenge_solves(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("solves", challenge_id)
    }

    pub fn challenge_tags(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("tags", challenge_id)
    }

    pub fn challenge_topics(&self, challenge_id: u32) -> Result<data::Data<String>, anyhow::Error> {
        self.challenge_generic("topics", challenge_id)
    }

    pub fn tag(&self, tag_id: u32) -> Result<data::Data<tags::Tag>, anyhow::Error> {
        let endpoint = String::from("tags/") + tag_id.to_string();
        self.platform.get(&endpoint, None)
    }

    pub fn tag_list(
        &self,
        challenge_id: Option<u32>,
        value: Option<&str>,
        q: Option<&str>,
    ) -> Result<data::Data<tags::TagList>, anyhow::Error> {
        let mut endpoint = String::from("tags");
        let mut query = Vec::new();

        macro_rules! add_query_param {
            ($param:expr, $value:expr) => {
                if let Some(value) = $value {
                    query.push(($param, value));
                }
            };
        }

        add_query_param!("challenge_id", challenge_id);
        add_query_param!("value", value);
        add_query_param!("q", q);

        self.platform.get(&endpoint, Some(&query))
    }

    pub fn topic(&self, topic_id: u32) -> Result<data::Data<topics::Topic>, anyhow::Error> {
        let endpoint = String::from("topics/") + topic_id.to_string();
        self.platform.get(&endpoint, None)
    }

    pub fn topic_list(
        &self,
        value: Option<u32>,
        q: Option<u32>,
    ) -> Result<data::Data<topics::TopicList>, anyhow::Error> {
        let endpoint = String::from("topics");
        let mut query = Vec::new();

        macro_rules! add_query_param {
            ($param:expr, $value:expr) => {
                if let Some(value) = $value {
                    query.push(($param, value));
                }
            };
        }

        add_query_param!("value", value);
        add_query_param!("q", q);

        self.platform.get(&endpoint, Some(&query))
    }

    pub fn award(&self, award_id: u32) -> Result<data::Data<awards::Award>, anyhow::Error> {
        let endpoint = String::from("awards/") + award_id.to_string();
        self.platform.get(&endpoint, None)
    }

    pub fn award_list(
        &self,
        user_id: Option<u32>,
        team_id: Option<u32>,
        type_: Option<&str>,
        value: Option<u32>,
        category: Option<&str>,
        icon: Option<&str>,
        q: Option<&str>,
    ) -> Result<data::Data<awards::AwardList>, anyhow::Error> {
        let mut endpoint = String::from("awards");
        let mut query = Vec::new();

        macro_rules! add_query_param {
            ($param:expr, $value:expr) => {
                if let Some(value) = $value {
                    query.push(($param, value));
                }
            };
        }

        add_query_param!("user_id", user_id);
        add_query_param!("team_id", team_id);
        add_query_param!("type", type_);
        add_query_param!("value", value);
        add_query_param!("category", category);
        add_query_param!("icon", icon);
        add_query_param!("q", q);

        self.platform.get(&endpoint, Some(&query))
    }
}
