use reqwest::{ Url, Error };
use reqwest::blocking::{ Client, Response };

use crate::api::Auth;

pub struct Provider {
    url: reqwest::Url,
    auth: Auth,
    client: Client,
}

impl Provider {
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
