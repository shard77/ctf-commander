use clap::{error::ErrorKind, Command, CommandFactory, Parser, Subcommand};

use super::enums::HackTheBox;
use crate::cli::platforms::hack_the_box::functions;

pub fn handle(cmd: HackTheBox) {
    match cmd {
        HackTheBox::Auth => {
            println!("Please input your HTB auth token");
            functions::auth::authenticate();
        }
        HackTheBox::GetMachineProfile => {
            functions::machine::get_machine_profile();
        }
    }
}
