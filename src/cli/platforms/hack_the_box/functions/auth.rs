use crate::cli::utils;

pub fn authenticate() {
    if let Ok(input) = utils::input::input_credentials() {
        match utils::secrets::store_secret("ctf-commander", "hackthebox", &input) {
            Ok(_) => println!("Auth token has been stored successfully."),
            Err(e) => println!("Err: {e}"),
        };
    }
}
