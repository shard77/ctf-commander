use crate::{api::platforms::hackthebox::HackTheBox, cli::utils};

pub fn get_machine_profile() {
    let token = utils::secrets::retrieve_secret("ctf-commander", "hackthebox").unwrap();
    let htb = HackTheBox::new(&token);

    let res = htb.get_machine_profile("RouterSpace");
    println!("{:#?}", res);
}
