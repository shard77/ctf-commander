use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::{Error, Url};
use std::fmt;
use std::time::Duration;

pub mod platforms;

// TODO: querybuilder => POST, GET, ...
// should include deserializing
// TODO: add secrets manager (token, creds)

#[derive(Clone)]
pub enum AuthMethod {
    Token(String),
    SessionCookie(String),
}

impl fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Token(t) => write!(f, "{}", t),
            Self::SessionCookie(sc) => write!(f, "{}", sc),
        }
    }
}

pub struct Platform {
    client: Client,
    base_url: Url,
    auth_method: AuthMethod,
}

impl Platform {
    pub fn new(base_url: Url, auth_method: AuthMethod) -> Self {
        Platform {
            client: Client::new(),
            base_url: base_url,
            auth_method: auth_method,
        }
    }

    fn get_request(&self, endpoint: &str) -> RequestBuilder {
        let target = self.base_url.join(endpoint).unwrap(); // todo: handle error

        self.client.get(target).timeout(Duration::from_secs(5))
    }

    fn auth(&self, request: RequestBuilder) -> RequestBuilder {
        match self.auth_method.clone() {
            AuthMethod::Token(token) => {
                request.header("Authorization", format!("Bearer {}", token))
            }
            AuthMethod::SessionCookie(cookie) => {
                todo!()
            }
        }
    }

    pub fn get(&self, endpoint: &str) -> Result<Response, Error> {
        let request = self.auth(self.get_request(endpoint));

        request.send()
    }
}
