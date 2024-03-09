use reqwest::blocking::{Client, Response};
use reqwest::{Error, Url};
use std::fmt;

// TODO: querybuilder => POST, GET, ...
// should include deserializing
// TODO: add secrets manager (token, creds)

#[derive(Clone)]
pub enum Auth {
    Token(String),
    SessionCookie(String),
}

impl fmt::Display for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(t) => write!(f, "{}", t),
            Self::SessionCookie(sc) => write!(f, "{}", sc),
        }
    }
}

pub struct Platform {
    url: reqwest::Url,
    auth: Auth,
    client: Client,
}

impl Platform {
    pub fn new(url: Url, auth: Auth) -> Self {
        Self {
            url,
            auth,
            client: Client::new(),
        }
    }

    pub fn get(&mut self, endpoint: String) -> Result<Response, Error> {
        let target = self.url.join(endpoint.as_str()).unwrap(); // todo: handle error

        self.client
            .get(target)
            .bearer_auth(self.auth.clone())
            .send()
    }
}
