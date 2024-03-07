use std::fmt;

pub mod providers;

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
