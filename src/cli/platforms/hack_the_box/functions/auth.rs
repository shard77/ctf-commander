use crate::cli::utils;

pub fn authenticate() {
    if let Ok(input) = utils::input::input_credentials() {
        let res = match utils::secrets::store_secret("ctf-commander", "hackthebox", &input) {
            Ok(sec) => println!("sec is fine"),
            Err(e) => println!("Err: {e}"),
        };

        let test = match utils::secrets::retrieve_secret("ctf-commander", "hackthebox") {
            Ok(resu) => println!("resu {:#?}", resu),
            Err(e) => println!("Err: {e}"),
        };

        println!("{:#?}", test);
    }
}
