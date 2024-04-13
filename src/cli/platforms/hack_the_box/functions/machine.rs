use super::super::printers::machine::print_machine_profile;
use crate::{
    api::platforms::hackthebox::HackTheBox,
    cli::{platforms::hack_the_box::printers::machine::print_machine_list, utils},
};

pub fn get_machine_profile(machine: String) {
    let token = utils::secrets::retrieve_secret("ctf-commander", "hackthebox").unwrap();
    let htb = HackTheBox::new(&token);

    let res = htb.machine_profile(&machine).unwrap();
    print_machine_profile(res);
}

pub fn list_free_machines() {
    let token = utils::secrets::retrieve_secret("ctf-commander", "hackthebox").unwrap();
    let htb = HackTheBox::new(&token);

    let res = htb.machine_paginated().unwrap();
    print_machine_list(res)
}
