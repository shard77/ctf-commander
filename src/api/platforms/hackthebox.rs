use crate::api::{AuthMethod, Platform};
use reqwest::Url;

use self::types::machines::MachineProfile;

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

    pub fn get_machine_profile(&self, machine: &str) -> Result<MachineProfile, reqwest::Error> {
        let endpoint = String::from("machine/profile/") + machine;
        self.platform.get(&endpoint)
    }
}
