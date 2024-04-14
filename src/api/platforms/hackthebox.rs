pub use self::types::machines::{MachineList, MachineProfile};
use crate::api::{AuthMethod, Platform};
use anyhow::Error;
use reqwest::Url;

mod types;

pub struct HackTheBox {
    platform: Platform,
}

impl HackTheBox {
    pub fn new(token: &str) -> HackTheBox {
        let base_url = Url::parse("https://labs.hackthebox.com/api/v4/").unwrap();
        let auth_method = AuthMethod::Token(token.to_string());

        HackTheBox {
            platform: Platform::new(base_url, auth_method),
        }
    }

    pub fn machine_profile(&self, machine: &str) -> Result<MachineProfile, anyhow::Error> {
        let endpoint = String::from("machine/profile/") + machine;
        self.platform.get(&endpoint, None)
    }

    pub fn machine_paginated(&self) -> Result<MachineList, anyhow::Error> {
        let endpoint = String::from("machine/paginated");
        self.platform.get(&endpoint, None)
    }
}
