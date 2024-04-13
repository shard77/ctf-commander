use crate::api::{AuthMethod, Platform};
use reqwest::Url;

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
}
