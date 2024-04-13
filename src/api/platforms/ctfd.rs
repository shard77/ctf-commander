use crate::api::{AuthMethod, Platform};
use reqwest::Url;

pub use self::types::{challenges, data};

mod types;

pub struct CTFd {
    platform: Platform,
}

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
    ) -> Result<data::Data<challenges::Challenge>, reqwest::Error> {
        let endpoint = String::from("challenges") + challenge_id.to_string();
        self.platform.get(&endpoint)
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
    ) -> Result<data::Data<challenges::ChallengeList>, reqwest::Error> {
        let mut endpoint = String::from("challenges");
        let mut query = Vec::new();

        if let Some(name) = name {
            query.push(format!("name={}", name));
        }

        if let Some(max_attempts) = max_attempts {
            query.push(format!("max_attempts={}", max_attempts));
        }

        if let Some(value) = value {
            query.push(format!("value={}", value));
        }

        if let Some(category) = category {
            query.push(format!("category={}", category));
        }

        if let Some(type_) = type_ {
            query.push(format!("type={}", type_));
        }

        if let Some(state) = state {
            query.push(format!("state={}", state));
        }

        if let Some(q) = q {
            query.push(format!("q={}", q));
        }

        if !query.is_empty() {
            endpoint.push_str("?");
            endpoint.push_str(&query.join("&"));
        }

        self.platform.get(&endpoint)
    }

    pub fn challenge_types(&self, challenge_id: u32) -> Result<data::Data<String>, reqwest::Error> {
        let endpoint = String::from("challenges/types?challenge_id=") + challenge_id.to_string();
        self.platform.get(&endpoint)
    }

    pub fn challenge_files(&self, challenge_id: u32) -> Result<data::Data<String>, reqwest::Error> {
        let endpoint = String::from("challenges/files?challenge_id=") + challenge_id.to_string();
        self.platform.get(&endpoint)
    }

    pub fn challenge_flags(&self, challenge_id: u32) -> Result<data::Data<String>, reqwest::Error> {
        let endpoint = String::from("challenges/flags?challenge_id=") + challenge_id.to_string();
        self.platform.get(&endpoint)
    }
}
