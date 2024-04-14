use anyhow::Error;
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::fmt;
use std::time::Duration;

pub mod platforms;

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

macro_rules! add_query_param {
    ($params:expr, $name:expr, $value:expr) => {
        if let Some(value) = $value {
            $params.push(($name.to_string(), value.to_string()));
        }
    };
}
pub(crate) use add_query_param;

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

    fn get_request(
        &self,
        endpoint: &str,
        params: Option<&Vec<(String, String)>>,
    ) -> Result<RequestBuilder, anyhow::Error> {
        let target = self.base_url.join(endpoint)?;

        let target = if let Some(params) = params {
            Url::parse_with_params(&target.to_string(), params)?
        } else {
            target
        };

        Ok(self.client.get(target).timeout(Duration::from_secs(5)))
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

    pub fn get<D>(
        &self,
        endpoint: &str,
        params: Option<&Vec<(String, String)>>,
    ) -> Result<D, anyhow::Error>
    where
        D: DeserializeOwned,
    {
        let request_builder = self.get_request(endpoint, params)?;
        let request = self.auth(request_builder);

        let response = request.send()?.json::<D>()?;

        Ok(response)
    }
}
