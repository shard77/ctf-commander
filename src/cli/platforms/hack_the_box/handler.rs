use clap::{error::ErrorKind, Command, CommandFactory, Parser, Subcommand};

use super::enums::HackTheBox;
use super::functions::*;

use crate::cli::platforms::hack_the_box::functions;
use crate::cli::utils;

pub fn handle(cmd: HackTheBox) {
    match cmd {
        HackTheBox::Auth => {
            println!("Please input your HTB auth token");
            functions::auth::authenticate();
        }
    }
}
