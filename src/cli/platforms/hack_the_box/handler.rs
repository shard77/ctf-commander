use clap::{error::ErrorKind, Command, CommandFactory, Parser, Subcommand};

use super::enums::HackTheBox;

pub fn handle(cmd: HackTheBox) {
    match cmd {
        HackTheBox::Auth { method } => {
            println!("method you've chosen: {}", method);
        }
    }
}
