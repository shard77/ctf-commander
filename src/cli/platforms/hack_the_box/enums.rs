use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum HackTheBox {
    Auth,
    GetMachineProfile,
}
